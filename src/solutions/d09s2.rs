use super::d09s1::*;
use super::solutions::final_answer;

const DAY: u8 = 09;
const SOL: u8 = 2;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum TileState {
    Corner,
    Edge,
    Inner,
    Outer,
    Untested,
}
impl TileState {
    pub fn to_char(&self) -> char {
        match self {
            TileState::Corner => '#',
            TileState::Edge => 'X',
            TileState::Inner => 'x',
            TileState::Outer => 'o',
            TileState::Untested => '.',
        }
    }
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;

    // well, I know this is awful but it seems to work..?
    // glad I have 62 GiB of RAM...
    let mut xs = vec![];
    let mut ys = vec![];
    for p in &input {
        xs.push(p.x);
        ys.push(p.y);
    }
    println!(
        "XRange: {} - {}",
        xs.iter().min().unwrap(),
        xs.iter().max().unwrap()
    );
    println!(
        "YRange: {} - {}",
        ys.iter().min().unwrap(),
        ys.iter().max().unwrap()
    );
    let xmax = *xs.iter().max().unwrap() as usize + 1;
    let ymax = *ys.iter().max().unwrap() as usize + 1;
    //let bounds = (xs.iter().max().unwrap() * ys.iter().max().unwrap()) as usize;
    let mut grid: Vec<Vec<TileState>> = Vec::with_capacity(ymax);
    // let grid_row = vec![TileState::Untested;xmax];
    for _ in 0..ymax {
        // grid.push(grid_row.clone());
        grid.push(vec![TileState::Untested; xmax]);
    }

    for p in &input {
        grid[p.y as usize][p.x as usize] = TileState::Corner;
    }

    for (i, p) in input.iter().enumerate() {
        let prev_i = if i == 0 { input.len() - 1 } else { i - 1 };
        let prev_p = &input[prev_i];
        // fill in the edges
        if prev_p.x == p.x {
            let x = p.x as usize;
            let range_max = *[prev_p.y, p.y].iter().max().unwrap() as usize;
            let range_min = *[prev_p.y, p.y].iter().min().unwrap() as usize;
            for y in (range_min + 1)..range_max {
                grid[y][x] = TileState::Edge;
            }
        } else if prev_p.y == p.y {
            let y = p.y as usize;
            let range_max = *[prev_p.x, p.x].iter().max().unwrap() as usize;
            let range_min = *[prev_p.x, p.x].iter().min().unwrap() as usize;
            for x in (range_min + 1)..range_max {
                grid[y][x] = TileState::Edge;
            }
        } else {
            panic!("prev_p and p don't share an edge!");
        }
    }

    // Terrible alternative to flood fill algo...
    for y in 0..ymax {
        let mut following_edge = false;
        let mut inside = false;
        for x in 0..xmax {
            if following_edge {
                if grid[y][x] == TileState::Corner {
                    following_edge = false;
                    if x + 1 < xmax {
                        inside = grid[y - 1][x + 1] == TileState::Inner;
                    }
                }
            } else {
                if grid[y][x] == TileState::Corner {
                    following_edge = true;
                } else if grid[y][x] == TileState::Edge {
                    inside = !inside;
                } else if grid[y][x] == TileState::Untested {
                    grid[y][x] = if inside {
                        TileState::Inner
                    } else {
                        TileState::Outer
                    };
                }
            }
        }
    }

    if example {
        for y in 0..ymax {
            for x in 0..xmax {
                print!("{}", grid[y][x].to_char());
            }
            println!("");
        }
    }

    // And now the worst solution ever...
    let mut largest_area = 0;
    for (i, p1) in input.iter().enumerate() {
        println!("{}%", (i as f64 / input.len() as f64) * 100.0);
        for p2 in &input {
            if p1 == p2 {
                continue;
            }
            let area = p1.get_area_with(p2);
            if area > largest_area {
                if is_valid_area(p1, p2, &grid) {
                    largest_area = area;
                    println!("new largest found:");
                    println!("area: {area}");
                    println!("{p1:?}\t{p2:?}");
                }
            }
        }
    }

    final_answer(largest_area, submit, DAY, SOL).await;
}

pub fn is_valid_area(p1: &Point2D, p2: &Point2D, grid: &Vec<Vec<TileState>>) -> bool {
    let xs = [p1.x, p2.x];
    let ys = [p1.y, p2.y];
    let xmin = *xs.iter().min().unwrap();
    let xmax = *xs.iter().max().unwrap();
    let ymin = *ys.iter().min().unwrap();
    let ymax = *ys.iter().max().unwrap();
    for y in ymin..ymax {
        for x in xmin..xmax {
            if grid[y as usize][x as usize] == TileState::Outer {
                return false;
            }
        }
    }

    true
}
