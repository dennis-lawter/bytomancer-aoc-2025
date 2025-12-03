use super::d01s1::*;
use super::solutions::final_answer;

const DAY: u8 = 01;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;

    let mut dial: i32 = 50;
    let mut zeroes = 0;
    for line in &input {
        zeroes += line.abs() / 100;
        let line = line % 100;

        let old_dial = dial;
        dial += line;

        if dial == 0 || dial == 100 || dial == -100 {
            zeroes += 1;
        } else if dial > 100 || dial <= -100 {
            zeroes += 1;
        } else if old_dial > 0 && dial < 0 || old_dial < 0 && dial > 0 {
            zeroes += 1;
        }

        dial %= 100;
    }

    final_answer(zeroes, submit, DAY, SOL).await;
}
