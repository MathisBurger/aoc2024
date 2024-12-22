/// Todays challange was really func, because it was something different than a 2d matrix and manhattan distances or path finding algorithms and graph theory.
/// I really enjoyed it.
use std::collections::{HashMap, HashSet};

struct Calculator {
    secrets: Vec<usize>,
    cache_all: HashMap<usize, usize>,
    sequence_totals: HashMap<(isize, isize, isize, isize), usize>,
}

impl From<String> for Calculator {
    fn from(value: String) -> Self {
        let secrets: Vec<usize> = value
            .split("\n")
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<usize>().unwrap())
            .collect();
        Calculator {
            secrets,
            cache_all: HashMap::new(),
            sequence_totals: HashMap::new(),
        }
    }
}

impl Calculator {
    pub fn task1(&mut self) -> usize {
        let mut sum = 0;
        for num in self.secrets.clone() {
            sum += self.calculate_nth_secret_number(num, 2);
        }
        sum
    }

    pub fn task2(&mut self) -> usize {
        for num in self.secrets.clone() {
            self.select_sequences(num);
        }

        let mut most_bananas: usize = 0;
        let mut seq = (0, 0, 0, 0);
        for sequence in self.sequence_totals.iter() {
            if *sequence.1 > most_bananas {
                most_bananas = *sequence.1;
                seq = *sequence.0;
            }
        }
        most_bananas
    }

    fn select_sequences(&mut self, num: usize) {
        let mut secret = num;
        let mut changes = vec![];
        let mut values = vec![secret % 10];
        let mut last = secret % 10;
        let mut sequences_got = HashSet::new();
        for _ in 0..2000 {
            if let Some(cached) = self.cache_all.get(&secret) {
                secret = *cached;
            } else {
                let res = Self::calc_step3(Self::calc_step2(Self::calc_step1(secret)));
                self.cache_all.insert(secret, res);
                secret = res;
            }
            values.push(secret % 10);
            changes.push((secret % 10) as isize - last as isize);
            last = secret % 10;
        }
        for i in 4..values.len() {
            let seq = (
                changes[i - 4],
                changes[i - 3],
                changes[i - 2],
                changes[i - 1],
            );
            if !sequences_got.contains(&seq) {
                if let Some(val) = self.sequence_totals.get_mut(&seq) {
                    *val += values[i];
                } else {
                    self.sequence_totals.insert(seq, values[i]);
                }
                sequences_got.insert(seq);
            }
        }
    }

    fn calculate_nth_secret_number(&mut self, secret_param: usize, nth: usize) -> usize {
        let mut secret = secret_param;
        for _ in 0..nth {
            if let Some(cached) = self.cache_all.get(&secret) {
                secret = *cached;
            } else {
                let res = Self::calc_step3(Self::calc_step2(Self::calc_step1(secret)));
                self.cache_all.insert(secret, res);
                secret = res;
            }
        }
        secret
    }

    fn calc_step1(before: usize) -> usize {
        Self::prune(Self::mix(before, before * 64))
    }

    fn calc_step2(before: usize) -> usize {
        Self::prune(Self::mix(before, before / 32))
    }

    fn calc_step3(before: usize) -> usize {
        Self::prune(Self::mix(before, before * 2048))
    }

    fn mix(first: usize, second: usize) -> usize {
        first ^ second
    }

    fn prune(num: usize) -> usize {
        num % 16777216
    }
}

pub fn run(contents: String) {
    let mut calculator = Calculator::from(contents);
    println!("Task 1: {}", calculator.task1());
    println!("Task 2: {}", calculator.task2());
}
