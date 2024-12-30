use advent_of_code_2024::*;
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::printer::{DotPrinter, PrinterContext};
use itertools::Itertools;
use std::collections::HashMap;
use std::io::Write;

#[derive(Clone)]
struct Wire {
    value: Option<bool>,
    gates: Vec<usize>,
    ready: bool,
}

impl Wire {
    fn run(&mut self, gates: &mut [Gate]) {
        if !self.ready || self.gates.is_empty() {
            return;
        }

        for gate_id in &self.gates {
            let gate = &mut gates[*gate_id];
            gate.ready_inputs += 1;
        }

        self.ready = false;
    }
}

#[derive(Clone)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Clone)]
struct Gate {
    gate_type: GateType,
    inputs: Vec<usize>,
    output: usize,
    ready_inputs: usize,
}

impl Gate {
    fn run(&mut self, wires: &mut [Wire]) {
        if self.ready_inputs < self.inputs.len() {
            return;
        }

        let input_values = self
            .inputs
            .iter()
            .map(|input_id| wires[*input_id].value.unwrap())
            .collect::<Vec<bool>>();
        let output_value = match self.gate_type {
            GateType::And => input_values[0] && input_values[1],
            GateType::Or => input_values[0] || input_values[1],
            GateType::Xor => input_values[0] ^ input_values[1],
        };

        wires[self.output].value = Some(output_value);
        wires[self.output].ready = true;
        self.ready_inputs = 0;
    }
}

struct WireNode {
    name: String,
    from: String,
    to: Vec<String>,
}

