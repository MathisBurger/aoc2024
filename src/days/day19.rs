use std::collections::HashMap;

struct Data {
    color_combinations: Vec<String>,
    desired: Vec<String>,
    memo: HashMap<String, usize>,
}

impl From<String> for Data {
    fn from(value: String) -> Self {
        let lines: Vec<&str> = value.split("\n").filter(|l| !l.is_empty()).collect();
        let combinations: Vec<String> = lines
            .get(0)
            .unwrap()
            .split(", ")
            .map(|comb| comb.to_string())
            .collect();

        Data {
            color_combinations: combinations,
            desired: lines
                .iter()
                .filter(|l| !l.contains(","))
                .map(|l| l.to_string())
                .collect(),
            memo: HashMap::new(),
        }
    }
}

impl Data {
    pub fn solve_all(&mut self) -> (usize, usize) {
        let mut solvable = 0;
        let mut solutions_count = 0;
        for wanted in self.desired.clone() {
            let res = Self::solve(&self.color_combinations, &wanted, &mut self.memo);
            if res > 0 {
                solvable += 1;
                solutions_count += res;
            }
        }

        (solvable, solutions_count)
    }

    fn solve(
        color_combinations: &Vec<String>,
        wanted: &String,
        memo: &mut HashMap<String, usize>,
    ) -> usize {
        if wanted.len() == 0 {
            return 1;
        }
        if let Some(cached) = memo.get(wanted) {
            return *cached;
        }

        let possibilities = color_combinations
            .iter()
            .filter(|cc| wanted.starts_with(*cc))
            .map(|cc| Self::solve(color_combinations, &wanted[cc.len()..].to_string(), memo))
            .sum();
        memo.insert(wanted.clone(), possibilities);
        possibilities
    }
}

pub fn run(contents: String) {
    let mut data = Data::from(contents);

    let results = data.solve_all();
    println!("Task 1: {}", results.0);
    println!("Task 2: {}", results.1);
}
