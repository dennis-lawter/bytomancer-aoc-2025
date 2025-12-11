use super::d11s1::*;
use super::solutions::final_answer;

const DAY: u8 = 11;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    final_answer(input[0].to_owned(), submit, DAY, SOL).await;
}
