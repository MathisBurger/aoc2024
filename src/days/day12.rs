/// NOTE: Todays code is really shitty. I had not enough time today to optimize my algorithm.
/// Therefore it is pretty slow and inefficient.
use std::collections::HashMap;

#[derive(Clone)]
struct Area {
    fields: Vec<(usize, usize)>,
    letter: char,
    sides: u64,
}

#[derive(PartialEq, Clone, Debug)]
enum SideHand {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Debug)]
struct Side {
    content: Vec<(usize, usize)>,
    hand: SideHand,
}

struct Matrix {
    fields: Vec<Vec<char>>,
    used_fields: HashMap<(usize, usize), bool>,
    areas: Vec<Area>,
}

impl PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        if self.hand != other.hand {
            return false;
        }

        for element in self.content.clone() {
            if !other.content.contains(&element) {
                return false;
            }
        }
        true
    }
}

impl From<String> for Matrix {
    fn from(value: String) -> Self {
        let lines: Vec<&str> = value.split("\n").filter(|l| !l.is_empty()).collect();
        let mut fields: Vec<Vec<char>> = vec![];
        for line in lines {
            fields.push(line.chars().collect());
        }
        Matrix {
            fields,
            used_fields: HashMap::new(),
            areas: vec![],
        }
    }
}

impl Matrix {
    pub fn task1(&mut self) -> usize {
        self.find_areas_in_matrix();
        let mut fence_cost: usize = 0;
        for area in self.areas.clone() {
            fence_cost += area.fields.len() * area.sides as usize;
        }
        fence_cost
    }

    pub fn task2(&mut self) -> usize {
        let mut costs: usize = 0;

        for area in self.areas.clone() {
            let side_fields = Self::find_side_fields(&area);
            let side_count = Self::get_side_count(&side_fields, &area.fields);
            costs += side_count * area.fields.len();
        }

        costs
    }

    fn get_side_count(fields: &Vec<(usize, usize)>, full_area: &Vec<(usize, usize)>) -> usize {
        if fields.len() == 1 {
            return 4;
        }
        let mut sides: Vec<Side> = vec![];
        for field in fields {
            let directions = Self::find_directions(field, full_area);
            for direction in directions {
                sides.push(Side {
                    content: vec![field.clone()],
                    hand: direction,
                });
            }
        }

        let mut size_before = usize::MAX;

        while size_before != sides.len() {
            size_before = sides.len();
            for (i, side) in sides.clone().iter().enumerate() {
                if side.hand == SideHand::Left || side.hand == SideHand::Right {
                    let comp = Self::find_hl_comp_x(&side);
                    let upper = sides.iter_mut().enumerate().find(|(index, s)| {
                        s.hand == side.hand && *index != i && s.content.contains(&comp[0])
                    });
                    if let Some(upper_unwrapped) = upper {
                        upper_unwrapped.1.content.extend(side.content.clone());
                        sides.remove(i);
                        break;
                    }

                    let lower = sides.iter_mut().enumerate().find(|(index, s)| {
                        s.hand == side.hand && s.content.contains(&comp[1]) && *index != i
                    });
                    if let Some(lower_unwrapped) = lower {
                        lower_unwrapped.1.content.extend(side.content.clone());
                        sides.remove(i);
                        break;
                    }
                }

                if side.hand == SideHand::Top || side.hand == SideHand::Bottom {
                    let comp = Self::find_hl_comp_y(&side);
                    let upper = sides.iter_mut().enumerate().find(|(index, s)| {
                        s.hand == side.hand && *index != i && s.content.contains(&comp[0])
                    });
                    if let Some(upper_unwrapped) = upper {
                        upper_unwrapped.1.content.extend(side.content.clone());
                        sides.remove(i);
                        break;
                    }

                    let lower = sides.iter_mut().enumerate().find(|(index, s)| {
                        s.hand == side.hand && *index != i && s.content.contains(&comp[1])
                    });
                    if let Some(lower_unwrapped) = lower {
                        lower_unwrapped.1.content.extend(side.content.clone());
                        sides.remove(i);
                        break;
                    }
                }
            }
        }

        sides.len()
    }

    fn find_hl_comp_x(side: &Side) -> [(usize, usize); 2] {
        let mut min = side.content[0].clone();
        let mut max = side.content[0].clone();

        for &item in side.content.iter() {
            if item.0 < min.0 {
                min = item.clone();
            }
            if item.0 > max.0 {
                max = item.clone();
            }
        }
        if min.0 > 0 {
            return [(min.0 - 1, min.1), (max.0 + 1, max.1)];
        }
        [(min.0, min.1), (max.0 + 1, max.1)]
    }

