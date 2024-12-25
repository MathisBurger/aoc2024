struct Schemantics {
    locks: Vec<[u8; 5]>,
    keys: Vec<[u8; 5]>,
}

impl From<String> for Schemantics {
    fn from(value: String) -> Self {
        let objects: Vec<&str> = value.split("\n\n").filter(|l| !l.is_empty()).collect();
        let mut locks = vec![];
        let mut keys = vec![];
        for object in objects {
            let rows: Vec<&str> = object.split("\n").filter(|l| !l.is_empty()).collect();
            if rows[0] == "#####" {
                let mut lock: [u8; 5] = [0; 5];
                for i in 0..5 {
                    let mut count = 0;
                    while rows[count].chars().nth(i).unwrap() == '#' {
                        count += 1;
                    }
                    if count > 0 {
                        lock[i] = (count - 1) as u8;
                    } else {
                        lock[i] = count as u8;
                    }
                }
                locks.push(lock);
            } else {
                let mut key: [u8; 5] = [0; 5];
                for i in 0..5 {
                    let mut count = 0;
                    while rows[6 - count].chars().nth(i).unwrap() == '#' {
                        count += 1;
                    }
                    if count > 0 {
                        key[i] = (count - 1) as u8;
                    } else {
                        key[i] = count as u8;
                    }
                }
                keys.push(key);
            }
        }

        Schemantics { locks, keys }
    }
}

impl Schemantics {
    pub fn task1(&self) -> u32 {
        self.get_matches()
    }

    fn get_matches(&self) -> u32 {
        let mut matches: u32 = 0;
        for lock in &self.locks {
            for key in &self.keys {
                if Self::fits(lock, key) {
                    matches += 1;
                }
            }
        }
        matches
    }

    fn fits(lock: &[u8; 5], key: &[u8; 5]) -> bool {
        for i in 0..5 {
            if (lock[i] + key[i]) > 5 {
                return false;
            }
        }
        true
    }
}

pub fn run(contents: String) {
    let schema = Schemantics::from(contents);
    println!("Task 1: {}", schema.task1());
}
