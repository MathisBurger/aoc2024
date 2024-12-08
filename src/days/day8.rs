#[derive(Clone, Copy)]
struct Field {
    pub antenna: Option<char>,
    pub is_antinode: bool,
}

struct Matrix {
    inner_matrix: Vec<Vec<Field>>,
    frequencies: Vec<char>,
}

fn get_antenna_status(c: char) -> Option<char> {
    if c == '.' {
        return None;
    }
    Some(c)
}

impl From<String> for Matrix {
    fn from(value: String) -> Self {
        let lines: Vec<&str> = value.split("\n").filter(|l| !l.is_empty()).collect();
        let mut matrix = vec![];
        let mut frequencies = vec![];
        for line in lines {
            let elements: Vec<&str> = line.split("").filter(|e| *e != "").collect();
            let mut field_elements: Vec<Field> = vec![];
            for element in elements {
                let chars: Vec<char> = element.chars().collect();
                let c = chars.get(0).unwrap();
                if *c != '.' && !frequencies.contains(c) {
                    frequencies.push(c.clone())
                }
                field_elements.push(Field {
                    is_antinode: false,
                    antenna: get_antenna_status(*c),
                })
            }
            matrix.push(field_elements);
        }
        Matrix {
            inner_matrix: matrix,
            frequencies,
        }
    }
}

impl Matrix {
    pub fn task1(&mut self) -> u32 {
        for frequency in self.frequencies.clone() {
            let locations = self.get_antenna_locations(frequency);
            self.set_antinode_locations(locations);
        }
        self.count_antinodes()
    }

    pub fn task2(&mut self) -> u32 {
        for frequency in self.frequencies.clone() {
            let locations = self.get_antenna_locations(frequency);
            self.set_antinode_locations_task2(locations);
        }
        self.count_antinodes()
    }

    fn count_antinodes(&mut self) -> u32 {
        let mut count: u32 = 0;
        for row in self.inner_matrix.clone() {
            for element in row {
                if element.is_antinode {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_antenna_locations(&mut self, frequency: char) -> Vec<(isize, isize)> {
        let mut locations = vec![];
        for x in 0..self.inner_matrix.len() {
            for (y, field) in self.inner_matrix.get(x).unwrap().iter().enumerate() {
                if let Some(antenna) = field.antenna {
                    if antenna == frequency {
                        locations.push((x as isize, y as isize));
                    }
                }
            }
        }
        locations
    }

    fn set_antinode_locations(&mut self, locations: Vec<(isize, isize)>) {
        for location in locations.clone() {
            let other_locations: Vec<(isize, isize)> = locations
                .iter()
                .filter(|loc| **loc != location)
                .map(|loc| loc.clone())
                .collect();
            for other_location in other_locations {
                let vector = (other_location.0 - location.0, other_location.1 - location.1);
                self.set_node(other_location.0 + vector.0, other_location.1 + vector.1);
                self.set_node(location.0 - vector.0, location.1 - vector.1);
            }
        }
    }

    fn set_antinode_locations_task2(&mut self, locations: Vec<(isize, isize)>) {
        for location in locations.clone() {
            let other_locations: Vec<(isize, isize)> = locations
                .iter()
                .filter(|loc| **loc != location)
                .map(|loc| loc.clone())
                .collect();
            self.set_node(location.0, location.1);
            for other_location in other_locations {
                let vector = (other_location.0 - location.0, other_location.1 - location.1);
                let mut vec_one = vector.clone();
                let mut vec_two = vector.clone();
                while self.set_node(location.0 + vec_one.0, location.1 + vec_one.1) {
                    vec_one = (vec_one.0 + vector.0, vec_one.1 + vector.1);
                }
                while self.set_node(location.0 - vec_two.0, location.1 - vec_two.1) {
                    vec_two = (vec_two.0 + vector.0, vec_two.1 + vector.1);
                }
            }
        }
    }

    fn set_node(&mut self, x: isize, y: isize) -> bool {
        if x < 0
            || y < 0
            || x >= self.inner_matrix.len() as isize
            || y >= self.inner_matrix.get(0).unwrap().len() as isize
        {
            return false;
        }
        let mut loc: &mut Field = self
            .inner_matrix
            .get_mut(x as usize)
            .unwrap()
            .get_mut(y as usize)
            .unwrap();
        loc.is_antinode = true;
        true
    }
}

pub fn run(contents: String) {
    let mut matrix = Matrix::from(contents);
    println!("Task1: {}", matrix.task1());
    println!("Task2: {}", matrix.task2());
}
