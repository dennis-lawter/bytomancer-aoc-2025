use super::d10s1::*;
use super::solutions::final_answer;
use std::collections::HashSet;

const DAY: u8 = 10;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let mut input = input(example).await;

    // if example {
    //     println!("{input:#?}");
    // }
    for m in input.iter_mut() {
        m.wires.sort_by(|a, b| {
            let alen = a.len();
            let blen = b.len();
            alen.cmp(&blen)
        });
        m.wires.reverse();
    }
    // if example {
    //     println!("{input:#?}");
    // }

    let mut ans = 0;
    let num_machs = input.len();
    for (i, mach) in input.iter().enumerate() {
        let perc = i as f64 / num_machs as f64;
        println!(
            "Processing machine {} / {} ({:.2}%)",
            i,
            num_machs,
            perc * 100.0
        );
        let num_jolts = mach.jolts.len();
        let init_state = vec![0u32; num_jolts];
        // let init_states = vec![init_state];
        let mut memo: HashSet<String> = HashSet::new();
        // let presses = find_jolt_buttons(&mach, &init_state, &mut memo).expect("No valid solution?");
        // tried bfs, tried dfs & dfs+memo, now going to try left-fill
        let presses = find_jolt_buttons(&mach, &init_state, &mut memo).expect("No valid solution?");
        ans += presses;
    }

    final_answer(ans, submit, DAY, SOL).await;
}

pub fn pattern_filter(wires: &&Vec<u32>, notouch: &Vec<usize>, want: usize) -> bool {
    for wire in wires.iter() {
        for i in notouch {
            if *wire as usize == *i {
                return false;
            }
        }
    }
    wires.contains(&(want as u32))
}

pub fn find_jolt_buttons(
    mach: &Machine,
    state: &Vec<u32>,
    memo: &mut HashSet<String>,
) -> Option<usize> {
    let goal = &mach.jolts;
    if goal == state {
        return Some(0);
    }
    let state_key = format!("{:?}", state);
    if memo.contains(&state_key) {
        return None;
    }
    memo.insert(state_key);
    let mut idx_need_jolts: Vec<usize> = vec![];
    let mut idx_no_touch: Vec<usize> = vec![];
    for i in 0..goal.len() {
        if state[i] > goal[i] {
            return None;
        } else if state[i] == goal[i] {
            idx_no_touch.push(i);
        }
        if state[i] < goal[i] {
            idx_need_jolts.push(i);
        }
    }
    for idx in 0..idx_need_jolts.len() {
        let i_want_this_number = idx_need_jolts[idx];
        // println!("{i_want_this_number}");
        let patterns_to_apply: Vec<Vec<u32>> = mach
            .wires
            .iter()
            .filter(|w| pattern_filter(w, &idx_no_touch, i_want_this_number))
            .map(|w| w.clone())
            .collect();
        // println!("{patterns_to_apply:?}");
        for p in patterns_to_apply {
            let new_state = apply_pattern_to_state(&p, &state);
            if let Some(x) = find_jolt_buttons(mach, &new_state, memo) {
                return Some(x + 1);
            }
        }
        return None; // remove in case I want to go through this loop; but I doubt I should...
    }
    None
}

// pub fn find_jolt_buttons(mach:&Machine, state:&Vec<u32>) -> Option<usize> {
//     let goal = &mach.jolts;
//     if goal == state {
//         return Some(0);
//     }
//     for i in 0..goal.len() {
//         if state[i] > goal[i] {
//             return None;
//         }
//     }
//     let state_key = format!("{:?}", state);
//     if memo.contains(&state_key) {
//         return None;
//     }
//     memo.insert(state_key);
//
//     for btn_pattern in &mach.wires {
//         let new_state = apply_pattern_to_state(btn_pattern, state);
//         if let Some(x) = find_jolt_buttons(mach, &new_state, memo) {
//             return Some(x+1);
//         }
//     }
//     None
// }

pub fn apply_pattern_to_state(pat: &Vec<u32>, state: &Vec<u32>) -> Vec<u32> {
    // println!("Applying {:?} to {:?}", pat, state);
    let mut new_state = state.clone();
    for i in pat {
        new_state[*i as usize] += 1;
    }
    new_state
}

// pub fn find_jolt_buttons(mach:&Machine, states: &Vec<Vec<u32>>) -> usize {
//     let nstates = states.len();
//     assert_ne!(0, nstates);
//     println!("{nstates}");
//
//     if states.contains(&mach.jolts) {
//         return 0;
//     }
//     let mut new_states: Vec<Vec<u32>> = Vec::new();
//     for state in states {
//         for wire in &mach.wires {
//             let mut new_state = state.clone();
//             for i in wire {
//                 new_state[*i as usize] += 1;
//             }
//             //
//              let mut valid = true;
//             for i in 0..mach.jolts.len() {
//                 if new_state[i] > mach.jolts[i] {
//                     valid = false;
//                     break;
//                 }
//             }
//             if valid && !new_states.contains(&new_state) {
//                 // println!("Adding {new_state:?}");
//                 new_states.push(new_state);
//             }
//         }
//     }
//
//     find_jolt_buttons(mach, &new_states) + 1
// }
