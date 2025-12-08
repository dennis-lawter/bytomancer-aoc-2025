use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 08;
const SOL: u8 = 1;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point3D {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}
impl Point3D {
    pub fn from_str(input: &str) -> Self {
        let pieces: Vec<String> = input.split(",").map(|i| i.to_owned()).collect();
        let nums: Vec<u64> = pieces
            .into_iter()
            .map(|i| -> u64 { i.parse::<u64>().expect("Invalid num") })
            .collect();
        Self {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        }
    }
    pub fn get_dist_sq(&self, other: &Point3D) -> u64 {
        let xs = [self.x, other.x];
        let ys = [self.y, other.y];
        let zs = [self.z, other.z];
        let xd = xs.iter().max().unwrap() - xs.iter().min().unwrap();
        let yd = ys.iter().max().unwrap() - ys.iter().min().unwrap();
        let zd = zs.iter().max().unwrap() - zs.iter().min().unwrap();
        xd * xd + yd * yd + zd * zd
    }
}

pub async fn input(example: bool) -> Vec<Point3D> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .filter(|item| item.len() > 0)
        .map(|item| Point3D::from_str(item))
        .collect();

    lines
}

#[derive(Clone, Debug)]
pub struct ConnDb {
    pub db: Vec<(Point3D, Point3D)>,
}
impl ConnDb {
    pub fn new() -> Self {
        Self { db: vec![] }
    }
    pub fn register(&mut self, l: &Point3D, r: &Point3D) {
        let new_l = l.clone();
        let new_r = r.clone();
        self.db.push((new_l, new_r));
    }
    pub fn contains(&self, l: &Point3D, r: &Point3D) -> bool {
        // println!("contains call {l:?} {r:?}\n{:?}", self.db);
        let ans = self.db.iter().any(|i| &(i.0) == l && &(i.1) == r);
        // println!("{ans:?}");
        ans
    }
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let ans = 0;
    // println!("{input:#?}");

    let iter_count = if example { 10usize } else { 1_000usize };

    let mut conns = ConnDb::new();

    for _ in 0..iter_count {
        let mut shortest_dist_sq = u64::MAX;
        let mut shortest_index_l = 0usize;
        let mut shortest_index_r = 0usize;
        for (l, lp) in input.iter().enumerate() {
            for (r, rp) in input.iter().enumerate() {
                if l == r {
                    continue;
                }
                if l > r {
                    continue;
                }
                if conns.contains(lp, rp) {
                    continue;
                }
                let dist_sq = lp.get_dist_sq(rp);
                if dist_sq < shortest_dist_sq {
                    shortest_dist_sq = dist_sq;
                    shortest_index_l = l;
                    shortest_index_r = r;
                }
            }
        }
        conns.register(&input[shortest_index_l], &input[shortest_index_r]);
    }
    //println!("{conns:#?}");
    println!("\n\n\n");

    for conn in conns.db.iter() {
        println!(
            "{:?}\t{:?}\t{}",
            conn.0,
            conn.1,
            conn.0.get_dist_sq(&conn.1)
        );
    }

    let mut cdb = CircDb::new();
    for node in &input {
        cdb.new_circuit(node);
    }

    final_answer(ans, submit, DAY, SOL).await;
}

#[derive(Clone, Debug)]
pub struct CircDb {
    pub db: Vec<Vec<Point3D>>,
}
impl CircDb {
    pub fn new() -> Self {
        Self { db: vec![] }
    }
    pub fn new_circuit(&mut self, pt: &Point3D) {
        self.db.push(vec![pt.clone()]);
    }
}