pub fn solve(context: &mut Context) {
    let mut wires_ids = HashMap::new();
    let mut wires = Vec::new();
    let mut gates = Vec::new();
    context.input().iter().for_each(|line| {
        if line.is_empty() {
            return;
        }

        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() == 2 {
            // initial wire
            let wire_name = parts[0][0..=2].to_owned();
            let wire_id = wires.len();
            wires_ids.insert(wire_name, wire_id);
            wires.push(Wire {
                value: Some(parts[1].parse::<u8>().unwrap() == 1),
                gates: Vec::new(),
                ready: true,
            });
        } else {
            let input_1 = get_wire_id_or_initialize(&mut wires_ids, &mut wires, parts[0]);
            let input_2 = get_wire_id_or_initialize(&mut wires_ids, &mut wires, parts[2]);
            let output = get_wire_id_or_initialize(&mut wires_ids, &mut wires, parts[4]);
            let gate_type = match parts[1] {
                "AND" => GateType::And,
                "OR" => GateType::Or,
                "XOR" => GateType::Xor,
                t => panic!("Unknown gate type {}", t),
            };

            let gate_id = gates.len();
            wires[input_1].gates.push(gate_id);
            wires[input_2].gates.push(gate_id);
            gates.push(Gate {
                gate_type,
                inputs: vec![input_1, input_2],
                output,
                ready_inputs: 0,
            });
        }
    });

    let outputs: Vec<usize> = wires_ids // id of the wires that are outputs (start with z), in reverse order
        .keys()
        .filter(|wire| wire.starts_with('z'))
        .sorted()
        .rev()
        .map(|wire| wires_ids[wire])
        .collect();

    let p1_wires = wires.clone();
    let p1_gates = gates.clone();
    let result = run_circuit(p1_wires, p1_gates, &outputs);
    context.set_sol1(result);

    // Part 2 - Generate graphviz graph of the circuit
    let mut n_or_gates = 0;
    let mut n_and_gates = 0;
    let mut n_xor_gates = 0;

    let mut graph = graph!(strict di id!("circuit"));

    let mut wires: HashMap<String, WireNode> = HashMap::new();

    context.input().iter().for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() < 5 {
            return;
        }

        let (gate_name, gate_color) = match parts[1] {
            "AND" => {
                n_and_gates += 1;
                (format!("AND{}", n_and_gates), "blue")
            }
            "OR" => {
                n_or_gates += 1;
                (format!("OR{}", n_or_gates), "green")
            }
            "XOR" => {
                n_xor_gates += 1;
                (format!("XOR{}", n_xor_gates), "red")
            }
            _ => panic!("Invalid gate type"),
        };

        // Add gate node to graph
        graph.add_stmt(stmt!(node!(gate_name; attr!("color",gate_color))));

        let input_1 = parts[0];
        wires
            .entry(input_1.to_string())
            .or_insert_with(|| WireNode {
                name: input_1.to_string(),
                from: "".to_string(),
                to: vec![],
            })
            .to
            .push(gate_name.clone());

        let input_2 = parts[2];
        wires
            .entry(input_2.to_string())
            .or_insert_with(|| WireNode {
                name: input_2.to_string(),
                from: "".to_string(),
                to: vec![],
            })
            .to
            .push(gate_name.clone());

        let output = parts[4];
        wires
            .entry(output.to_string())
            .or_insert_with(|| WireNode {
                name: output.to_string(),
                from: "".to_string(),
                to: vec![],
            })
            .from = gate_name.clone();
    });

    // Add subgraph for inputs
    let mut input_graph = subgraph!("inputs"; attr!("rank","same"));
    wires
        .iter()
        .filter(|(_, wire)| wire.from.is_empty())
        .for_each(|(_, wire)| {
            input_graph
                .stmts
                .push(stmt!(node!(wire.name; attr!("shape","box"))));
        });
    graph.add_stmt(stmt!(input_graph));

    // Add subgraph for outputs
    let mut output_graph = subgraph!("outputs"; attr!("rank","same"));
    wires
        .iter()
        .filter(|(_, wire)| wire.to.is_empty())
        .for_each(|(_, wire)| {
            output_graph
                .stmts
                .push(stmt!(node!(wire.name; attr!("shape","box"))));
        });
    graph.add_stmt(stmt!(output_graph));

    // Add edges
    wires.iter().for_each(|(_, wire)| {
        let from = if wire.from.is_empty() {
            &wire.name
        } else {
            &wire.from
        };
        let to_solo = [wire.name.clone()];
        let to = if wire.to.is_empty() {
            &to_solo[..]
        } else {
            &wire.to[..]
        };

        to.iter().for_each(|to| {
            graph.add_stmt(stmt!(
                edge!(node_id!(from) => node_id!(to); attr!("label",wire.name))
            ));
        });
    });

    let dot = graph.print(&mut PrinterContext::default());
    let mut file = std::fs::File::create("graph.dot").unwrap();
    file.write_all(dot.as_bytes()).unwrap();

    // Manual analysis of the graph to find solution
    // gwh,z09,wgb,wbw,rcb,z21,jct,z39
    let mut sol = ["gwh","z09","wgb","wbw","rcb","z21","jct","z39"];
    sol.sort();
    context.set_sol2(sol.join(","));
}

fn run_circuit(mut wires: Vec<Wire>, mut gates: Vec<Gate>, outputs: &[usize]) -> u64 {
    loop {
        for wire in wires.iter_mut() {
            wire.run(&mut gates);
        }

        for gate in gates.iter_mut() {
            gate.run(&mut wires);
        }

        if outputs.iter().all(|output_id| wires[*output_id].ready) {
            break;
        }
    }

    outputs
        .iter()
        .map(|output_id| wires[*output_id].value.unwrap() as u64)
        .fold(0, |acc, value| (acc << 1) + value)
}

fn get_wire_id_or_initialize(
    wires_ids: &mut HashMap<String, usize>,
    wires: &mut Vec<Wire>,
    wire_name: &str,
) -> usize {
    if let Some(wire_id) = wires_ids.get(wire_name) {
        *wire_id
    } else {
        let wire_id = wires.len();
        wires_ids.insert(wire_name.to_owned(), wire_id);
        wires.push(Wire {
            value: None,
            gates: Vec::new(),
            ready: false,
        });
        wire_id
    }
}
