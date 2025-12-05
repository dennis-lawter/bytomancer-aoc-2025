use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 05;
const SOL: u8 = 1;

pub async fn input(example: bool) -> (Vec<(u64, u64)>, Vec<u64>) {
    let raw = input_raw(DAY, example).await;
    let (part1, part2) = raw.split_once("\n\n").expect("no dbl ln");
    let p1 = part1
        .lines()
        .filter(|item| item.len() > 0)
        .map(|item| {
            let (a, b) = item.split_once("-").expect("no -");
            let (a, b): (u64, u64) = (a.parse().expect("a invalid"), b.parse().expect("b invalid"));
            (a, b)
        })
        .collect();

    let p2 = part2
        .lines()
        .filter(|item| item.len() > 0)
        .map(|item| item.parse::<u64>().expect("Invalid ingredient"))
        .collect();

    (p1, p2)
}

pub async fn solve(submit: bool, example: bool) {
    let (ranges, ingredients) = input(example).await;
    let mut ans: usize = 0;
    for ingredient in ingredients {
        if ing_is_fresh(ingredient, &ranges) {
            ans += 1;
        }
    }

    final_answer(ans, submit, DAY, SOL).await;
}

pub fn ing_is_fresh(ingredient: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for range in ranges {
        let (l, r) = range;
        if ingredient >= *l && ingredient <= *r {
            return true;
        }
    }
    false
}
