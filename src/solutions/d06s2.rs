use super::solutions::final_answer;
use super::solutions::input_raw;
use std::collections::VecDeque;

const DAY: u8 = 06;
const SOL: u8 = 2;

pub async fn input(example: bool) -> (Vec<Vec<u64>>, Vec<char>) {
    let raw = input_raw(DAY, example).await;
    let mut lines_raw: VecDeque<String> = raw
        .lines()
        .filter(|i| i.len() > 0)
        .map(|i| i.to_owned())
        .collect();
    let last_line = lines_raw.pop_back().unwrap().replace(" ", "");
    let ops = last_line.chars().rev().collect();

    let mut nums: Vec<Vec<u64>> = vec![];
    let grid: Vec<Vec<char>> = lines_raw
        .into_iter()
        .map(|l| -> Vec<char> { l.chars().collect() })
        .collect();

    let mut n_group: Vec<u64> = vec![];
    for x in (0..grid[0].len()).rev() {
        let mut num = 0;

        for y in (0..grid.len()) {
            if grid[y][x] != ' ' {
                num *= 10;
                num += grid[y][x].to_digit(10).expect("Invalid char...") as u64;
            }
        }
        println!("{num}");
        if num == 0 {
            nums.push(n_group);
            n_group = vec![];
        } else {
            n_group.push(num);
        }
    }
    nums.push(n_group);

    (nums, ops)
}

pub async fn solve(submit: bool, example: bool) {
    let (nums, ops) = input(example).await;
    println!("{nums:#?}\n\n\n");
    let mut col_ans: Vec<u64> = vec![];
    for (i, group) in nums.iter().enumerate() {
        let op = ops[i];
        let col = match op {
            '*' => group.iter().product(),
            '+' => group.iter().sum(),
            _ => panic!("Invalid op"),
        };
        for (j, n) in group.iter().enumerate() {
            if j != 0 {
                print!("{op}")
            }
            print!("{n}");
        }
        println!(" = {col}");
        col_ans.push(col);
    }
    // println!("{col_ans:#?}");
    let ans: u64 = col_ans.iter().sum();

    final_answer(ans, submit, DAY, SOL).await;
}
