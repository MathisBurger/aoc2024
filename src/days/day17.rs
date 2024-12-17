struct NeumannArchitecture {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    prog: Vec<u64>,
    output: Vec<u64>,
}

impl From<String> for NeumannArchitecture {
    fn from(value: String) -> Self {
        let lines: Vec<&str> = value.split("\n").filter(|l| !l.is_empty()).collect();
        let values: Vec<&str> = lines
            .iter()
            .map(|l| l.split(": ").nth(1).unwrap())
            .collect();
        NeumannArchitecture {
            reg_a: values.get(0).unwrap().parse::<u64>().unwrap(),
            reg_b: values.get(1).unwrap().parse::<u64>().unwrap(),
            reg_c: values.get(2).unwrap().parse::<u64>().unwrap(),
            prog: values
                .get(3)
                .unwrap()
                .split(",")
                .map(|num| num.parse::<u64>().unwrap())
                .collect(),
            output: vec![],
        }
    }
}

impl NeumannArchitecture {
    pub fn task1(&mut self) -> String {
        let mut step_counter: usize = 0;
        while self.can_execute(step_counter) {
            self.execute(&mut step_counter);
        }

        self.output
            .iter()
            .map(|&n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn task2(&mut self) -> u64 {
        self.get_best_quine_input(self.prog.len() - 1, 0).unwrap()
    }

    fn get_best_quine_input(&mut self, cursor: usize, sofar: u64) -> Option<u64> {
        for candidate in 0..8 {
            // Calculate the candidate and run the program with it
            let input = sofar * 8 + candidate;
            let output = self.run(input);

            if output == &self.prog[cursor..] {
                // If cursor is 0, return the input value
                if cursor == 0 {
                    return Some(input);
                }

                // Otherwise, recursively call get_best_quine_input with cursor - 1
                if let Some(ret) = self.get_best_quine_input(cursor - 1, input) {
                    return Some(ret);
                }
            }
        }
        None
    }

    fn run(&mut self, reg_a: u64) -> Vec<u64> {
        self.reg_a = reg_a;
        self.reg_b = 0;
        self.reg_c = 0;
        self.output = vec![];
        self.task1();
        self.output.clone()
    }

    fn can_execute(&self, index: usize) -> bool {
        return self.prog.len() > index + 1;
    }

    fn execute(&mut self, index: &mut usize) {
        let op_code = self.prog.get(*index).unwrap();
        let exec = self.prog.get(*index + 1).unwrap();
        // Valid
        if *op_code == 0 {
            self.reg_a /= u64::pow(2, self.obtain_value(*exec));
        }
        // Valid
        if *op_code == 1 {
            self.reg_b ^= *exec as u64;
        }
        if *op_code == 2 {
            self.reg_b = (self.obtain_value(*exec) % 8) as u64;
        }
        // Valid
        if *op_code == 3 && self.reg_a != 0 {
            *index = *exec as usize;
            return;
        }
        // Valid
        if *op_code == 4 {
            self.reg_b ^= self.reg_c;
        }
        // Valid
        if *op_code == 5 {
            self.output.push((self.obtain_value(*exec) % 8) as u64);
        }
        if *op_code == 6 {
            self.reg_b = self.reg_a / u64::pow(2, self.obtain_value(*exec));
        }
        if *op_code == 7 {
            self.reg_c = self.reg_a / u64::pow(2, self.obtain_value(*exec));
        }
        *index += 2;
    }

    fn obtain_value(&self, exec: u64) -> u32 {
        if exec <= 3 {
            return exec as u32;
        }
        if exec == 4 {
            return self.reg_a as u32;
        }
        if exec == 5 {
            return self.reg_b as u32;
        }
        if exec == 6 {
            return self.reg_c as u32;
        }
        unreachable!();
    }
}

pub fn run(contents: String) {
    let mut neumann = NeumannArchitecture::from(contents);
    println!("Task 1: {}", neumann.task1());
    println!("Task 2: {}", neumann.task2());
}
