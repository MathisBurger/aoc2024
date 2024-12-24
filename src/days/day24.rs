/// Todays puzzle was pretty fun. For part1 I had an pretty efficient recursive approach which worked for the
/// demo inputs, but not for my puzzle input. Therefore, I`ve switched to an intelligent iteratve approach which was
/// nearly as efficient as my recursive approach.
/// Part2 was al little trickier today. Fortunately, I still remember the way binary addition works from university.
/// So I started to build an algorithm that validates whether the operations follow the rules of binary additions. If they do not, they
/// must be wrong.
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq)]
enum Operator {
    OR,
    XOR,
    AND,
}

struct SwitchingNetwork {
    pub operations: Vec<(String, Operator, String, String)>,
    gate_values: HashMap<String, u64>,
}

impl From<String> for SwitchingNetwork {
    fn from(value: String) -> Self {
        let (input_lines_raw, net) = value.split_once("\n\n").unwrap();
        let mut inputs = HashMap::new();
        let input_lines: Vec<&str> = input_lines_raw
            .split("\n")
            .filter(|l| !l.is_empty())
            .collect();
        for input_line in input_lines {
            let (name, value) = input_line.split_once(": ").unwrap();
            inputs.insert(name.to_string(), value.parse::<u64>().unwrap());
        }

        let net_lines: Vec<&str> = net.split("\n").filter(|l| !l.is_empty()).collect();
        let mut operations = vec![];

        for net_line in net_lines {
            let tokens: Vec<&str> = net_line.split(" ").collect();
            let reg1 = tokens.get(0).unwrap().to_string();
            let reg2 = tokens.get(2).unwrap().to_string();
            let res = tokens.get(4).unwrap().to_string();
            let op = match *tokens.get(1).unwrap() {
                "OR" => Operator::OR,
                "AND" => Operator::AND,
                "XOR" => Operator::XOR,
                _ => unimplemented!(),
            };
            operations.push((reg1, op, reg2, res));
        }

        SwitchingNetwork {
            gate_values: inputs,
            operations,
        }
    }
}

impl SwitchingNetwork {
    pub fn task1(&mut self) -> u64 {
        self.calc_net();
        self.calc_net_output_decimal()
    }

    pub fn task2(&self, ops: &Vec<(String, Operator, String, String)>) -> String {
        let mut z_s: Vec<String> = self
            .gate_values
            .clone()
            .keys()
            .filter(|k| k.starts_with("z"))
            .cloned()
            .collect();
        z_s.sort();
        let mut wrong: Vec<String> = Self::check_wrong_operations(ops, &z_s[z_s.len() - 1])
            .iter()
            .cloned()
            .collect();
        wrong.sort();
        wrong.join(",")
    }

    fn check_wrong_operations(
        operations: &Vec<(String, Operator, String, String)>,
        highest_z: &String,
    ) -> HashSet<String> {
        let mut wrong = HashSet::new();

        for (op1, op, op2, res) in operations {
            // Check if the result starts with 'z', the operator is not XOR, and it's not the highest_z
            // Example: 01 + 01 = 10 => 1 XOR 1 = 0
            if res.starts_with("z") && *op != Operator::XOR && res != highest_z {
                wrong.insert(res.clone());
            }

            // Check if the operation is XOR and operands/res don't start with "x", "y", or "z"
            // The XOR as part of the addition is only allowed to before the result or
            // as the first step of binary addition between x and y
            // Could go a little deeper here, but should still work
            if *op == Operator::XOR
                && !["x", "y", "z"].contains(&&res[0..1])
                && !["x", "y", "z"].contains(&&op1[0..1])
                && !["x", "y", "z"].contains(&&op2[0..1])
            {
                wrong.insert(res.clone());
            }

            // Check if the operation is AND and neither operand is "x00"
            // Same reasoning as further upwards
            // Example: 01 + 01 = 10 => 1 AND 1 = 1 => Would be just wrong
            if *op == Operator::AND
                && !["x00".to_string()].contains(&op1)
                && !["x00".to_string()].contains(&op2)
            {
                for (subop1, subop, subop2, _) in operations {
                    // If result matches and sub-operation is not "OR", mark it as wrong
                    if (res == subop1 || res == subop2) && *subop != Operator::OR {
                        wrong.insert(res.clone());
                    }
                }
            }

            // Check for XOR operation and if sub-operation is OR
            if *op == Operator::XOR {
                for (subop1, subop, subop2, _) in operations {
                    // If result matches and sub-operation is "OR", mark it as wrong
                    if (res == subop1 || res == subop2) && *subop == Operator::OR {
                        wrong.insert(res.clone());
                    }
                }
            }
        }

        wrong
    }

    fn calc_net_output_decimal(&self) -> u64 {
        let mut outputs: Vec<String> = self
            .gate_values
            .clone()
            .keys()
            .filter(|k| k.starts_with("z"))
            .cloned()
            .collect();
        outputs.sort();
        let mut sum: u64 = 0;
        for i in 0..outputs.len() {
            let output_val = self.gate_values.get(outputs.get(i).unwrap()).unwrap();
            sum += output_val << i;
        }
        sum
    }

    fn calc_net(&mut self) {
        while let Some(operations) = self.get_solveable() {
            for operation in operations.clone() {
                self.gate_values.insert(
                    operation.3,
                    Self::calc_with_op(
                        *self.gate_values.get(&operation.0).unwrap(),
                        *self.gate_values.get(&operation.2).unwrap(),
                        operation.1,
                    ),
                );
            }
            self.operations = self
                .operations
                .iter()
                .filter(|op| !operations.contains(op))
                .cloned()
                .collect();
        }
    }

    fn get_solveable(&self) -> Option<Vec<(String, Operator, String, String)>> {
        let mut solvable = vec![];
        for operation in self.operations.clone() {
            if self.gate_values.contains_key(&operation.0)
                && self.gate_values.contains_key(&operation.2)
            {
                solvable.push(operation);
            }
        }
        if solvable.len() > 0 {
            return Some(solvable);
        }
        None
    }

    fn calc_with_op(reg1: u64, reg2: u64, op: Operator) -> u64 {
        match op {
            Operator::OR => reg1 | reg2,
            Operator::AND => reg1 & reg2,
            Operator::XOR => reg1 ^ reg2,
        }
    }
}

pub fn run(contents: String) {
    let mut network = SwitchingNetwork::from(contents.clone());
    let cloned_ops = network.operations.clone();
    println!("Task 1: {}", network.task1());
    println!("Task 2: {}", network.task2(&cloned_ops));
}
