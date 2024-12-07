use std::time::Instant;

#[derive(Clone)]
struct Calibration {
    pub result: u64,
    pub equation_members: Vec<u64>,
}

#[derive(Clone, PartialEq)]
enum Operand {
    Addition,
    Multiplication,
    Concat,
}

struct CalibrationsDataSet {
    calibrations: Vec<Calibration>,
}

impl From<&str> for Calibration {
    fn from(value: &str) -> Self {
        let splitted_up: Vec<&str> = value.split(": ").collect();
        let result = splitted_up.get(0).unwrap().parse::<u64>().unwrap();
        let equation_members: Vec<u64> = splitted_up
            .get(1)
            .unwrap()
            .split(" ")
            .map(|m| m.parse::<u64>().unwrap())
            .collect();
        Calibration {
            result,
            equation_members,
        }
    }
}

impl From<String> for CalibrationsDataSet {
    fn from(value: String) -> Self {
        let equations = value
            .split("\n")
            .filter(|l| !l.is_empty())
            .map(|eq| Calibration::from(eq))
            .collect();
        CalibrationsDataSet {
            calibrations: equations,
        }
    }
}

impl CalibrationsDataSet {
    pub fn task1(&self) -> u64 {
        let mut solvable: u64 = 0;
        for calibration in self.calibrations.clone() {
            let combinations =
                Self::get_operand_combinations(calibration.equation_members.len() - 1);
            if Self::equation_solvable(&calibration, &combinations) {
                solvable += calibration.result;
            }
        }

        solvable
    }

    pub fn task2(&self) -> u64 {
        let mut solvable: u64 = 0;
        for calibration in self.calibrations.clone() {
            let combinations =
                Self::get_operand_combinations_task2(calibration.equation_members.len() - 1);
            if Self::equation_solvable(&calibration, &combinations) {
                solvable += calibration.result;
            }
        }

        solvable
    }

    fn equation_solvable(eq: &Calibration, op_combinations: &Vec<Vec<Operand>>) -> bool {
        for combination in op_combinations {
            let calc_res = Self::calculate(eq, &combination, eq.result);
            if eq.result == calc_res.0 {
                return true;
            }
        }

        false
    }

    fn get_operand_combinations(len: usize) -> Vec<Vec<Operand>> {
        let total = 1 << len;
        (0..total)
            .map(|num| {
                (0..len)
                    .rev()
                    .map(|i| {
                        if (num >> i) & 1 == 0 {
                            Operand::Addition
                        } else {
                            Operand::Multiplication
                        }
                    })
                    .collect()
            })
            .collect()
    }

    fn get_operand_combinations_task2(len: usize) -> Vec<Vec<Operand>> {
        let total = 3_usize.pow(len as u32);

        (0..total)
            .map(|num| {
                (0..len)
                    .rev()
                    .map(|i| match (num / 3_usize.pow(i as u32)) % 3 {
                        0 => Operand::Addition,
                        1 => Operand::Multiplication,
                        _ => Operand::Concat,
                    })
                    .collect()
            })
            .collect()
    }

    // This could be further improved to also exclude next results with similar structure
    fn calculate(eq: &Calibration, ops: &Vec<Operand>, desired: u64) -> (u64, usize) {
        let mut sum: u64 = Self::combine_with_op(
            *eq.equation_members.get(0).unwrap(),
            *eq.equation_members.get(1).unwrap(),
            ops.get(0).unwrap(),
        );

        for i in 2..eq.equation_members.len() {
            // early return invalid result
            if sum > desired {
                return (sum, i);
            }
            sum = Self::combine_with_op(
                sum,
                *eq.equation_members.get(i).unwrap(),
                ops.get(i - 1).unwrap(),
            );
        }
        (sum, eq.equation_members.len() - 1)
    }

    fn combine_with_op(a: u64, b: u64, op: &Operand) -> u64 {
        match op {
            Operand::Addition => a + b,
            Operand::Multiplication => a * b,
            Operand::Concat => format!("{}{}", a, b).parse::<u64>().unwrap_or(u64::MAX),
        }
    }
}

pub fn run(contents: String) {
    let data_set = CalibrationsDataSet::from(contents);
    let start = Instant::now();
    println!("Task 1: {}", data_set.task1());
    println!("Task 2: {}", data_set.task2());
    let section_1_time = start.elapsed();
    println!("Tasks took: {:?}", section_1_time);
}
