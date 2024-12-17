use advent_of_code_2024::*;

struct Computer {
    ip: usize,
    regs: [u64; 3],
    program: Vec<u8>,
}

impl Computer {
    fn run(&mut self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        while let Some(opcode) = self.program.get(self.ip) {
            let operand = *match self.program.get(self.ip + 1) {
                Some(operand) => operand,
                None => break,
            };
            self.ip += 2;

            match *opcode {
                0 => {
                    // adv
                    self.regs[0] /= 2u64.pow(self.get_combo_operand(operand) as u32);
                }
                1 => {
                    // bxl
                    self.regs[1] ^= operand as u64;
                }
                2 => {
                    // bst
                    self.regs[1] = self.get_combo_operand(operand) % 8;
                }
                3 => {
                    // jnz
                    if self.regs[0] != 0 {
                        self.ip = operand as usize;
                    }
                }
                4 => {
                    // bxc
                    self.regs[1] ^= self.regs[2];
                }
                5 => {
                    // out
                    output.push((self.get_combo_operand(operand) % 8) as u8);
                }
                6 => {
                    // bdv
                    self.regs[1] = self.regs[0] / 2u64.pow(self.get_combo_operand(operand) as u32);
                }
                7 => {
                    // cdv
                    self.regs[2] = self.regs[0] / 2u64.pow(self.get_combo_operand(operand) as u32);
                }
                _ => unreachable!(),
            }
        }
        output
    }

    fn get_combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.regs[0],
            5 => self.regs[1],
            6 => self.regs[2],
            _ => unreachable!(),
        }
    }
}

fn parse_register(input: &str) -> u64 {
    input.split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap()
}

pub fn solve(context: &mut Context) {
    let input = context.input();
    let a = parse_register(&input[0]);
    let b = parse_register(&input[1]);
    let c = parse_register(&input[2]);
    let program = input[4]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut computer = Computer {
        ip: 0,
        regs: [a, b, c],
        program: program.clone(),
    };
    let output = computer.run();
    let result = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    context.set_sol1(result);

    // Part 2 - DFS approach
    let mut states = vec![(0, 0)];  // state is (a, i), where i is index backwards
    let mut res = vec![];
    while let Some((a, i)) = states.pop() {
        if i == program.len() {
            res.push(a);
            continue;
        }
        for b in 0..8 {
            let new_a = (a << 3) | b;
            let target = program[program.len() - i - 1];
            let mut computer = Computer {
                ip: 0,
                regs: [new_a, 0, 0],
                program: program.clone(),
            };
            let output = computer.run();
            if output[0] == target {
                states.push((new_a, i + 1));
            }
        }
    }
    let result = *res.iter().min().unwrap();
    context.set_sol2(result);
}
