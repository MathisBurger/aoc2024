/// Todays solution also has not the best code quality.
/// With task 1 it was pretty solid but task2 made things different.
/// If I had known the requirements before, I would have chosen another approach.
/// I could optimize the runtime and quality of this code, but to be honest I am curently
/// too lazy to to that :=)
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq)]
enum LocationState {
    Free,
    Box,
    LeftBox,
    RightBox,
    Wall,
}

struct RoboMap {
    matrix: Vec<Vec<LocationState>>,
    movements: Vec<char>,
    robo_pos: (usize, usize),
}

impl From<String> for RoboMap {
    fn from(value: String) -> Self {
        let lines: Vec<&str> = value.split("\n").collect();
        let map_lines: Vec<&&str> = lines.iter().filter(|l| l.contains("#")).collect();
        let movement_lines: Vec<&str> = lines
            .iter()
            .filter(|l| l.contains("^") || l.contains("v") || l.contains("<") || l.contains(">"))
            .map(|l| l.clone())
            .collect();

        let mut map_matrix: Vec<Vec<LocationState>> = vec![];
        let mut robo_pos = (0, 0);
        for (i, line) in map_lines.iter().enumerate() {
            map_matrix.push(
                line.chars()
                    .into_iter()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '#' => LocationState::Wall,
                        'O' => LocationState::Box,
                        '[' => LocationState::LeftBox,
                        ']' => LocationState::RightBox,
                        '@' => {
                            robo_pos = (i, j);
                            LocationState::Free
                        }
                        _ => LocationState::Free,
                    })
                    .collect(),
            );
        }

        let movements: Vec<char> = movement_lines.concat().chars().collect();

        RoboMap {
            matrix: map_matrix,
            movements,
            robo_pos,
        }
    }
}

impl Debug for RoboMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (x, row) in self.matrix.iter().enumerate() {
            for (y, element) in row.iter().enumerate() {
                if x == self.robo_pos.0 && y == self.robo_pos.1 {
                    write!(f, "@").unwrap();
                } else {
                    match element {
                        LocationState::Wall => write!(f, "#").unwrap(),
                        LocationState::Free => write!(f, ".").unwrap(),
                        LocationState::Box => write!(f, "O").unwrap(),
                        LocationState::LeftBox => write!(f, "[").unwrap(),
                        LocationState::RightBox => write!(f, "]").unwrap(),
                    };
                }
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}

impl RoboMap {
    pub fn tasks(&mut self) -> usize {
        self.simulate_runs();
        print!("{:?}", self);
        self.get_gps_sum()
    }

    fn get_gps_sum(&self) -> usize {
        let mut sum: usize = 0;
        for (i, row) in self.matrix.iter().enumerate() {
            for (j, element) in row.iter().enumerate() {
                if *element == LocationState::Box || *element == LocationState::LeftBox {
                    sum += i * 100 + j;
                }
            }
        }
        sum
    }

    fn simulate_runs(&mut self) {
        for movement in self.movements.clone() {
            if movement == '^' {
                if self.try_move(-1, 0, 1, None, true, true) {
                    self.try_move(-1, 0, 1, None, true, false);
                }
            } else if movement == 'v' {
                if self.try_move(1, 0, 1, None, true, true) {
                    self.try_move(1, 0, 1, None, true, false);
                }
            } else if movement == '<' {
                self.try_move(0, -1, 1, None, true, false);
            } else if movement == '>' {
                self.try_move(0, 1, 1, None, true, false);
            }
        }
    }

    fn try_move(
        &mut self,
        x: isize,
        y: isize,
        inc_count: isize,
        different_start_pos: Option<(isize, isize)>,
        recheck: bool,
        no_move: bool,
    ) -> bool {
        let target_field_pos = match different_start_pos {
            Some(diff) => (diff.0 + x * inc_count, diff.1 + y * inc_count),
            None => (
                self.robo_pos.0 as isize + x * inc_count,
                self.robo_pos.1 as isize + y * inc_count,
            ),
        };
        if let Some(target_field) = self.get_at_loc(target_field_pos) {
            if target_field == LocationState::Wall {
                return false;
            } else if target_field == LocationState::Box
                || (y != 0
                    && (target_field == LocationState::LeftBox
                        || target_field == LocationState::RightBox))
            {
                if self.try_move(x, y, inc_count + 1, None, true, false) {
                    let target = self
                        .get_at_loc_mut((target_field_pos.0 + x, target_field_pos.1 + y))
                        .unwrap();
                    *target = target_field;
                    let current = self.get_at_loc_mut(target_field_pos).unwrap();
                    *current = LocationState::Free;
                } else {
                    return false;
                }
            } else if target_field == LocationState::LeftBox && y == 0 {
                if recheck
                    && !self.try_move(
                        x,
                        y,
                        0,
                        Some((target_field_pos.0, target_field_pos.1 + 1)),
                        false,
                        no_move,
                    )
                {
                    return false;
                }

                if self.try_move(x, y, 1, Some(target_field_pos), true, no_move) {
                    if !no_move {
                        let target = self
                            .get_at_loc_mut((target_field_pos.0 + x, target_field_pos.1 + y))
                            .unwrap();
                        *target = LocationState::LeftBox;
                        let current = self.get_at_loc_mut(target_field_pos).unwrap();
                        *current = LocationState::Free;
                    }
                } else {
                    return false;
                }
            } else if target_field == LocationState::RightBox && y == 0 {
                if recheck
                    && !self.try_move(
                        x,
                        y,
                        0,
                        Some((target_field_pos.0, target_field_pos.1 - 1)),
                        false,
                        no_move,
                    )
                {
                    return false;
                }
                if self.try_move(x, y, 1, Some(target_field_pos), true, no_move) {
                    if !no_move {
                        let target = self
                            .get_at_loc_mut((target_field_pos.0 + x, target_field_pos.1 + y))
                            .unwrap();
                        *target = LocationState::RightBox;
                        let current = self.get_at_loc_mut(target_field_pos).unwrap();
                        *current = LocationState::Free;
                    }
                } else {
                    return false;
                }
            }
            if inc_count == 1 && different_start_pos.is_none() && !no_move {
                self.robo_pos = (target_field_pos.0 as usize, target_field_pos.1 as usize);
            }
            return true;
        }
        false
    }

    fn get_at_loc(&self, loc: (isize, isize)) -> Option<LocationState> {
        if loc.0 < 0
            || loc.1 < 0
            || loc.0 >= self.matrix.len() as isize
            || loc.1 >= self.matrix.get(0).unwrap().len() as isize
        {
            return None;
        }
        Some(
            *self
                .matrix
                .get(loc.0 as usize)
                .unwrap()
                .get(loc.1 as usize)
                .unwrap(),
        )
    }

    fn get_at_loc_mut(&mut self, loc: (isize, isize)) -> Option<&mut LocationState> {
        if loc.0 < 0
            || loc.1 < 0
            || loc.0 >= self.matrix.len() as isize
            || loc.1 >= self.matrix.get(0).unwrap().len() as isize
        {
            return None;
        }
        Some(
            self.matrix
                .get_mut(loc.0 as usize)
                .unwrap()
                .get_mut(loc.1 as usize)
                .unwrap(),
        )
    }
}

fn stretch(input: String) -> String {
    input
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.")
}

pub fn run(contents: String) {
    //let mut map = RoboMap::from(contents.clone());
    //println!("Task 1: {}", map.tasks());
    let mut map = RoboMap::from(stretch(contents));
    println!("Task 2: {}", map.tasks());
}
