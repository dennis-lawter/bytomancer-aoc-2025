use super::d11s1::*;
use super::solutions::final_answer;
use std::collections::HashSet;

const DAY: u8 = 11;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let tree = dev_list_to_btree_map(input);

    let can_reach_fft = find_all_with_access_to(&tree, "fft");
    println!("FFT {can_reach_fft:?}");
    let can_reach_dac = find_all_with_access_to(&tree, "dac");
    println!("DAC {can_reach_dac:?}");
    let can_reach_out = find_all_with_access_to(&tree, "out");
    println!("OUT {can_reach_out:?}");
    println!("\n");

    // let mut optimization_ignoring_hack: Vec<String> = tree.keys().map(|i| i.to_string()).collect();
    // optimization_ignoring_hack.push("out".to_string()); // not a device, just an exit node...
    // let svr_to_fft_bad = traverse_all(&tree, "svr", "fft", &optimization_ignoring_hack);
    // println!("{svr_to_fft_bad:?}");

    let svr_to_fft = traverse_all(&tree, "svr", "fft", &can_reach_fft);
    println!("svr to fft # {}", svr_to_fft.len());
    // println!("\n\n{svr_to_fft:?}\n\n");
    // panic!("debug");
    let svr_to_dac = traverse_all(&tree, "svr", "dac", &can_reach_dac);
    println!("svr to dac # {}", svr_to_dac.len());

    let fft_to_dac = traverse_all(&tree, "fft", "dac", &can_reach_dac);
    println!("fft to dac # {}", fft_to_dac.len());
    let dac_to_fft = traverse_all(&tree, "dac", "fft", &can_reach_fft);
    println!("dac to fft # {}", dac_to_fft.len());

    let fft_to_out = traverse_all(&tree, "fft", "out", &can_reach_out);
    println!("fft to out # {}", fft_to_out.len());
    let dac_to_out = traverse_all(&tree, "dac", "out", &can_reach_out);
    println!("fft to out # {}", dac_to_out.len());

    let sfdo = svr_to_fft.len() * fft_to_dac.len() * dac_to_out.len();
    let sdfo = svr_to_dac.len() * dac_to_fft.len() * fft_to_out.len();
    let ans = sfdo + sdfo;

    final_answer(ans, submit, DAY, SOL).await;
}

pub fn find_all_with_access_to(tree: &DevTree, node: &str) -> Vec<String> {
    let mut out = vec!();
    for (name, _) in tree.iter() {
        let mut memo = HashSet::new();
        if can_access(tree, name, node, &mut memo) {
            out.push(name.to_string());
        }
    }
    out.push(node.to_string()); // need this for optimization to work...
    out
}

pub fn can_access(tree: &DevTree, start: &str, end: &str, memo: &mut HashSet<String>) -> bool {
    memo.insert(start.to_string());
    // println!("testing {start}");
    let access = match tree.get(start) {
        Some(a) => a,
        None => {return false;}
    };
    for test in access {
        if test == end {
            return true;
        }
        if !memo.contains(test) && can_access(tree, test, end, memo) {
            return true;
        }
    }

    false
}
