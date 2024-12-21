/// Todays puzzle was a complete mess. To be honest I an wondering how it actally works.
/// I just noticed after a few hours that I should have chosen a complete different approach to actually solve this challange efficiently.
/// I am also aware of the fact, that this solution might not work for every puzzle input, but it somehow did for mine.
/// My code for task1 was way to inefficient for task2, therefore I went with a new a new path and got some inspriration from reddit to solve
/// todays puzzle, because it was also pretty late.
/// If the solution in here does not work properly, I might have somehow found another workarround that is not in git.
///
/// NOTICE: This solution here does NOT work. I found an workarround with python that was a little easier.
use std::cmp::min;
use std::collections::HashMap;
use std::iter::{self, repeat};
use std::vec::Vec;
use std::{iter::once, sync::LazyLock};

type Position = (i8, i8);

fn sub(pos1: Position, pos2: Position) -> Position {
    return (pos1.0 - pos2.0, pos1.1 - pos2.1);
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum NumericKeypad {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    ABtn,
}

impl From<char> for NumericKeypad {
    fn from(value: char) -> Self {
        match value {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'A' => Self::ABtn,
            _ => Self::ABtn,
        }
    }
}

impl NumericKeypad {
    fn to_pos(&self) -> Position {
        match self {
            Self::Seven => (0, 0),
            Self::Eight => (0, 1),
            Self::Nine => (0, 2),
            Self::Four => (1, 0),
            Self::Five => (1, 1),
            Self::Six => (1, 2),
            Self::One => (2, 0),
            Self::Two => (2, 1),
            Self::Three => (2, 2),
            Self::Zero => (3, 1),
            Self::ABtn => (3, 2),
        }
    }

    fn translate_directional(
        &self,
        to: NumericKeypad,
        approach_one: bool,
    ) -> Vec<DirectionalKeypad> {
        let mut seq = vec![];
        let diff = sub(self.to_pos(), to.to_pos());

        if ((*self == Self::Seven || *self == Self::Four || *self == Self::One)
            && (to == Self::Zero || to == Self::ABtn))
            || approach_one
        {
            if diff.1 < 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Right).take(diff.1.unsigned_abs() as usize),
                );
            } else if diff.1 > 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Left).take(diff.1.unsigned_abs() as usize),
                );
            }
            if diff.0 < 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Down).take(diff.0.unsigned_abs() as usize),
                );
            } else if diff.0 > 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Up).take(diff.0.unsigned_abs() as usize),
                );
            }
        } else if ((*self == Self::Zero || *self == Self::ABtn)
            && (to == Self::Seven || to == Self::Four || to == Self::One))
            || !approach_one
        {
            if diff.0 < 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Down).take(diff.0.unsigned_abs() as usize),
                );
            } else if diff.0 > 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Up).take(diff.0.unsigned_abs() as usize),
                );
            }
            if diff.1 < 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Right).take(diff.1.unsigned_abs() as usize),
                );
            } else if diff.1 > 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Left).take(diff.1.unsigned_abs() as usize),
                );
            }
        }

        seq.push(DirectionalKeypad::ABtn);
        seq
    }

    pub fn to_directional(from: &Vec<NumericKeypad>, approach_one: bool) -> Vec<DirectionalKeypad> {
        let mut seq = vec![];
        let mut current = NumericKeypad::ABtn;
        for field in from {
            seq.extend(current.translate_directional(*field, approach_one));

            current = *field;
        }
        seq
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum DirectionalKeypad {
    Up,
    Down,
    Left,
    Right,
    ABtn,
}

impl From<char> for DirectionalKeypad {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '<' => Self::Left,
            '>' => Self::Right,
            'v' => Self::Down,
            'A' => Self::ABtn,
            _ => Self::ABtn,
        }
    }
}

impl DirectionalKeypad {
    fn to_pos(&self) -> Position {
        match self {
            Self::Up => (0, 1),
            Self::ABtn => (0, 2),
            Self::Left => (1, 0),
            Self::Down => (1, 1),
            Self::Right => (1, 2),
        }
    }

