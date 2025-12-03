use super::solutions::final_answer;
use super::d03s1::*;

const DAY: u8 = 03;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let mut ans = 0u64;
    for line in input {
        let digits: Vec<u8> = line.
            chars()
            .map(|item| item.to_string().parse::<u8>().expect("Invalid char"))
            .collect();
        
        let bank_jolts = get_highest(&digits, 0, 12);
        ans += bank_jolts as u64;
        println!("{line} : {bank_jolts}");
    }
    final_answer(ans, submit, DAY, SOL).await;
}

fn get_highest(data: &Vec<u8>, start: usize, ignore: usize) -> u64
{
    if ignore == 0 {
        return 0;
    }

    let mut highest_i = start;
    for i in start..(data.len() - (ignore-1)) {
        if data[i] > data[highest_i] {
            highest_i = i;
        }
    }
    let exp = 10u64.pow((ignore - 1) as u32);
    ((data[highest_i] as u64) * exp) + get_highest(data, highest_i + 1, ignore - 1)
}
