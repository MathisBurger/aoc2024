#[derive(PartialEq, Clone, Copy)]
enum FieldType {
    Obstruction,
    Free,
    Visited,
    StartPos,
}

#[derive(Clone)]
struct Map {
    empty_map: Vec<Vec<FieldType>>,
    inner_map: Vec<Vec<FieldType>>,
    init_position: (usize, usize),
    current_position: (usize, usize),
    current_direction: (i8, i8),
}

impl From<&str> for FieldType {
    fn from(value: &str) -> Self {
        match value {
            "#" => FieldType::Obstruction,
            "^" => FieldType::StartPos,
            _ => FieldType::Free,
        }
    }
}

impl From<String> for Map {
    fn from(value: String) -> Self {
        let lines: Vec<&str> = value.split("\n").filter(|l| !l.is_empty()).collect();
        let mut map: Vec<Vec<FieldType>> = vec![];
        let mut start_pos = (0, 0);
        for (i, line) in lines.iter().enumerate() {
            let elements: Vec<FieldType> = line
                .split("")
                .enumerate()
                .map(|(j, e)| {
                    let t = FieldType::from(e);
                    if t == FieldType::StartPos {
                        start_pos = (i, j);
                    }
                    t
                })
                .collect();
            map.push(elements)
        }
        Map {
            inner_map: map.clone(),
            empty_map: map,
            current_position: start_pos,
            init_position: start_pos,
            current_direction: (-1, 0),
        }
    }
}

impl Map {
    // The max size of the matrix
    const MAX_CIRCLE_THRESHOLD: u8 = 131;

    pub fn run_task1(&mut self) -> u32 {
        self.walk_paths();
        let mut count: u32 = 0;
        for row in self.inner_map.clone() {
            for element in row {
                if element == FieldType::Visited {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn run_task2(&mut self) -> u32 {
        let mut count: u32 = 0;

        for (x, row) in self.inner_map.clone().iter().enumerate() {
            for (y, element) in row.iter().enumerate() {
                if element == &FieldType::Visited {
                    let mut new_map = self.new_resetted();
                    new_map.set_field_status((x, y), FieldType::Obstruction);
                    if new_map.walk_paths_circle_detection() {
                        count += 1;
                    }
                }
            }
        }
        count
        // Step 1: Place Obstacle for every visited
        // Step 2: Add already visited_count_in_row count if > 10 then is in loop
    }

    fn new_resetted(&self) -> Map {
        Map {
            empty_map: self.empty_map.clone(),
            inner_map: self.empty_map.clone(),
            init_position: self.init_position,
            current_position: self.init_position,
            current_direction: (-1, 0),
        }
    }

    fn walk_paths(&mut self) {
        loop {
            if self.next_walk_leaves_map() {
                break;
            }

            if self.next_walk_is_obstruction() {
                self.change_direction();
                continue;
            }

            let x = self.current_position.0 as isize + self.current_direction.0 as isize;
            let y = self.current_position.1 as isize + self.current_direction.1 as isize;
            self.current_position = (x as usize, y as usize);
            self.set_field_status(self.current_position, FieldType::Visited);
        }
    }

    fn walk_paths_circle_detection(&mut self) -> bool {
        let mut multi_count: u8 = 0;

        loop {
            if self.next_walk_leaves_map() {
                break;
            }

            if self.next_walk_is_obstruction() {
                self.change_direction();
                continue;
            }

            let x = self.current_position.0 as isize + self.current_direction.0 as isize;
            let y = self.current_position.1 as isize + self.current_direction.1 as isize;
            self.current_position = (x as usize, y as usize);

            if self.get_field_status(self.current_position) == FieldType::Visited {
                multi_count += 1;
            } else {
                multi_count = 0;
            }

            if multi_count > Self::MAX_CIRCLE_THRESHOLD {
                break;
            }

            self.set_field_status(self.current_position, FieldType::Visited);
        }

        return multi_count > Self::MAX_CIRCLE_THRESHOLD;
    }

    fn next_walk_leaves_map(&mut self) -> bool {
        if self.current_direction.0 > 0 && self.current_position.0 == self.inner_map.len() - 1 {
            return true;
        }
        if self.current_direction.0 < 0 && self.current_position.0 == 0 {
            return true;
        }
        if self.current_direction.1 > 0
            && self.current_position.1 == self.inner_map.get(0).unwrap().len() - 1
        {
            return true;
        }
        if self.current_direction.1 < 0 && self.current_position.1 == 0 {
            return true;
        }
        false
    }

    fn next_walk_is_obstruction(&mut self) -> bool {
        let x = self.current_position.0 as isize + self.current_direction.0 as isize;
        let y = self.current_position.1 as isize + self.current_direction.1 as isize;
        return self
            .inner_map
            .get(x as usize)
            .unwrap()
            .get(y as usize)
            .unwrap()
            == &FieldType::Obstruction;
    }

    fn change_direction(&mut self) {
        if self.current_direction == (-1, 0) {
            self.current_direction = (0, 1);
        } else if self.current_direction == (0, 1) {
            self.current_direction = (1, 0);
        } else if self.current_direction == (1, 0) {
            self.current_direction = (0, -1);
        } else {
            self.current_direction = (-1, 0);
        }
    }

    fn set_field_status(&mut self, pos: (usize, usize), ftype: FieldType) {
        let mut_val = self
            .inner_map
            .get_mut(pos.0)
            .unwrap()
            .get_mut(pos.1)
            .unwrap();
        *mut_val = ftype;
    }

    fn get_field_status(&mut self, pos: (usize, usize)) -> FieldType {
        *self
            .inner_map
            .get_mut(pos.0)
            .unwrap()
            .get_mut(pos.1)
            .unwrap()
    }
}

pub fn run(contents: String) {
    let mut map = Map::from(contents);
    println!("Task1: {}", map.run_task1());
    println!("Task2: {}", map.run_task2());
}
