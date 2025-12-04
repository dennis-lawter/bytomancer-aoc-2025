use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 02;
const SOL: u8 = 1;

pub async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .replace("\n", "")
        .split(",")
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let mut invalid_ids: Vec<u64> = vec![];
    for line in input {
        let mut split = line.split("-");
        let l_str = split.nth(0).expect("No L");
        let r_str = split.nth(0).expect("No R");
        let l: u64 = l_str.parse().expect("bad L");
        let r: u64 = r_str.parse().expect("bad R");

        for i in l..=r {
            if num_is_repeat(i) {
                invalid_ids.push(i);
            }
        }
    }
    let ans: u64 = invalid_ids.into_iter().sum();
    final_answer(ans, submit, DAY, SOL).await;
}

fn num_is_repeat(input: u64) -> bool {
    let mut digits: Vec<u64> = vec![];
    let mut input = input;
    while input > 0 {
        digits.push(input % 10);
        input /= 10;
    }

    if digits.len() % 2 != 0 {
        return false;
    }

    let mut i = 0;
    let mut j = digits.len() / 2;
    let i_stop = j;

    while i < i_stop {
        if digits[i] != digits[j] {
            return false;
        }
        i += 1;
        j += 1;
    }

    true
}
