use std::collections::HashMap;

pub fn run(input: String) {
    let lines: Vec<String> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    let mut left_lane: Vec<u32> = vec![];
    let mut right_lane: Vec<u32> = vec![];

    let mut appearance_map: HashMap<u32, u32> = HashMap::new();

    for line in lines {
        let nums: Vec<u32> = line
            .split("   ")
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        let right_val = *nums.get(1).unwrap();

        left_lane.push(*nums.get(0).unwrap());
        right_lane.push(right_val);

        match appearance_map.get(&right_val) {
            Some(x) => appearance_map.insert(right_val, x + 1),
            None => appearance_map.insert(right_val, 1),
        };
    }

    left_lane.sort();
    right_lane.sort();

    let mut diffs: Vec<u32> = vec![];
    for i in 0..left_lane.len() {
        let diff: u32 = left_lane
            .get(i)
            .unwrap()
            .abs_diff(*right_lane.get(i).unwrap());
        diffs.push(diff);
    }

    let mut sum: u32 = 0;
    for diff in diffs {
        sum += diff;
    }
    println!("Your distance is: {}", sum);

    let mut appearance_sum: u32 = 0;
    for entry in left_lane {
        match appearance_map.get(&entry) {
            None => {}
            Some(x) => appearance_sum += entry * x,
        };
    }
    println!("The similarity score is: {}", appearance_sum);
}
