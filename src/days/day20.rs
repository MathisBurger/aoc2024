use std::collections::{HashMap, HashSet, VecDeque};

type Shortcuts = HashSet<((usize, usize), (usize, usize), usize)>;

struct RaceTrack {
    matrix: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    path: Vec<(usize, usize)>,
    // Map location on track -> distance driven, distance left
    track_stats: HashMap<(usize, usize), (usize, usize)>,
    shortcuts: Shortcuts,
}

impl From<String> for RaceTrack {
    fn from(value: String) -> Self {
        let lines: Vec<&str> = value.split("\n").filter(|l| !l.is_empty()).collect();
        let mut matrix = vec![];
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (x, line) in lines.iter().enumerate() {
            let chars: Vec<char> = line
                .chars()
                .enumerate()
                .map(|(y, c)| {
                    if c == 'S' {
                        start = (x, y);
                    } else if c == 'E' {
                        end = (x, y);
                    }
                    c
                })
                .collect();
            matrix.push(chars);
        }
        RaceTrack {
            matrix,
            start,
            end,
            path: vec![],
            track_stats: HashMap::new(),
            shortcuts: HashSet::new(),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Cheat {
    s: (usize, usize), // start coordinate
    e: (usize, usize), // end coordinate
}

impl RaceTrack {
    pub fn task1(&mut self) -> usize {
        self.collect_track();
        self.initialize_distances();
        self.get_shortcuts(2);
        let results: Shortcuts = self
            .shortcuts
            .iter()
            .filter(|s| s.2 >= 0)
            .map(|s| *s)
            .collect();
        results.len()
    }

    /*pub fn task2(&mut self) -> usize {
        let mut valid_cheats = 0;
        for pos in self.path.clone() {
            let val1 = self.track_stats.get(&pos).unwrap().0 as isize;
            for pos2 in self.path.clone() {
                let val2 = self.track_stats.get(&pos2).unwrap().0 as isize;
                let distance = Self::manhattan_distance(pos, pos2) as isize;
                if distance <= 20 && val2 - val1 - distance >= 50 {
                    valid_cheats += 1;
                }
            }
        }
        valid_cheats
    }*/

    fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
        (p1.0 as isize - p2.0 as isize).abs() as usize
            + (p1.1 as isize - p2.1 as isize).abs() as usize
    }

    pub fn task2(&self) -> usize {
        let mut unique_cheats = HashMap::new();
        let mut count = 0;

        // Iterate over all pairs of points in the path
        for (&p1, &d1) in self.track_stats.iter() {
            for (&p2, &d2) in self.track_stats.iter() {
                let distance = Self::manhattan_distance(p1, p2) as isize;
                if (d2.0 as isize - d1.0 as isize) - distance >= 100 && distance <= 20 {
                    let cheat = Cheat { s: p1, e: p2 };
                    unique_cheats.insert(cheat, (d2.0 - d1.0) as isize);
                    count += 1;
                }
            }
        }
        count
    }

    fn get_shortcuts(&mut self, radius: usize) {
        for pos in self.path.clone() {
            let from_field = self.track_stats.get(&pos).unwrap();
            let reachable = self.get_reachable_fields(pos, radius);
            for field in reachable {
                let to_field = self.track_stats.get(&field.0).unwrap();
                if to_field.1 < from_field.1 {
                    let adv = from_field.1 - to_field.1;
                    if adv > field.1 {
                        self.shortcuts.insert((pos, field.0, adv - field.1));
                    }
                }
            }
        }
    }

    fn collect_track(&mut self) {
        let mut current = self.start;
        while *self.get_field_at(current).unwrap() != 'E' {
            if let Some(_) = self.get_track_at((current.0, current.1 + 1)) {
                self.path.push(current);
                current = (current.0, current.1 + 1);
            } else if let Some(_) = self.get_track_at((current.0, current.1 - 1)) {
                self.path.push(current);
                current = (current.0, current.1 - 1);
            } else if let Some(_) = self.get_track_at((current.0 + 1, current.1)) {
                self.path.push(current);
                current = (current.0 + 1, current.1);
            } else if let Some(_) = self.get_track_at((current.0 - 1, current.1)) {
                self.path.push(current);
                current = (current.0 - 1, current.1);
            } else {
                break;
            }
        }
    }

    fn initialize_distances(&mut self) {
        for i in 0..self.path.len() {
            let pos = self.path.get(i).unwrap();
            self.track_stats.insert(*pos, (i, self.path.len() - i - 1));
        }
        self.track_stats.insert(self.end, (self.path.len(), 0));
    }

    fn get_reachable_fields(
        &self,
        pos: (usize, usize),
        radius: usize,
    ) -> Vec<((usize, usize), usize)> {
        let rows = self.matrix.len();
        let cols = self.matrix[0].len();

        let directions = vec![
            (-1, 0), // Up
            (1, 0),  // Down
            (0, -1), // Left
            (0, 1),  // Right
        ];

        // BFS setup
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; cols]; rows];

        queue.push_back((pos, 0));
        visited[pos.0][pos.1] = true;

        let mut reachable_fields = Vec::new();

        while let Some(((x, y), distance)) = queue.pop_front() {
            // Explore the 4 possible directions
            if distance < radius {
                for (dx, dy) in &directions {
                    let new_x = x as isize + *dx;
                    let new_y = y as isize + *dy;

                    if new_x >= 0 && new_x < rows as isize && new_y >= 0 && new_y < cols as isize {
                        let new_x = new_x as usize;
                        let new_y = new_y as usize;

                        if !visited[new_x][new_y] && self.matrix[new_x][new_y] == '#' {
                            visited[new_x][new_y] = true;
                            queue.push_back(((new_x, new_y), distance + 1));
                        } else if (self.matrix[new_x][new_y] == '.'
                            || self.matrix[new_x][new_y] == 'E')
                            && distance > 0
                        {
                            reachable_fields.push(((new_x, new_y), distance + 1));
                        }
                    }
                }
            }
        }
        reachable_fields
    }

    fn get_track_at(&self, pos: (usize, usize)) -> Option<&char> {
        // Already visited spots of the path should not be driven twice
        if self.path.contains(&pos) {
            return None;
        }
        if let Some(field) = self.get_field_at(pos) {
            if *field != '#' {
                return Some(field);
            }
            return None;
        }
        None
    }

    fn get_field_at(&self, pos: (usize, usize)) -> Option<&char> {
        if pos.0 >= self.matrix.len() {
            return None;
        }
        self.matrix.get(pos.0).unwrap().get(pos.1)
    }
}

pub fn run(contents: String) {
    let mut track = RaceTrack::from(contents);
    println!("Task 1: {}", track.task1());
    println!("Task 2: {}", track.task2());
}
