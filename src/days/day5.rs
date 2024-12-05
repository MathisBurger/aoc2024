use std::collections::HashMap;

type RuleMapping = HashMap<u8, Vec<u8>>;

struct RuleSet {
    backwards: RuleMapping,
}

impl From<String> for RuleSet {
    fn from(value: String) -> Self {
        let rules: Vec<&str> = value.split("\n").filter(|l| l.contains("|")).collect();

        let mut backward_rules: RuleMapping = HashMap::new();

        for rule_line in rules {
            let parts: Vec<u8> = rule_line
                .split("|")
                .map(|num| num.parse::<u8>().unwrap())
                .collect();
            let first = parts.get(0).unwrap();
            let last = parts.get(1).unwrap();

            let mut backwards_vec: Vec<u8> =
                backward_rules.get(last).unwrap_or(&Vec::new()).clone();
            backwards_vec.push(*first);
            backward_rules.insert(*last, backwards_vec);
        }

        RuleSet {
            backwards: backward_rules,
        }
    }
}

impl RuleSet {
    pub fn validate_order(&self, row: &String) -> Option<u8> {
        let mut partials: Vec<u8> = row
            .split(",")
            .map(|num| num.parse::<u8>().unwrap())
            .collect();
        partials.reverse();

        let rule_set = Self::filter_out_non_included(self.backwards.clone(), partials.clone());

        for index in 0..partials.len() {
            if !Self::nums_are_after(rule_set.get(partials.get(index).unwrap()), &partials, index) {
                return None;
            }
        }

        let middle_index = partials.len() - 1 - (partials.len() / 2);
        Some(*partials.get(middle_index).unwrap())
    }

    pub fn update_order(&self, row: &String) -> Option<u8> {
        if self.validate_order(row).is_some() {
            return None;
        }

        let mut partials: Vec<u8> = row
            .split(",")
            .map(|num| num.parse::<u8>().unwrap())
            .collect();

        let rule_set = Self::filter_out_non_included(self.backwards.clone(), partials.clone());

        let mut new_ordered: Vec<u8> = vec![];

        let len = partials.len();

        for _ in 0..len {
            for (i, num) in partials.iter().enumerate() {
                if Self::validate_array_equality(rule_set.get(num), &partials, *num) {
                    new_ordered.push(*num);
                    partials.remove(i);
                    break;
                }
            }
        }

        let middle_index = new_ordered.len() - 1 - (new_ordered.len() / 2);
        Some(*new_ordered.get(middle_index).unwrap())
    }

    fn validate_array_equality(rule_option: Option<&Vec<u8>>, row: &Vec<u8>, filter: u8) -> bool {
        let mut rule: &Vec<u8> = &vec![];
        if let Some(unwrapped) = rule_option {
            rule = unwrapped;
        }

        let filtered_row: Vec<u8> = row.iter().filter(|v| **v != filter).map(|v| *v).collect();

        if filtered_row.len() != rule.len() {
            return false;
        }

        for val in rule {
            if !row.contains(val) {
                return false;
            }
        }
        true
    }

    fn nums_are_after(rule_option: Option<&Vec<u8>>, row: &Vec<u8>, index: usize) -> bool {
        if rule_option.is_none() {
            return true;
        }
        let rule = rule_option.unwrap();
        if rule.len() == 0 {
            return true;
        }

        if index + 1 == row.len() && rule.len() > 0 {
            return false;
        }

        for i in (index + 1)..row.len() {
            let val = row.get(i).unwrap();
            if !rule.contains(val) {
                return false;
            }
        }
        true
    }

    fn filter_out_non_included(set: RuleMapping, row: Vec<u8>) -> RuleMapping {
        let mut new_mapping: RuleMapping = HashMap::new();

        for mapping in set {
            if row.contains(&mapping.0) {
                let filterd_rule: Vec<u8> = mapping
                    .1
                    .iter()
                    .filter(|num| row.contains(&num))
                    .map(|v| *v)
                    .collect();
                new_mapping.insert(mapping.0, filterd_rule);
            }
        }

        new_mapping
    }
}

pub fn run(contents: String) {
    let rule_set = RuleSet::from(contents.clone());

    let rows: Vec<String> = contents
        .split("\n")
        .filter(|r| r.contains(","))
        .map(|r| r.to_string())
        .collect();

    let mut sum: u32 = 0;

    for row in rows.clone() {
        if let Some(value) = rule_set.validate_order(&row) {
            sum += value as u32;
        }
    }
    println!("Task 1: {}", sum);

    rule_set.update_order(rows.get(3).unwrap());

    let mut sum2: u32 = 0;

    for row in rows {
        if let Some(value) = rule_set.update_order(&row) {
            sum2 += value as u32;
        }
    }
    println!("Task 2: {}", sum2);
}
