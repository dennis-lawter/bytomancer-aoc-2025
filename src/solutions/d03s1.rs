use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 03;
const SOL: u8 = 1;

pub async fn input(example: bool) -> Vec<String> {
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
    let mut ans = 0u64;
    for line in input {
        let digits: Vec<u8> = line
            .chars()
            .map(|item| item.to_string().parse::<u8>().expect("Invalid char"))
            .collect();
        let mut highest_i = 0;
        for i in 0..digits.len() - 1 {
            if &digits[i] > &digits[highest_i] {
                highest_i = i;
            }
        }
        let mut highest_j = highest_i + 1;
        for j in highest_j..digits.len() {
            if &digits[j] > &digits[highest_j] {
                highest_j = j;
            }
        }
        let bank_jolts = digits[highest_i] * 10 + digits[highest_j];
        ans += bank_jolts as u64;
        println!("{line} : {bank_jolts}");
    }
    final_answer(ans, submit, DAY, SOL).await;
}
