use super::d04s1::*;
use super::solutions::final_answer;

const DAY: u8 = 04;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let mut input = input(example).await;
    let mut ans = 0;
    let mut last_ans = -1;
    while ans != last_ans {
        last_ans = ans;
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                let adj = adj_at_count(&input, y, x);
                if adj < 4 && input[y][x] == '@' {
                    ans += 1;
                    input[y][x] = '.';
                } else {
                }
            }
        }
    }
    final_answer(ans, submit, DAY, SOL).await;
}
