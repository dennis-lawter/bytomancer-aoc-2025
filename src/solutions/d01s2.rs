use super::solutions::final_answer;
use super::solutions::input_raw;
use regex::Regex;

const DAY: u8 = 01;
const SOL: u8 = 2;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;

    let mut dial: i32 = 50;
    let mut zeroes = 0;
    let regex = Regex::new("([LR])(\\d+)").expect("Invalid regex");
    for line in &input {
        if let Some(caps) = regex.captures(line) {
            let dir = &caps[1];
            let digit_str = &caps[2];
            let val: i32 = digit_str.parse().expect("Invalid digits");
            zeroes += val / 100; // just burn these excess turns now into zeroes
            let val = val % 100; // then worry only about the ending
            let old_dial = dial;
            match dir {
                "L" => dial -= val,
                "R" => dial += val,
                d => panic!("Invalid dir: {}", d),
            };

            if dial == 0 || dial == 100 || dial == -100 {
                zeroes += 1;
            } else if dial > 100 || dial <= -100 {
                zeroes += 1;
            } else if old_dial > 0 && dial < 0 || old_dial < 0 && dial > 0 {
                zeroes += 1;
            }

            dial %= 100;
        }
    }

    final_answer(zeroes, submit, DAY, SOL).await;
}

