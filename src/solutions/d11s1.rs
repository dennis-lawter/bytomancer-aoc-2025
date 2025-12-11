use super::solutions::final_answer;
use super::solutions::input_raw;
use std::collections::BTreeMap;
use std::collections::VecDeque;

const DAY: u8 = 11;
const SOL: u8 = 1;

pub type DevTree = BTreeMap<String, VecDeque<String>>;

pub struct Device {
    pub name: String,
    pub out: VecDeque<String>,
}
impl Device {
    pub fn from_str(input: &str) -> Self {
        let (name, outputs) = input.split_once(": ").unwrap();
        let name = name.to_owned();
        let out = outputs.split(" ").map(|i| i.to_owned()).collect();
        Self { name, out }
    }
}

pub fn dev_list_to_btree_map(devs: Vec<Device>) -> DevTree {
    let mut tree = BTreeMap::new();
    for dev in devs {
        tree.insert(dev.name, dev.out);
    }
    tree
}

pub async fn input(example: bool) -> Vec<Device> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .filter(|item| item.len() > 0)
        .map(|item| Device::from_str(item))
        .collect();

    lines
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let tree = dev_list_to_btree_map(input);
    let mut optimization_ignoring_hack: Vec<String> = tree.keys().map(|i| i.to_string()).collect();
    optimization_ignoring_hack.push("out".to_string()); // not a device, just an exit node...
    println!("{optimization_ignoring_hack:?}");
    let paths = traverse_all(&tree, "you", "out", &optimization_ignoring_hack);
    println!("{paths:#?}");
    final_answer(paths.len(), submit, DAY, SOL).await;
}

pub fn traverse_all(devs: &DevTree, start: &str, end: &str, can_get_there: &Vec<String>) -> Vec<Vec<String>> {
    println!("ALL from {start} to {end}");
    let first_path = vec![start.to_owned()];
    let goal = end.to_owned();

    let paths = vec![first_path];
    traverse_all_to(devs, &paths, &goal, can_get_there)
}

pub fn traverse_all_to(devs: &DevTree, paths: &Vec<Vec<String>>, goal: &str, can_get_there: &Vec<String>) -> Vec<Vec<String>> {
    let mut out = vec![];
    let mut paths_in_progress = paths.clone();
    while let Some(path) = paths_in_progress.pop() {
        let end = path.last().unwrap();
        if end == goal {
            out.push(path.clone());
        } else {
            let mut new_paths = traverse_step(devs, &path, can_get_there);
            // println!("{new_paths:?}");
            paths_in_progress.append(&mut new_paths);
        }
    }
    out
}

pub fn traverse_step(devs: &DevTree, path: &Vec<String>, can_get_there: &Vec<String>) -> Vec<Vec<String>> {
    let mut out = vec![];
    let end = path.last().unwrap();
    if let Some(end_list) = devs.get(end) {
        for next_step in end_list.iter() {
            // if !can_get_there.contains(next_step) {
                // println!("WTF: {next_step}");
            // }
            if !path.contains(next_step) && can_get_there.contains(next_step) {
                let mut new_path = path.clone();
                new_path.push(next_step.clone());
                out.push(new_path);
            }
        }
    }
    // } else {
    //     println!("WARN: no key found for {end}");
    // }
    out
}
