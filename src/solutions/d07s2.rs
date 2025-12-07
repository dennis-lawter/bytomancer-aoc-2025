use super::d07s1::*;
use super::solutions::final_answer;
use std::collections::VecDeque;

const DAY: u8 = 07;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let mut input = input(example).await;
    let mut stacks: VecDeque<VecDeque<usize>> = VecDeque::new();
    let w = input[0].len();
    let h = input.len();
    let mut line_stack: VecDeque<usize> = VecDeque::new();
    for _ in 0..w {
        line_stack.push_back(0);
    }
    for _ in 0..h {
        stacks.push_back(line_stack.clone());
    }
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let c = input[y][x];
            match c {
                'S' => {
                    input[y + 1][x] = '|';
                    stacks[y + 1][x] = 1;
                }
                '^' => {
                    if input[y - 1][x] == '|' {
                        input[y][x - 1] = '*';
                        input[y][x + 1] = '*';
                        stacks[y][x - 1] += stacks[y - 1][x];
                        stacks[y][x + 1] += stacks[y - 1][x];
                    }
                }
                '.' | '|' | '*' => {
                    if y != 0 && (input[y - 1][x] == '|' || input[y - 1][x] == '*') {
                        input[y][x] = '|';
                        stacks[y][x] += stacks[y - 1][x];
                    }
                }
                _ => {}
            }
        }
    }
    for line in &input {
        for c in line {
            print!("{c}");
        }
        println!("");
    }
    for line in &stacks {
        for v in line {
            print!("{v}\t");
        }
        println!("");
    }
    let final_row = stacks.pop_back().unwrap();
    let ans: usize = final_row.iter().sum();
    final_answer(ans, submit, DAY, SOL).await;
}
