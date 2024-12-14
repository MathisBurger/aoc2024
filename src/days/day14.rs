/// Notice: I am aware of the fact that an matrix would provide better performance
/// for part two, but due to the small dataset of the task, I`ve went with the simpler approach here
/// Initially I thought task 2 would be something like: And now check what happens after you waited a decade...

#[derive(Clone, Copy)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

struct RoboMatrix {
    robots: Vec<Robot>,
    pub width: isize,
    pub height: isize,
}

impl From<String> for RoboMatrix {
    fn from(value: String) -> Self {
        let rob_lines: Vec<&str> = value.split("\n").filter(|l| !l.is_empty()).collect();
        let pv: Vec<Vec<&str>> = rob_lines.iter().map(|l| l.split(" ").collect()).collect();
        let robots: Vec<Robot> = pv
            .iter()
            .map(|pv_l| {
                let pos: Vec<isize> = pv_l
                    .get(0)
                    .unwrap()
                    .split("=")
                    .nth(1)
                    .unwrap()
                    .split(",")
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect();
                let vel: Vec<isize> = pv_l
                    .get(1)
                    .unwrap()
                    .split("=")
                    .nth(1)
                    .unwrap()
                    .split(",")
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect();
                Robot {
                    position: (*pos.get(0).unwrap(), *pos.get(1).unwrap()),
                    velocity: (*vel.get(0).unwrap(), *vel.get(1).unwrap()),
                }
            })
            .collect();
        RoboMatrix {
            robots,
            width: 101,
            height: 103,
        }
    }
}

impl RoboMatrix {
    pub fn task1(&mut self) -> u32 {
        self.simulate_runs(100);
        self.calculate_safety_factor()
    }

    pub fn task2(&mut self) -> u32 {
        let mut i = 0;
        loop {
            self.simulate_runs(1);
            if self.has_tree() {
                return i + 1;
            }
            i += 1;
        }
    }

    fn has_tree(&mut self) -> bool {
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                if self.robot_exists_at(x, y) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            if line.contains("########################") {
                return true;
            }
        }
        false
    }

    fn robot_exists_at(&mut self, x: isize, y: isize) -> bool {
        for robot in &self.robots {
            if robot.position.0 == x && robot.position.1 == y {
                return true;
            }
        }
        false
    }

    fn simulate_runs(&mut self, runs: isize) {
        for robot in self.robots.iter_mut() {
            robot.position.0 = (robot.position.0 + robot.velocity.0 * runs) % self.width;
            if robot.position.0 < 0 {
                robot.position.0 = self.width + robot.position.0;
            }
            robot.position.1 = (robot.position.1 + robot.velocity.1 * runs) % self.height;
            if robot.position.1 < 0 {
                robot.position.1 = self.height + robot.position.1;
            }
        }
    }

    fn calculate_safety_factor(&mut self) -> u32 {
        let x_div = (self.width - 1) / 2;
        let y_div = (self.height - 1) / 2;
        let mut q1: u32 = 0;
        let mut q2: u32 = 0;
        let mut q3: u32 = 0;
        let mut q4: u32 = 0;

        for robot in self.robots.clone() {
            if robot.position.0 < x_div {
                if robot.position.1 < y_div {
                    q1 += 1;
                } else if robot.position.1 > y_div {
                    q4 += 1;
                }
            } else if robot.position.0 > x_div {
                if robot.position.1 < y_div {
                    q2 += 1;
                } else if robot.position.1 > y_div {
                    q3 += 1;
                }
            }
        }
        q1 * q2 * q3 * q4
    }
}

pub fn run(contents: String) {
    let mut matrix = RoboMatrix::from(contents.clone());
    matrix.width = 101;
    matrix.height = 103;
    println!("Task 1: {}", matrix.task1());
    let mut matrix2 = RoboMatrix::from(contents);
    println!("Task 2: {}", matrix2.task2());
}
