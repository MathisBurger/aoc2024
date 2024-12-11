use std::collections::HashMap;

struct StoneLine {
    inner: Vec<u64>,
    cache: HashMap<(u64, usize), usize>,
}

impl From<String> for StoneLine {
    fn from(value: String) -> Self {
        let inner: Vec<u64> = value
            .replace("\n", "")
            .split(" ")
            .map(|num| num.parse::<u64>().unwrap())
            .collect();
        StoneLine {
            inner,
            cache: HashMap::new(),
        }
    }
}

impl StoneLine {
    /*pub fn simulate_blink(&mut self, amount: usize) -> usize {
        let mut count: usize = 0;
        while i < self.inner.len() {
            let stone = self.inner.get_mut(i).unwrap();
            if 0 == *stone {
                *stone = 1;
            } else if Self::has_even_digits(*stone) {
                let (num1, num2) = Self::split_number_in_half(*stone);
                *stone = num2;
                self.inner.insert(i, num1);
                i += 1;
            } else {
                *stone *= 2024;
            }
            i += 1;
        }
        count
    }*/

    pub fn simulate_blink(&mut self, amount: usize) -> usize {
        return self.blink_for_num(self.inner.clone(), amount);
    }

    fn blink_for_num(&mut self, nums: Vec<u64>, blinks: usize) -> usize {
        if blinks == 0 {
            return nums.len();
        }
        let mut new_nums: Vec<u64> = vec![];
        for num in nums {
            if 0 == num {
                new_nums.push(1);
            } else if Self::has_even_digits(num) {
                let (num1, num2) = Self::split_number_in_half(num);
                new_nums.push(num1);
                new_nums.push(num2);
            } else {
                new_nums.push(num * 2024);
            }
        }
        let mut count: usize = 0;
        for new_num in new_nums {
            if let Some(c_val) = self.cache.get(&(new_num, blinks)) {
                count += c_val;
            } else {
                let c_inc = self.blink_for_num(vec![new_num], blinks - 1);
                self.cache.insert((new_num, blinks), c_inc);
                count += c_inc;
            }
        }
        count
    }

    fn has_even_digits(n: u64) -> bool {
        let mut num = n;
        let mut digit_count = 0;

        while num > 0 {
            digit_count += 1;
            num /= 10;
        }

        digit_count % 2 == 0
    }

    fn split_number_in_half(n: u64) -> (u64, u64) {
        let num_str = n.to_string();
        let len = num_str.len();
        let mid = len / 2;
        let (first_half_str, second_half_str) = num_str.split_at(mid + len % 2);

        let first_half = first_half_str.parse::<u64>().unwrap();
        let second_half = second_half_str.parse::<u64>().unwrap();

        (first_half, second_half)
    }
}

pub fn run(contents: String) {
    let mut line = StoneLine::from(contents);
    println!("Task 1: {}", line.simulate_blink(25));
    println!("Task 2: {}", line.simulate_blink(75));
}
