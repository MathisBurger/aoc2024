pub struct Matrix {
    matrix: Vec<Vec<char>>,
    counter: i32,
    pub intent: usize,
}

pub struct Vector {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

impl Vector {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Self {
        Vector { from, to }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to)
            || (self.from == other.to && self.to == other.from)
    }

    fn ne(&self, other: &Self) -> bool {
        return self.eq(other);
    }
}

impl From<String> for Matrix {
    fn from(value: String) -> Self {
        let lines: Vec<&str> = value.split("\n").filter(|f| !f.is_empty()).collect();
        let mut mx: Vec<Vec<char>> = vec![];
        for line in lines {
            let row: Vec<char> = line.chars().collect();
            mx.push(row);
        }
        Matrix {
            matrix: mx,
            counter: 0,
            intent: 3,
        }
    }
}

impl Matrix {
    pub fn get_x_max(&self) -> usize {
        self.matrix.len()
    }

    pub fn get_y_max(&self) -> usize {
        self.matrix.get(0).unwrap().len()
    }

    pub fn get_vector_count(&self) -> i32 {
        self.counter
    }

    pub fn find_arround(&mut self, x: usize, y: usize) {
        if self.has_right(x, y)
            && self.find_vector_string(&Vector::new((x, y), (x, y + self.intent)))
                == "XMAS".to_string()
        {
            self.counter += 1;
        }

        if self.has_left(x, y)
            && self.find_vector_string(&Vector::new((x, y), (x, y - self.intent)))
                == "XMAS".to_string()
        {
            self.counter += 1;
        }

        if self.has_top(x, y)
            && self.find_vector_string(&Vector::new((x, y), (x - self.intent, y)))
                == "XMAS".to_string()
        {
            self.counter += 1;
        }

        if self.has_bottom(x, y)
            && self.find_vector_string(&Vector::new((x, y), (x + self.intent, y)))
                == "XMAS".to_string()
        {
            self.counter += 1;
        }

        if self.has_top(x, y)
            && self.has_left(x, y)
            && self.find_vector_string(&Vector::new((x, y), (x - self.intent, y - self.intent)))
                == "XMAS".to_string()
        {
            self.counter += 1;
        }

        if self.has_top(x, y)
            && self.has_right(x, y)
            && self.find_vector_string(&Vector::new((x, y), (x - self.intent, y + self.intent)))
                == "XMAS".to_string()
        {
            self.counter += 1;
        }

        if self.has_bottom(x, y)
            && self.has_left(x, y)
            && self.find_vector_string(&Vector::new((x, y), (x + self.intent, y - self.intent)))
                == "XMAS".to_string()
        {
            self.counter += 1;
        }

        if self.has_bottom(x, y)
            && self.has_right(x, y)
            && self.find_vector_string(&Vector::new((x, y), (x + self.intent, y + self.intent)))
                == "XMAS".to_string()
        {
            self.counter += 1;
        }
    }

    pub fn find_arround_2(&mut self, x: usize, y: usize) {
        // Check inner value for performance increase
        if self.find_value_at(x, y) != 'A' {
            return;
        }

        if self.has_top(x, y)
            && self.has_left(x, y)
            && self.has_bottom(x, y)
            && self.has_right(x, y)
        {
            let mut mas_count = 0;

            if self.find_vector_string(&Vector::new((x - 1, y - 1), (x + 1, y + 1))) == "MAS" {
                mas_count += 1;
            }
            if self.find_vector_string(&Vector::new((x + 1, y + 1), (x - 1, y - 1))) == "MAS" {
                mas_count += 1;
            }
            if self.find_vector_string(&Vector::new((x + 1, y - 1), (x - 1, y + 1))) == "MAS" {
                mas_count += 1;
            }
            if self.find_vector_string(&Vector::new((x - 1, y + 1), (x + 1, y - 1))) == "MAS" {
                mas_count += 1;
            }

            if mas_count == 2 {
                self.counter += 1;
            }
        }
    }

    pub fn find_vector_string(&self, v: &Vector) -> String {
        let x_mod = Self::get_x_mod(v);
        let y_mod = Self::get_y_mod(v);

        let mut x = v.from.0;
        let mut y = v.from.1;

        let mut word = String::new();

        while (x_mod == 0 || (x_mod < 0 && x >= v.to.0) || x_mod > 0 && x <= v.to.0)
            && (y_mod == 0 || (y_mod < 0 && y >= v.to.1) || y_mod > 0 && y <= v.to.1)
        {
            word = format!("{}{}", word, self.find_value_at(x, y));
            let x_local = x_mod + (x as isize);
            let y_local = y_mod + (y as isize);
            if x_local < 0 || y_local < 0 {
                break;
            }
            x = x_local as usize;
            y = y_local as usize;
        }

        word
    }

    fn find_value_at(&self, x: usize, y: usize) -> char {
        self.matrix.get(x).unwrap().get(y).unwrap().clone()
    }

    fn get_x_mod(v: &Vector) -> isize {
        if v.from.0 == v.to.0 {
            return 0;
        }
        if v.from.0 > v.to.0 {
            return -1;
        }
        return 1;
    }

    fn get_y_mod(v: &Vector) -> isize {
        if v.from.1 == v.to.1 {
            return 0;
        }
        if v.from.1 > v.to.1 {
            return -1;
        }
        return 1;
    }

    fn has_right(&self, x: usize, y: usize) -> bool {
        return self.matrix.get(x).unwrap().len() - 1 >= y + self.intent;
    }

    fn has_left(&self, x: usize, y: usize) -> bool {
        return y >= self.intent;
    }

    fn has_top(&self, x: usize, y: usize) -> bool {
        return x >= self.intent;
    }

    fn has_bottom(&self, x: usize, y: usize) -> bool {
        return self.matrix.len() - 1 >= x + self.intent;
    }
}

pub fn run(content: String) {
    let mut matrix = Matrix::from(content);
    for x in 0..matrix.get_x_max() {
        for y in 0..matrix.get_y_max() {
            matrix.find_arround(x, y);
        }
    }
    println!("Task 1: {}", matrix.get_vector_count());

    matrix.intent = 1;
    matrix.counter = 0;

    for x in 0..matrix.get_x_max() {
        for y in 0..matrix.get_y_max() {
            matrix.find_arround_2(x, y);
        }
    }
    println!("Task 2: {}", matrix.get_vector_count());
}
