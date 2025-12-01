use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 01;
const SOL: u8 = 1;

pub async fn input(example: bool) -> Vec<i32> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .filter(|item| item.len() > 0)
        .map(|item| {
            let item = item.replace("L", "-");
            let item = item.replace("R", "");
            item.parse::<i32>().expect("Invalid number")
        })
        .collect();

    lines
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;

    let mut dial: i32 = 50;
    let mut zeroes = 0;
    for line in &input {
        dial += line;
        dial %= 100;
        if dial == 0 {
            zeroes += 1;
        }
    }

    final_answer(zeroes, submit, DAY, SOL).await;
}
