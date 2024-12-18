use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::{BinaryHeap, HashMap};

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Node {
    x: usize,
    y: usize,
}

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const BYTE_COUNT: usize = 1024;
#[derive(Clone, PartialEq, Eq)]
struct AStarNode {
    node: Node,
    g: usize,
    h: usize,
}

impl AStarNode {
    fn f(&self) -> usize {
        self.g + self.h
    }
}

// Define a priority queue for A* (min-heap based on f = g + h)
impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f().cmp(&self.f()) // Reverse to make it a min-heap
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(start: Node, goal: Node, grid: &Vec<Vec<char>>) -> Option<Vec<Node>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_scores = HashMap::new();

    g_scores.insert(start, 0);

    let start_node = AStarNode {
        node: start,
        g: 0,
        h: heuristic(&start, &goal),
    };

    open_set.push(start_node);

    while let Some(current) = open_set.pop() {
        let current_node = current.node;

        // If we've reached the goal, reconstruct the path
        if current_node == goal {
            let mut path = Vec::new();
            let mut node = goal;
            while let Some(parent) = came_from.get(&node) {
                path.push(node);
                node = *parent;
            }
            path.push(start);
            path.reverse();
            return Some(path); // Shortest path
        }

        // Check the four possible directions
        for (dx, dy) in DIRECTIONS.iter() {
            let (nx, ny) = (current_node.x as isize + dx, current_node.y as isize + dy);

            if nx < 0 || ny < 0 || nx >= WIDTH as isize || ny >= HEIGHT as isize {
                continue; // Out of bounds
            }

            let (nx, ny) = (nx as usize, ny as usize);

            if grid[nx][ny] == '#' {
                continue; // Blocked cell
            }

            let tentative_g_score = g_scores[&current_node] + 1;
            let neighbor = Node { x: nx, y: ny };

            if !g_scores.contains_key(&neighbor) || tentative_g_score < g_scores[&neighbor] {
                g_scores.insert(neighbor, tentative_g_score);
                let neighbor_node = AStarNode {
                    node: neighbor,
                    g: tentative_g_score,
                    h: heuristic(&neighbor, &goal),
                };
                open_set.push(neighbor_node);
                came_from.insert(neighbor, current_node);
            }
        }
    }

    None
}

// Helper function to compute the Manhattan distance
fn heuristic(a: &Node, b: &Node) -> usize {
    (a.x as isize - b.x as isize).abs() as usize + (a.y as isize - b.y as isize).abs() as usize
}

fn parse_memory_until(mem: String, until: usize) -> (HashSet<(usize, usize)>, Vec<(usize, usize)>) {
    let lines: Vec<&str> = mem.split("\n").filter(|l| !l.is_empty()).collect();
    let mut set = HashSet::new();
    let mut vec = vec![];
    let mut i = 0;
    for line in lines {
        let nums: Vec<usize> = line
            .split(",")
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        if i >= until {
            vec.push((*nums.get(0).unwrap(), *nums.get(1).unwrap()));
        } else {
            set.insert((*nums.get(0).unwrap(), *nums.get(1).unwrap()));
        }
        i += 1;
    }
    (set, vec)
}

fn convert_to_grid(dump: HashSet<(usize, usize)>) -> Vec<Vec<char>> {
    let mut grid = vec![];
    for x in 0..HEIGHT {
        let mut row = vec![];
        for y in 0..WIDTH {
            if dump.get(&(x, y)).is_some() {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        grid.push(row);
    }
    grid
}

fn add_obstacle_to_grid(grid: &mut Vec<Vec<char>>, x: usize, y: usize) {
    let val = grid.get_mut(x).unwrap().get_mut(y).unwrap();
    *val = '#';
}

pub fn run(contents: String) {
    let memory_dump = parse_memory_until(contents, BYTE_COUNT);
    let mut grid = convert_to_grid(memory_dump.0);
    let start = Node { x: 0, y: 0 };
    let goal = Node {
        x: WIDTH - 1,
        y: HEIGHT - 1,
    };
    // Remove one, because it is start field
    println!("Task 1: {}", a_star(start, goal, &grid).unwrap().len() - 1);

    let mut byte_count: usize = 0;
    while let Some(_) = a_star(start, goal, &grid) {
        let obstacle = memory_dump.1.get(byte_count).unwrap();
        add_obstacle_to_grid(&mut grid, obstacle.0, obstacle.1);
        byte_count += 1;
    }
    println!("Task 2: {:?}", memory_dump.1.get(byte_count - 1).unwrap())
}