    fn translate_directional(
        &mut self,
        to: DirectionalKeypad,
        approach_one: bool,
    ) -> Vec<DirectionalKeypad> {
        let mut seq = vec![];

        let diff = sub(self.to_pos(), to.to_pos());

        if *self == Self::Left || to == Self::Left || approach_one {
            if diff.1 < 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Right).take(diff.1.unsigned_abs() as usize),
                );
            } else if diff.1 > 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Left).take(diff.1.unsigned_abs() as usize),
                );
            }
            if diff.0 < 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Down).take(diff.0.unsigned_abs() as usize),
                );
            } else if diff.0 > 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Up).take(diff.0.unsigned_abs() as usize),
                );
            }
        } else if *self != Self::Left && to != Self::Left && !approach_one {
            if diff.0 < 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Down).take(diff.0.unsigned_abs() as usize),
                );
            } else if diff.0 > 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Up).take(diff.0.unsigned_abs() as usize),
                );
            }
            if diff.1 < 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Right).take(diff.1.unsigned_abs() as usize),
                );
            } else if diff.1 > 0 {
                seq.extend(
                    iter::repeat(DirectionalKeypad::Left).take(diff.1.unsigned_abs() as usize),
                );
            }
        }

        seq.push(DirectionalKeypad::ABtn);
        seq
    }

    pub fn to_directional(
        from: &Vec<DirectionalKeypad>,
        approach_one: bool,
    ) -> Vec<DirectionalKeypad> {
        let mut seq = vec![];
        let mut current = DirectionalKeypad::ABtn;
        for field in from {
            seq.extend(current.translate_directional(*field, approach_one));

            current = *field;
        }
        seq
    }
}

fn parse_codes(content: String) -> Vec<String> {
    content
        .split("\n")
        .filter(|c| !c.is_empty())
        .map(|c| c.to_string())
        .collect()
}

fn conv_code(code: String, approach_one: bool, robots: usize) -> Vec<DirectionalKeypad> {
    let code: Vec<NumericKeypad> = code.chars().map(|c| NumericKeypad::from(c)).collect();
    let mut dir = NumericKeypad::to_directional(&code, approach_one);
    for _ in 0..robots {
        let dir1 = DirectionalKeypad::to_directional(&dir, true);
        let dir2 = DirectionalKeypad::to_directional(&dir, false);
        if dir1.len() > dir2.len() {
            dir = dir2;
        } else {
            dir = dir1;
        }
    }
    dir
}

fn task1(codes: &Vec<String>) -> usize {
    let mut sum = 0;
    for raw_code in codes {
        let shortest1 = conv_code(raw_code.clone(), true, 2);
        let shortest2 = conv_code(raw_code.clone(), false, 2);

        let numeric = raw_code.replace("A", "").parse::<usize>().unwrap();
        sum += min(shortest1.len(), shortest2.len()) * numeric;
    }
    sum
}

use std::collections::{HashSet, VecDeque};
use std::f64::INFINITY;

// Direction map for movement
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];
const DIRECTION_STRINGS: [char; 4] = ['>', '^', '<', 'v'];

// Check if a position is valid in the matrix
fn valid(mat: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    x < mat.len() && y < mat[x].len() && mat[x][y] != '#'
}

// Precompute all possible paths between points
fn precalc(mat: &Vec<Vec<char>>) -> HashMap<(usize, usize, usize, usize), Vec<Vec<char>>> {
    let mut resp = HashMap::new();
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j] == '#' {
                continue;
            }
            for k in 0..mat.len() {
                for l in 0..mat[k].len() {
                    if k == i && l == j {
                        continue;
                    }
                    let paths = paths(mat, (i, j), (k, l));
                    resp.insert((i, j, k, l), paths);
                }
            }
        }
    }
    resp
}

// Find the shortest path between start and end
fn paths(mat: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Vec<Vec<char>> {
    let mut q = VecDeque::new();
    q.push_back((start.0, start.1, vec![start]));
    let mut result = Vec::new();
    let mut m = INFINITY;

    while let Some((x, y, path)) = q.pop_front() {
        if (x, y) == end && path.len() as f64 <= m {
            if (path.len() as f64) < m {
                result.clear();
            }
            m = path.len() as f64;
            result.push(path);
            continue;
        }

        for (dx, dy) in DIRECTIONS.iter() {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;

            if valid(mat, nx, ny) && !path.contains(&(nx, ny)) {
                let mut new_path = path.clone();
                new_path.push((nx, ny));
                q.push_back((nx, ny, new_path));
            }
        }
    }

    let mut final_paths = Vec::new();
    for r in result {
        let mut f = Vec::new();
        for i in 1..r.len() {
            let mov = (
                r[i].0 as i32 - r[i - 1].0 as i32,
                r[i].1 as i32 - r[i - 1].1 as i32,
            );
            if let Some(dir) = DIRECTIONS.iter().position(|&d| d == mov) {
                f.push(DIRECTION_STRINGS[dir]);
            }
        }
        f.push('A');
        final_paths.push(f);
    }

    final_paths
}

// Convert a pad to a coordinate mapping
fn to_coord(mat: &Vec<Vec<char>>) -> HashMap<char, (usize, usize)> {
    let mut result = HashMap::new();
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            result.insert(mat[i][j], (i, j));
        }
    }
    result
}

