use super::d08s1::*;
use super::solutions::final_answer;

const DAY: u8 = 08;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let _input = input(example).await;
    let ans = 0;
    final_answer(ans, submit, DAY, SOL).await;
}
