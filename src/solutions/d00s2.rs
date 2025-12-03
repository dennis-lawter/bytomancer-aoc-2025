use super::solutions::final_answer;
use super::d00s1::*;

const DAY: u8 = 00;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    final_answer(input[0].to_owned(), submit, DAY, SOL).await;
}
