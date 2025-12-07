#![allow(warnings)]

use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 07;
const SOL: u8 = 1;

pub async fn input(example: bool) -> Vec<Vec<char>> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .filter(|item| item.len() > 0)
        .map(|item| item.to_owned().chars().collect())
        .collect();

    lines
}

pub async fn solve(submit: bool, example: bool) {
    let mut input = input(example).await;
    let mut splits = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let c = input[y][x];
            match c {
                'S' => {
                    input[y+1][x] = '|';
                }
                '^' => {
                    if input[y-1][x] == '|' {
                        input[y][x-1] = '|';
                        input[y][x+1] = '|';
                        splits += 1;
                    }
                }
                '.' => {
                    if y != 0 && input[y-1][x] == '|' {
                        input[y][x] = '|';
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
    final_answer(splits, submit, DAY, SOL).await;
}
