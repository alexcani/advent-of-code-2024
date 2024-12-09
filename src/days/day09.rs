use advent_of_code_2024::*;

use itertools::Itertools;

pub fn solve(context: &mut Context) {
    let input = &context.input()[0];

    // if Some(u8) then u8 is the block id
    // if None then it's free space
    let mut blocks: Vec<Option<u32>> = Vec::new();
    let mut next_id = 0u32;
    input.chars().chunks(2).into_iter().for_each(|mut chunk| {
        let block_size = chunk.next().unwrap().to_digit(10).unwrap() as u8;
        let free_space = chunk.next().unwrap_or('0').to_digit(10).unwrap() as u8;

        let mut block = vec![Some(next_id); block_size as usize];
        let mut free = vec![None; free_space as usize];
        blocks.append(&mut block);
        blocks.append(&mut free);
        next_id += 1;
    });
    let len = blocks.len();

    let mut blocks2 = blocks.clone(); // for part 2

    let mut current_block = len;
    let mut current_free = 0;
    while let (Some(block), Some(free)) = (
        find_next_block(&blocks, current_block),
        find_next_free_space(&blocks, len, current_free),
    ) {
        if free > block {
            break;
        }
        blocks.swap(block, free);
        current_block = block;
        current_free = free + 1;
    }

    context.set_sol1(calculate_checksum(&blocks));

    let (mut start, mut end) = find_next_block_chunk(&blocks2, len);
    while start > 0 {
        let size = end - start + 1;
        if let Some(idx_free) = find_free_chunk_of_size(&blocks2, len, size) {
            if idx_free < start {
                // to the left
                (start..=end)
                    .zip(idx_free..)
                    .for_each(|(i, j)| blocks2.swap(i, j));
            }
        }
        (start, end) = find_next_block_chunk(&blocks2, start);
    }

    context.set_sol2(calculate_checksum(&blocks2));
}

fn find_next_free_space(blocks: &[Option<u32>], len: usize, start: usize) -> Option<usize> {
    (start..len).find(|i| blocks[*i].is_none())
}

fn find_next_block(blocks: &[Option<u32>], start: usize) -> Option<usize> {
    // look backwards
    (0..start).rev().find(|i| blocks[*i].is_some())
}

fn calculate_checksum(blocks: &[Option<u32>]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .map(|(i, x)| (x.unwrap_or_default() * i as u32) as u64)
        .sum()
}

fn find_next_block_chunk(blocks: &[Option<u32>], start: usize) -> (usize, usize) {
    let end_idx = (0..start).rev().find(|i| blocks[*i].is_some()).unwrap();
    let block_id = blocks[end_idx].unwrap();
    let mut start_idx = end_idx;
    while start_idx > 0 && blocks[start_idx - 1] == Some(block_id) {
        start_idx -= 1;
    }

    (start_idx, end_idx)
}

fn find_free_chunk_of_size(blocks: &[Option<u32>], len: usize, size: usize) -> Option<usize> {
    (0..len).find(|i| blocks[*i..].iter().take(size).all(|x| x.is_none()))
}

#[allow(dead_code)]
fn print_blocks(blocks: &Vec<Option<u32>>) {
    for block in blocks {
        match block {
            Some(id) => print!("{}", id),
            None => print!("."),
        }
    }
    println!();
}
