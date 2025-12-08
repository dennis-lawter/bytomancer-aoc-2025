use super::d08s1::*;
use super::solutions::final_answer;
use std::collections::BTreeMap;

const DAY: u8 = 08;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    // println!("{input:#?}");

    let mut conns = ConnDb::new();

    let mut memo: BTreeMap<(usize, usize), u64> = BTreeMap::new();

    for (l, lp) in input.iter().enumerate() {
        for (r, rp) in input.iter().enumerate() {
            if l <= r {
                continue;
            }
            let dist_sq = lp.get_dist_sq(rp);
            memo.insert((l,r), dist_sq);
        }
    }

    let mut memo_sorted: Vec<(usize,usize,u64)> = memo
        .iter()
        .map(|(k,v)| (k.0, k.1, *v))
        .collect();
    memo_sorted.sort_by(|l, r| l.2.cmp(&r.2) );

    for (l, r, _val) in &memo_sorted {
        let lp = &input[*l];
        let rp = &input[*r];
        conns.register(lp, rp);
    }

    //println!("{conns:#?}");
    println!("\n\n\n");

    // for conn in conns.db.iter() {
    //     println!(
    //         "{:?}\t{:?}\t{}",
    //         conn.0,
    //         conn.1,
    //         conn.0.get_dist_sq(&conn.1)
    //     );
    // }

    let mut cdb = CircDb::new();
    for node in &input {
        cdb.new_circuit(node);
    }
    let mut ans = 0;
    for conn in conns.db.iter() {
        let lp = &conn.0;
        let rp = &conn.1;
        cdb.combine(lp, rp);
        if !cdb.can_combine_more() {
            println!("Combined {:?} and {:?}", lp, rp);
            ans = lp.x * rp.x;
            break;
        }
    }

    // println!("{cdb:#?}");
    // let mut sizes = cdb.sizes();
    // sizes.sort();
    // sizes.reverse();
    // println!("{:?}", sizes);
    //
    // let ans = sizes[0]*sizes[1]*sizes[2];
    
    //let ans = 0;

    final_answer(ans, submit, DAY, SOL).await;
}