    fn find_hl_comp_y(side: &Side) -> [(usize, usize); 2] {
        let mut min = side.content[0].clone();
        let mut max = side.content[0].clone();

        for &item in side.content.iter() {
            if item.1 < min.1 {
                min = item.clone();
            }
            if item.1 > max.1 {
                max = item.clone();
            }
        }
        if min.1 > 0 {
            return [(min.0, min.1 - 1), (max.0, max.1 + 1)];
        }
        [(min.0, min.1), (max.0, max.1 + 1)]
    }

    fn find_directions(pos: &(usize, usize), full_area: &Vec<(usize, usize)>) -> Vec<SideHand> {
        let mut directions = Vec::new();
        if pos.0 == 0 {
            directions.push(SideHand::Top);
        }
        if pos.1 == 0 {
            directions.push(SideHand::Left);
        }
        if pos.0 != 0 && !full_area.contains(&(pos.0 - 1, pos.1)) {
            directions.push(SideHand::Top);
        }
        if !full_area.contains(&(pos.0 + 1, pos.1)) {
            directions.push(SideHand::Bottom);
        }
        if pos.1 != 0 && !full_area.contains(&(pos.0, pos.1 - 1)) {
            directions.push(SideHand::Left);
        }
        if !full_area.contains(&(pos.0, pos.1 + 1)) {
            directions.push(SideHand::Right);
        }
        directions
    }

    fn find_side_fields(area: &Area) -> Vec<(usize, usize)> {
        let mut side_fields = vec![];
        for field in area.fields.clone() {
            if field.0 == 0 || field.1 == 0 {
                side_fields.push(field);
                continue;
            }
            if !area.fields.contains(&(field.0 - 1, field.1)) {
                side_fields.push(field);
                continue;
            }
            if !area.fields.contains(&(field.0 + 1, field.1)) {
                side_fields.push(field);
                continue;
            }
            if !area.fields.contains(&(field.0, field.1 - 1)) {
                side_fields.push(field);
                continue;
            }
            if !area.fields.contains(&(field.0 - 1, field.1 + 1)) {
                side_fields.push(field);
                continue;
            }
        }
        side_fields
    }

    fn find_areas_in_matrix(&mut self) {
        for (x, row) in self.fields.clone().iter().enumerate() {
            for (y, element) in row.iter().enumerate() {
                let mut area: Vec<(usize, usize)> = vec![(x, y)];
                let mut fence_count: u64 = 0;
                self.find_from_spot((x, y), &mut area, &mut fence_count, *element);
                if fence_count > 0 {
                    self.areas.push(Area {
                        fields: area,
                        letter: *element,
                        sides: fence_count,
                    });
                }
            }
        }
    }

    fn find_from_spot(
        &mut self,
        start_spot: (usize, usize),
        area: &mut Vec<(usize, usize)>,
        fences: &mut u64,
        c: char,
    ) {
        // Already entered
        if self.used_fields.get(&start_spot).is_some() {
            return;
        }
        self.used_fields.insert(start_spot, true);

        // top
        if start_spot.0 > 0 {
            if self.get_char_at_pos(start_spot.0 - 1, start_spot.1) == c {
                let new_spot = (start_spot.0 - 1, start_spot.1);
                if !area.contains(&new_spot) {
                    area.push(new_spot);
                    self.find_from_spot(new_spot, area, fences, c);
                }
            } else {
                *fences += 1;
            }
        } else {
            *fences += 1;
        }

        // bottom
        if start_spot.0 < self.fields.len() - 1 {
            if self.get_char_at_pos(start_spot.0 + 1, start_spot.1) == c {
                let new_spot = (start_spot.0 + 1, start_spot.1);
                if !area.contains(&new_spot) {
                    area.push(new_spot);
                    self.find_from_spot(new_spot, area, fences, c);
                }
            } else {
                *fences += 1;
            }
        } else {
            *fences += 1;
        }

        // left
        if start_spot.1 > 0 {
            if self.get_char_at_pos(start_spot.0, start_spot.1 - 1) == c {
                let new_spot = (start_spot.0, start_spot.1 - 1);
                if !area.contains(&new_spot) {
                    area.push(new_spot);
                    self.find_from_spot(new_spot, area, fences, c);
                }
            } else {
                *fences += 1;
            }
        } else {
            *fences += 1;
        }

        // right
        if start_spot.1 < self.fields.get(0).unwrap().len() - 1 {
            if self.get_char_at_pos(start_spot.0, start_spot.1 + 1) == c {
                let new_spot = (start_spot.0, start_spot.1 + 1);
                if !area.contains(&new_spot) {
                    area.push(new_spot);
                    self.find_from_spot(new_spot, area, fences, c);
                }
            } else {
                *fences += 1;
            }
        } else {
            *fences += 1;
        }
    }

    fn get_char_at_pos(&self, x: usize, y: usize) -> char {
        *self.fields.get(x).unwrap().get(y).unwrap()
    }
}

pub fn run(contents: String) {
    let mut matrix = Matrix::from(contents);
    println!("Task 1: {}", matrix.task1());
    println!("Task 2: {}", matrix.task2());
}
