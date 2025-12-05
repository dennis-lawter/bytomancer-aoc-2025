#![allow(warnings)]

use super::d05s1::*;
use super::solutions::final_answer;

const DAY: u8 = 05;
const SOL: u8 = 2;

pub async fn solve(submit: bool, example: bool) {
    let (db, _) = input(example).await;
    let db = sort_db(db);
    let mut db = kill_useless_rows(db);
    while let Some(merge_index) = db_needs_merge(&db) {
        //println!("{merge_index}");
        //println!("{db:#?}\n\n");
        do_merge(&mut db, merge_index);
    }
    //println!("{db:#?}");
    let ans = count_sizes(&db);
    final_answer(ans, submit, DAY, SOL).await;
}

pub fn sort_db(db: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut db = db;
    db.sort_by(|a, b| a.0.cmp(&b.0));
    db
}

pub fn kill_useless_rows(db: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut new_db = vec![];
    let mut bad_db_rows = vec![];
    for row1 in &db {
        for row2 in &db {
            if row1 != row2 && row2.0 >= row1.0 && row2.1 <= row1.1 {
                // row2 is fully contained in row1!
                bad_db_rows.push(row2.clone());
            }
        }
    }
    for row in &db {
        if !bad_db_rows.contains(row) {
            new_db.push(row.clone());
        }
    }
    new_db
}

pub fn db_needs_merge(db: &Vec<(u64, u64)>) -> Option<usize> {
    for i in 0..db.len() - 1 {
        let (_, curr_r) = &db[i];
        let (next_l, next_r) = &db[i + 1];
        if curr_r >= next_l && curr_r <= next_r {
            return Some(i);
        }
    }
    None
}

pub fn do_merge(db: &mut Vec<(u64, u64)>, i: usize) {
    let first = db.remove(i);
    let second = db.remove(i);
    let new = merge_two_ranges(&first, &second);
    db.insert(i, new);
}

pub fn merge_two_ranges(r1: &(u64, u64), r2: &(u64, u64)) -> (u64, u64) {
    let ls = [r1.0, r2.0];
    let rs = [r1.1, r2.1];
    (*ls.iter().min().unwrap(), *rs.iter().max().unwrap())
}

pub fn count_sizes(db: &Vec<(u64, u64)>) -> u64 {
    let mut ans = 0;
    for row in db {
        ans += row.1 - row.0 + 1;
    }
    ans
}
