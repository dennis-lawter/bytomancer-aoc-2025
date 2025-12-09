use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 09;
const SOL: u8 = 1;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Point2D {
    pub x: u64,
    pub y: u64,
}
impl Point2D {
    pub fn from_str(input: &str) -> Self {
        let (l,r) = input.split_once(",").expect("No ,");
        let ln = l.parse().expect("Invalid L");
        let rn = r.parse().expect("Invalid R");
        Self {
            x: ln,
            y: rn,
        }
    }
    pub fn get_area_with(&self, other: &Point2D) -> u64 {
        let x_min = [self.x,other.x].iter().min().unwrap().to_owned();
        let x_max = [self.x,other.x].iter().max().unwrap().to_owned();
        let y_min = [self.y,other.y].iter().min().unwrap().to_owned();
        let y_max = [self.y,other.y].iter().max().unwrap().to_owned();
        let x_diff = x_max - x_min + 1;
        let y_diff = y_max - y_min + 1;
        x_diff * y_diff
    }
}

pub async fn input(example: bool) -> Vec<Point2D> {
    let raw = input_raw(DAY, example).await;
    raw
        .lines()
        .filter(|item| item.len() > 0)
        .map(|item| Point2D::from_str(item))
        .collect()
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let mut largest_area = 0u64;
    for p1 in input.iter() {
        for p2 in input.iter() {
            let area = p1.get_area_with(p2);
            if area > largest_area {
                println!("new largest found:");
                println!("area: {area}");
                println!("{p1:?}\t{p2:?}");
                println!("");
                largest_area = area;
            }
        }
    }
    
    final_answer(largest_area, submit, DAY, SOL).await;
}
