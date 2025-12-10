use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 10;
const SOL: u8 = 1;

#[derive(Debug)]
pub struct Machine {
    pub lights: Vec<bool>,
    pub wires: Vec<Vec<u32>>,
    pub jolts: Vec<u32>,
}
impl Machine {
    pub fn from_str(input: &str) -> Self {
        let re_raw = r#"\[(.*)\] (.*) \{(.*)\}"#;
        let re = regex::Regex::new(re_raw).unwrap();
        let caps = re.captures(input).expect("re match fail");

        let lights = caps[1].chars()
            .map(|i| i == '#')
            .collect();

        // let wire_re_raw = r#"\(([\)]+)\)"#;
        // let wire_re = regex::Regex::new(wire_re_raw).unwrap();
        // let wire_caps = wire_re.captures_iter(&caps[2]);
        // println!("{wire_caps:#?}");
        // let mut wires = vec!();
        // for (_, [wire_cap_group, _]) in wire_caps.map(|c| c.extract()) {
        //     wires.push(csv_nums_to_vec(wire_cap_group));
        // }
        
        let cleaned = caps[2].replace("(", "").replace(")", "");
        let wires = cleaned.split(" ")
            .map(csv_nums_to_vec)
            .collect();

        let jolts = csv_nums_to_vec(&caps[3]);

        Self {
            lights,
            wires,
            jolts,
        }
    }
    pub fn debug_lights(&self) -> String {
        self.lights.iter()
            .map(|i| if *i { '#' } else { '.' })
            .collect()
    }
}

#[derive(Debug)]
pub struct SimpMach {
    pub lights: u32,
    pub wires: Vec<u32>,
}
impl SimpMach {
    pub fn from_mach(input: &Machine) -> Self {
        let mut lights = 0;
        for src_l in input.lights.iter() {
            lights = lights << 1;
            if *src_l {
                lights += 1;
            }
        }

        let mut wires = vec!();
        let num_lights = input.lights.len();
        for wire_group in &input.wires {
            // let mut wire_val = 0;
            // for wire in wire_group {
            //     wire_val += 1 << wire;
            // }
            // wires.push(wire_val);
            
            // bad solution...
            let mut tmp = vec![false;num_lights];
            for wire in wire_group {
                tmp[*wire as usize] = true;
            }
            let mut wire_val = 0;
            for t in tmp {
                wire_val = wire_val << 1;
                if t {
                    wire_val += 1;
                }
            }
            wires.push(wire_val);
        }

        Self {
            lights,
            wires,
        }
    }
}

pub fn csv_nums_to_vec(input: &str) -> Vec<u32> {
    input.split(",").map(|i| i.parse().expect("invalid num {i}")).collect()
}

pub async fn input(example: bool) -> Vec<Machine> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .filter(|item| item.len() > 0)
        .map(Machine::from_str)
        .collect();

    lines
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;

    // println!("{input:#?}");

    let mut ans = 0usize;

    for mach in &input {
        let simp = SimpMach::from_mach(mach);
        println!("{}\t{:b}", mach.debug_lights(), simp.lights);
        println!("M: {:?}", mach.wires);
        println!("S: {:?}", simp.wires);
        println!("");
        // let num_lights = mach.lights.len();
        // let start_state = vec![false;num_lights];
        // let fewest_buttons = find_fewest_buttons(&mach, &start_state);
        let fewest_buttons = find_simp_buttons(&simp, &[0]);
        ans += fewest_buttons;
    }
    
    final_answer(ans, submit, DAY, SOL).await;
}

pub fn find_simp_buttons(simp:&SimpMach, states: &[u32]) -> usize {
    if states.contains(&simp.lights) {
        return 0;
    }
    let mut new_states: Vec<u32> = Vec::new();
    for state in states {
        for btn in &simp.wires {
            new_states.push(*state ^ *btn);
        }
    }
    find_simp_buttons(simp, &new_states) + 1
    // let mut new_: Vec<usize> = Vec::with_capacity(simp.wires.len());
    // for btn in &simp.wires {
    //     presses.push(find_simp_buttons(simp, state ^ *btn) + 1);
    // }
    // *presses.iter().min().unwrap()
}

// pub fn find_fewest_buttons(mach: &Machine, state: &Vec<bool>) -> usize {
//     if state == simp.
//     0 // todo
// }
