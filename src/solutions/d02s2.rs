use super::solutions::final_answer;
use super::d02s1::*;
//use std::collections::HashSet;

const DAY: u8 = 02;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let mut invalid_ids: Vec<u64> = vec!();
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
    let ans:u64 = invalid_ids.into_iter().sum();
    final_answer(ans, submit, DAY, SOL).await;
}

fn num_is_repeat(input: u64) -> bool {
    let num = format!("{input}");
    
    for pattern_size in 1..=num.len() / 2 {
        if test_pattern_size(&num, pattern_size) {
            return true;
        }
    }
    
    false
}

fn test_pattern_size(input: &str, plen: usize) -> bool {
    let num_reps_req = input.len() / plen;
    let (start, rest) = input.split_at(plen);
    let repeated_start = start.to_owned().repeat(num_reps_req - 1);

    repeated_start == rest
}
