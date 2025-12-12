use super::d12s1::*;
use super::solutions::final_answer;

const DAY: u8 = 12;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    final_answer(0, submit, DAY, SOL).await;
}