// Generic pad function for finding the shortest path from one point to another
fn generic_pad(
    c: &str,
    p2cord: &HashMap<char, (usize, usize)>,
    all_paths: &HashMap<(usize, usize, usize, usize), Vec<Vec<char>>>,
    mat: &Vec<Vec<char>>,
) -> Vec<String> {
    let mut current = p2cord[&'A'];
    let mut options = vec![];

    for i in c.chars() {
        let target = p2cord[&i];
        let paths = if current != target {
            all_paths[&(current.0, current.1, target.0, target.1)].clone()
        } else {
            vec![vec!['A']]
        };

        if options.is_empty() {
            options = paths.clone();
        } else {
            let mut new_options = Vec::new();
            for o in options {
                for p in paths.iter() {
                    let mut new_option = o.clone();
                    new_option.extend_from_slice(p);
                    new_options.push(new_option);
                }
            }
            options = new_options;
        }
        current = target;
    }

    let m = options.iter().map(|o| o.len()).min().unwrap();
    options
        .into_iter()
        .filter(|o| o.len() == m)
        .map(|o| o.iter().collect())
        .collect()
}

// Numpad path function
fn num_pad(
    c: &str,
    pad2cord: &HashMap<char, (usize, usize)>,
    pad_paths: &HashMap<(usize, usize, usize, usize), Vec<Vec<char>>>,
    pad: &Vec<Vec<char>>,
) -> Vec<String> {
    generic_pad(c, pad2cord, pad_paths, pad)
}

// Directional pad path function
fn dir_pad(
    c: &str,
    start: char,
    dpad2cord: &HashMap<char, (usize, usize)>,
    dpad_paths: &HashMap<(usize, usize, usize, usize), Vec<Vec<char>>>,
    dpad: &Vec<Vec<char>>,
) -> Vec<String> {
    let mut result = Vec::new();
    for x in c.chars() {
        result.extend(generic_pad(&x.to_string(), dpad2cord, dpad_paths, dpad));
    }
    let m = result.iter().map(|x| x.len()).min().unwrap();
    result.into_iter().filter(|x| x.len() == m).collect()
}

// Dynamic programming function for calculating the minimum distance
fn dp(
    c: &str,
    d: usize,
    dpad2cord: &HashMap<char, (usize, usize)>,
    dpad_paths: &HashMap<(usize, usize, usize, usize), Vec<Vec<char>>>,
) -> usize {
    let mut response = 0;
    let c = format!("A{}", c);
    for i in 1..c.len() {
        let paths = generic_pad(&c[i..i + 1], dpad2cord, dpad_paths, &vec![]);
        if d == 0 {
            response += paths.iter().map(|path| path.len()).min().unwrap();
        } else {
            response += paths
                .iter()
                .map(|path| dp(path, d - 1, dpad2cord, dpad_paths))
                .min()
                .unwrap();
        }
    }
    response
}

// Main solve function for the problem
fn solve(
    c: &str,
    pad2cord: &HashMap<char, (usize, usize)>,
    pad_paths: &HashMap<(usize, usize, usize, usize), Vec<Vec<char>>>,
    dpad2cord: &HashMap<char, (usize, usize)>,
    dpad_paths: &HashMap<(usize, usize, usize, usize), Vec<Vec<char>>>,
) -> usize {
    num_pad(c, pad2cord, pad_paths, &vec![]).len()
}

fn task2() -> usize {
    let pad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['#', '0', 'A'],
    ];
    let pad2cord = to_coord(&pad);
    let pad_paths = precalc(&pad);

    let dpad = vec![vec!['#', '^', 'A'], vec!['<', 'v']];
    let dpad2cord = to_coord(&dpad);
    let dpad_paths = precalc(&dpad);

    let l = vec!["140A", "169A", "170A", "528A", "340A"];
    let mut resp = 0;

    for c in l {
        let numeric = c.replace('A', "").parse::<usize>().unwrap();
        let size = solve(c, &pad2cord, &pad_paths, &dpad2cord, &dpad_paths);
        println!("{:?} {:?}", size, numeric);
        resp += size * numeric;
    }

    resp
}

pub fn run(contents: String) {
    let codes = parse_codes(contents.clone());
    println!("Task 1: {}", task1(&codes));
    println!("Task 2: {}", task2());
}
