use super::solutions::final_answer;
use super::solutions::input_raw;
use std::collections::VecDeque;
use std::collections::HashSet;

const DAY: u8 = 12;
const SOL: u8 = 1;

const ROT_CW: [usize; 9] = [6,3,0,7,4,1,8,5,2];
pub fn rot_cw(input: &Vec<char>) -> Vec<char> {
    let mut out = vec![' ';9];
    for (i, j) in ROT_CW.iter().enumerate() {
        out[*j] = input[i];
    }
    assert_eq!(false, out.contains(&' '));
    out
}

const FLIP_LR: [usize; 9] = [2,1,0,5,4,3,8,7,6];
pub fn flip_lr(input: &Vec<char>) -> Vec<char> {
    let mut out = vec![' ';9];
    for (i, j) in FLIP_LR.iter().enumerate() {
        out[*j] = input[i];
    }
    assert_eq!(false, out.contains(&' '));
    out
}

const FLIP_UD: [usize; 9] = [6,7,8,3,4,5,0,1,2];
pub fn flip_ud(input: &Vec<char>) -> Vec<char> {
    let mut out = vec![' ';9];
    for (i, j) in FLIP_LR.iter().enumerate() {
        out[*j] = input[i];
    }
    assert_eq!(false, out.contains(&' '));
    out
}

#[derive(Default, Debug)]
pub struct Pres {
    pub perms: Vec<u16>
}
impl Pres {
    pub fn new(input: &str) -> Self {
        // discard first line, just the idx
        let (_, remainder) = input.split_once("\n").expect("Can't split pres");
        let raw = remainder.replace("\n", "");
        let first_chars: Vec<char> = raw.replace("#", "1").replace(".", "0").chars().collect();
        
        let rot_once = rot_cw(&first_chars);
        let rot_twice = rot_cw(&rot_once);
        let rot_thrice = rot_cw(&rot_twice);

        let curr_perms = vec!(first_chars, rot_once, rot_twice, rot_thrice);
        let mut all_perms: HashSet<Vec<char>> = HashSet::new();
        for perm in curr_perms {
            let flr = flip_lr(&perm);
            let fud = flip_ud(&perm);
            let fboth = flip_ud(&flr);
            all_perms.insert(perm);
            all_perms.insert(flr);
            all_perms.insert(fud);
            all_perms.insert(fboth);
        }
        
        let perms = all_perms
            .iter()
            .map(|i| {
                let s: String = i.iter().collect();
                u16::from_str_radix(&s, 2).expect("Invalid binary: {s}")
            })
            .collect();

        // let data = u16::from_str_radix(&raw_binary, 2).expect("Invalid binary");
        Self {
            perms,
        }
    }
}

#[derive(Default, Debug)]
pub struct Reg {
    pub w: usize,
    pub h: usize,
    pub idx_counts: Vec<usize>,
}
impl Reg {
    pub fn new(input: &str) -> Self {
        let (dim_raw, list_raw) = input.split_once(": ").expect("Can't split reg");
        let (w_raw, h_raw) = dim_raw.split_once("x").expect("dim has no x");
        let w = w_raw.parse().expect("Invalid w");
        let h = h_raw.parse().expect("Invalid h");

        let idx_counts = list_raw
            .split(" ")
            .map(|i| i.parse().expect("Invalid idx"))
            .collect();

        Self {
            w,
            h,
            idx_counts,
        }
    }
}

#[derive(Default, Debug)]
pub struct Prob {
    pub presents: Vec<Pres>,
    pub regions: Vec<Reg>,
}

pub async fn input(example: bool) -> Prob {
    let raw = input_raw(DAY, example).await;
    let mut groups: VecDeque<String> = raw
        .split("\n\n")
        .map(|i| i.to_string())
        .collect();
    let spaces = groups.pop_back().unwrap();
    
    let mut prob = Prob::default();

    while let Some(group) = groups.pop_front() {
        let pres = Pres::new(&group);
        prob.presents.push(pres);
    };

    for line in spaces.lines() {
        let reg = Reg::new(line);
        prob.regions.push(reg);
    }

    prob
}

pub async fn solve(submit: bool, example: bool) {
    let prob = input(example).await;
    println!("{prob:#?}");
    let mut ans = 0;
    for (i, reg) in prob.regions.iter().enumerate() {
        if can_solve(&prob, reg) {
            ans += 1;
        }
    }
    final_answer(ans, submit, DAY, SOL).await;
}

pub fn can_solve(prob: &Prob, reg: &Reg) -> bool {
    let start_state = vec![0u64; reg.h];
    let pres_req = &reg.idx_counts;
    solve_state(&start_state, reg.w, &pres_req)
}

pub fn solve_state(state: &Vec<u64>, w: usize, req: &Vec<usize>) -> bool {
    todo!()
}
