use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 04;
const SOL: u8 = 1;

pub async fn input(example: bool) -> Vec<Vec<char>> {
    let raw = input_raw(DAY, example).await;
    let lines: Vec<String> = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();
    let mut grid: Vec<Vec<char>> = vec![];
    for line in &lines {
        let grid_line: Vec<char> = line.chars().collect();
        grid.push(grid_line);
    }
    grid
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let mut ans = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let adj = adj_at_count(&input, y, x);
            if adj < 4 && input[y][x] == '@' {
                ans += 1;
                print! {"X"};
            } else {
                print!("{}", input[y][x]);
            }
        }
        println!("");
    }
    final_answer(ans, submit, DAY, SOL).await;
}

pub fn adj_at_count(input: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let mut ans = 0;
    let w = input[y].len();
    let h = input.len();
    if (y as usize) > 0 {
        if (x as usize) > 0 {
            if input[y as usize - 1][x as usize - 1] == '@' {
                ans += 1;
            }
        }
        if input[y as usize - 1][x as usize] == '@' {
            ans += 1;
        }
        if (x as usize) < w - 1 {
            if input[y as usize - 1][x as usize + 1] == '@' {
                ans += 1;
            }
        }
    }
    if (x as usize) > 0 {
        if input[y as usize][x as usize - 1] == '@' {
            ans += 1;
        }
    }
    if (x as usize) < w - 1 {
        if input[y as usize][x as usize + 1] == '@' {
            ans += 1;
        }
    }
    if (y as usize) < h - 1 {
        if (x as usize) > 0 {
            if input[y as usize + 1][x as usize - 1] == '@' {
                ans += 1;
            }
        }
        if input[y as usize + 1][x as usize] == '@' {
            ans += 1;
        }
        if (x as usize) < w - 1 {
            if input[y as usize + 1][x as usize + 1] == '@' {
                ans += 1;
            }
        }
    }

    ans
}
