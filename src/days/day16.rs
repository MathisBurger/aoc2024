use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct Maze {
    fields: Vec<Vec<char>>,
    start: (usize, usize),
}

#[derive(PartialEq, Eq)]
struct Item {
    pos: (usize, usize),
    dir: (isize, isize),
    score: u32,
    path: Vec<(usize, usize)>,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.score.cmp(&self.score))
    }
}

impl From<String> for Maze {
    fn from(value: String) -> Self {
        let lines: Vec<&str> = value.split("\n").filter(|l| !l.is_empty()).collect();
        let mut matrix: Vec<Vec<char>> = vec![];
        let mut start = (0, 0);
        for (x, line) in lines.iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            for (y, c) in chars.iter().enumerate() {
                if *c == 'S' {
                    start = (x, y);
                }
            }
            matrix.push(chars);
        }

        Maze {
            fields: matrix,
            start,
        }
    }
}

impl Maze {
    pub fn solve(&self) -> (u32, usize) {
        let mut queue = BinaryHeap::new();
        let mut visited = HashMap::new();

        queue.push(Item {
            pos: self.start,
            dir: (0, 1),
            score: 0,
            path: vec![self.start],
        });

        let mut best_path = None;
        let mut all_entries: HashSet<(usize, usize)> = HashSet::new();

        while let Some(Item {
            pos,
            dir,
            score,
            path,
        }) = queue.pop()
        {
            if let Some(&prev) = visited.get(&(pos, dir)) {
                if score > prev {
                    continue;
                }
            } else {
                visited.insert((pos, dir), score);
            }

            if self.get_value(pos) == 'E' {
                if let Some(real_best) = best_path {
                    if score == real_best {
                        all_entries.extend(path.iter());
                    }
                } else {
                    best_path = Some(score);
                    all_entries.extend(path.iter());
                }

                continue;
            }

            // Move foreword
            let next = Self::sum_pos_and_dir(pos, dir);
            let mut next_path = path.clone();
            next_path.push(next);
            if self.is_free(next) {
                queue.push(Item {
                    pos: next,
                    dir,
                    score: score + 1,
                    path: next_path,
                });
            }
            queue.push(Item {
                pos,
                dir: Self::get_next_direction(dir, -1),
                score: score + 1000,
                path: path.clone(),
            });
            queue.push(Item {
                pos,
                dir: Self::get_next_direction(dir, 1),
                score: score + 1000,
                path,
            });
        }
        return (best_path.unwrap(), all_entries.len());
    }

    fn is_free(&self, pos: (usize, usize)) -> bool {
        if pos.0 < 0
            || pos.1 < 0
            || pos.0 >= self.fields.len()
            || pos.1 >= self.fields.get(0).unwrap().len()
        {
            return false;
        }
        self.get_value(pos) != '#'
    }

    fn get_value(&self, pos: (usize, usize)) -> char {
        *self.fields.get(pos.0).unwrap().get(pos.1).unwrap()
    }

    fn sum_pos_and_dir(pos: (usize, usize), dir: (isize, isize)) -> (usize, usize) {
        (
            (pos.0 as isize + dir.0) as usize,
            (pos.1 as isize + dir.1) as usize,
        )
    }

    fn get_next_direction(current: (isize, isize), next: isize) -> (isize, isize) {
        let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let current_index = dirs
            .iter()
            .enumerate()
            .find(|(_, v)| **v == current)
            .unwrap();
        if current_index.0 as isize + next < 0 {
            return *dirs.get(3).unwrap();
        }
        if current_index.0 as isize + next > 3 {
            return *dirs.get(0).unwrap();
        }
        *dirs
            .get((current_index.0 as isize + next) as usize)
            .unwrap()
    }
}

pub fn run(contents: String) {
    let maze = Maze::from(contents);
    let (best, paths) = maze.solve();
    println!("Task 1: {}", best);
    println!("Task 2: {}", paths);
}
