use super::solutions::final_answer;
use super::solutions::input_raw;
use std::collections::BTreeMap;

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
    // pub fn contains(&self, l: &Point3D, r: &Point3D) -> bool {
    //     // println!("contains call {l:?} {r:?}\n{:?}", self.db);
    //     let ans = self.db.iter().any(|i| &(i.0) == l && &(i.1) == r);
    //     // println!("{ans:?}");
    //     ans
    // }
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    // println!("{input:#?}");

    let iter_count = if example { 10usize } else { 1_000usize };

    let mut conns = ConnDb::new();

    let mut memo: BTreeMap<(usize, usize), u64> = BTreeMap::new();

    for (l, lp) in input.iter().enumerate() {
        for (r, rp) in input.iter().enumerate() {
            if l <= r {
                continue;
            }
            let dist_sq = lp.get_dist_sq(rp);
            memo.insert((l, r), dist_sq);
        }
    }

    let mut memo_sorted: Vec<(usize, usize, u64)> =
        memo.iter().map(|(k, v)| (k.0, k.1, *v)).collect();
    memo_sorted.sort_by(|l, r| l.2.cmp(&r.2));

    for (l, r, _val) in &memo_sorted {
        let lp = &input[*l];
        let rp = &input[*r];
        conns.register(lp, rp);
        if conns.db.len() >= iter_count {
            break;
        }
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
    for conn in conns.db.iter() {
        cdb.combine(&conn.0, &conn.1);
    }

    println!("{cdb:#?}");
    let mut sizes = cdb.sizes();
    sizes.sort();
    sizes.reverse();
    println!("{:?}", sizes);

    let ans = sizes[0] * sizes[1] * sizes[2];

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
    fn pop_entry_with_pt(&mut self, pt: &Point3D) -> Vec<Point3D> {
        for i in 0..self.db.len() {
            if self.db[i].contains(pt) {
                return self.db.remove(i);
            }
        }
        panic!("Can't find {pt:?}!");
    }
    pub fn combine(&mut self, lp: &Point3D, rp: &Point3D) {
        let mut lc = self.pop_entry_with_pt(lp);
        if lc.contains(rp) {
            println!("WARN: lc contains rp");
            self.db.push(lc);
        } else {
            let mut rc = self.pop_entry_with_pt(rp);
            lc.append(&mut rc);
            self.db.push(lc);
        }
    }
    pub fn can_combine_more(&self) -> bool {
        self.db.len() != 1
    }
    pub fn sizes(&self) -> Vec<usize> {
        let mut ans = vec![];
        for circ in &self.db {
            ans.push(circ.len());
        }
        ans
    }
}
