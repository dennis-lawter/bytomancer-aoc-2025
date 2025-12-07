use super::solutions::final_answer;
use super::solutions::input_raw;
use std::collections::VecDeque;

const DAY: u8 = 06;
const SOL: u8 = 1;

pub async fn input(example: bool) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut raw = input_raw(DAY, example).await;
    while raw.contains("  ") {
        raw = raw.replace("  ", " ");
    }
    let mut lines: VecDeque<String> = raw
        .lines()
        .filter(|item| item.len() > 0)
        .map(|item| item.to_owned())
        .collect();
    let last_line = lines.pop_back().unwrap();
    let last_line_smaller = last_line.replace(" ", "");

    let mut numbers: Vec<Vec<u64>> = vec![];
    while let Some(line) = lines.pop_front() {
        numbers.push(
            line.split(" ")
                .filter(|i| i.len() > 0)
                .map(|i| i.parse().expect(&format!("Invalid num: {i}")))
                .collect::<Vec<u64>>(),
        );
    }

    let ops = last_line_smaller.chars().collect();

    (numbers, ops)
}

pub async fn solve(submit: bool, example: bool) {
    let (nums, ops) = input(example).await;
    println!("{ops:#?}");
    let mut col_ans: Vec<u64> = vec![];
    for x in 0..nums[0].len() {
        let mut col = 0u64;
        for y in 0..nums.len() {
            match ops[x] {
                '+' => {
                    col += nums[y][x];
                }
                '*' => {
                    if y == 0 {
                        col = nums[y][x];
                    } else {
                        col *= nums[y][x];
                    }
                }
                _ => {
                    panic!("Invalid op: {}", ops[x]);
                }
            }
        }
        col_ans.push(col);
    }
    println!("{col_ans:#?}");
    let ans: u64 = col_ans.iter().sum();
    final_answer(ans, submit, DAY, SOL).await;
}
