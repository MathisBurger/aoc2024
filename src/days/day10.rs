struct Matrix {
    map: Vec<Vec<u8>>,
}

impl From<String> for Matrix {
    fn from(value: String) -> Self {
        let lines: Vec<&str> = value.split("\n").filter(|l| !l.is_empty()).collect();
        let mut map = vec![];
        for line in lines {
            let row: Vec<u8> = line.chars().map(|c| c as u8 - '0' as u8).collect();
            map.push(row);
        }
        Matrix { map }
    }
}

impl Matrix {
    pub fn task1(&self) -> usize {
        let starting_points = self.find_starts();
        let mut count: usize = 0;
        for starting_point in starting_points {
            let mut ends: Vec<(usize, usize)> = vec![];
            self.find_paths(starting_point, &mut ends);
            count += ends.len();
        }
        count
    }

    pub fn task2(&self) -> usize {
        let starting_points = self.find_starts();
        let mut count: usize = 0;
        for starting_point in starting_points {
            let mut ends: Vec<(usize, usize)> = vec![];
            self.find_paths_task2(starting_point, &mut ends);
            count += ends.len();
        }
        count
    }

    fn find_paths(&self, start: (usize, usize), ends: &mut Vec<(usize, usize)>) {
        let current = self.get_value_at(start);

        if current == 9 {
            if !ends.contains(&start) {
                ends.push(start);
            }
            return;
        }
        if start.0 > 0 {
            let up = self.get_value_at((start.0 - 1, start.1));
            if up == current + 1 {
                self.find_paths((start.0 - 1, start.1), ends);
            }
        }
        if start.0 + 1 < self.map.len() {
            let down = self.get_value_at((start.0 + 1, start.1));
            if down == current + 1 {
                self.find_paths((start.0 + 1, start.1), ends);
            }
        }
        if start.1 > 0 {
            let left = self.get_value_at((start.0, start.1 - 1));
            if left == current + 1 {
                self.find_paths((start.0, start.1 - 1), ends);
            }
        }
        if start.1 + 1 < self.map.get(0).unwrap().len() {
            let right = self.get_value_at((start.0, start.1 + 1));
            if right == current + 1 {
                self.find_paths((start.0, start.1 + 1), ends);
            }
        }
    }

    fn find_paths_task2(&self, start: (usize, usize), ends: &mut Vec<(usize, usize)>) {
        let current = self.get_value_at(start);

        if current == 9 {
            ends.push(start);
            return;
        }
        if start.0 > 0 {
            let up = self.get_value_at((start.0 - 1, start.1));
            if up == current + 1 {
                self.find_paths_task2((start.0 - 1, start.1), ends);
            }
        }
        if start.0 + 1 < self.map.len() {
            let down = self.get_value_at((start.0 + 1, start.1));
            if down == current + 1 {
                self.find_paths_task2((start.0 + 1, start.1), ends);
            }
        }
        if start.1 > 0 {
            let left = self.get_value_at((start.0, start.1 - 1));
            if left == current + 1 {
                self.find_paths_task2((start.0, start.1 - 1), ends);
            }
        }
        if start.1 + 1 < self.map.get(0).unwrap().len() {
            let right = self.get_value_at((start.0, start.1 + 1));
            if right == current + 1 {
                self.find_paths_task2((start.0, start.1 + 1), ends);
            }
        }
    }

    fn get_value_at(&self, pos: (usize, usize)) -> u8 {
        *self.map.get(pos.0).unwrap().get(pos.1).unwrap()
    }

    fn find_starts(&self) -> Vec<(usize, usize)> {
        let mut starts: Vec<(usize, usize)> = vec![];
        for (x, row) in self.map.iter().enumerate() {
            for (y, element) in row.iter().enumerate() {
                if *element == 0 {
                    starts.push((x, y));
                }
            }
        }
        starts
    }
}

pub fn run(contents: String) {
    let mut matrix = Matrix::from(contents);
    println!("Task 1: {}", matrix.task1());
    println!("Task 2: {}", matrix.task2());
}
