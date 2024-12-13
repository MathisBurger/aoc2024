/// Todays challange had too much math for my taste.
/// First I tried to bruteforce the whole solution but then task2 would ran for always.
/// Therefore, I did some basic math and found some equations I could use to calculate the amount of required
/// button presses with min-max

struct SlotMaschine {
    pub button_a: (f64, f64),
    pub button_b: (f64, f64),
    pub price: (f64, f64),
}

struct MaschineCollection {
    pub maschines: Vec<SlotMaschine>,
}

impl From<String> for MaschineCollection {
    fn from(value: String) -> Self {
        let maschine_strings: Vec<&str> = value.split("\n\n").collect();
        let maschines: Vec<SlotMaschine> = maschine_strings
            .iter()
            .map(|s| {
                let lines = s.split("\n").filter(|l| !l.is_empty());
                let numbers: Vec<Vec<&str>> = lines
                    .map(|l| l.split(": ").nth(1).unwrap().split(", ").collect())
                    .collect();
                let button_a: Vec<f64> = numbers
                    .get(0)
                    .unwrap()
                    .iter()
                    .map(|num| num.split("+").nth(1).unwrap().parse().unwrap())
                    .collect();
                let button_b: Vec<f64> = numbers
                    .get(1)
                    .unwrap()
                    .iter()
                    .map(|num| num.split("+").nth(1).unwrap().parse().unwrap())
                    .collect();
                let price: Vec<f64> = numbers
                    .get(2)
                    .unwrap()
                    .iter()
                    .map(|num| num.split("=").nth(1).unwrap().parse().unwrap())
                    .collect();
                SlotMaschine {
                    button_a: (*button_a.get(0).unwrap(), *button_a.get(1).unwrap()),
                    button_b: (*button_b.get(0).unwrap(), *button_b.get(1).unwrap()),
                    price: (*price.get(0).unwrap(), *price.get(1).unwrap()),
                }
            })
            .collect();
        MaschineCollection { maschines }
    }
}

impl SlotMaschine {
    /// Gives option how many times I have to press A and B
    pub fn get_lowest_price(&self) -> Option<f64> {
        // px = ca * ax + cb * bx
        // py = ca * ay + cb * by
        // c = ca * 3 + cb
        //
        // Lets build an matrix equation
        // |ax, bx| * |ca| = |px|
        // |ay, by|   |cb|   |py|
        //
        // |ca| = |ax, bx|^-1  * |px| = A
        // |cb|   |ay, by|       |py|
        //
        // Calculate inverse matrix A^-1
        // Solve equation
        //
        // ca = (by*px - bx*py) / (ax * by - bx * ay)
        // cb = (−ay*px+ax*py) / (ax*by - bx * ay)
        //
        // c = 3*ca + cb
        //
        // After some optimzation this results in following code implementation:

        if self.button_a.0 == 0.0
            || self.button_a.1 == 0.0
            || self.button_b.0 == 0.0
            || self.button_b.1 == 0.0
        {
            return None;
        }

        let button_b_presses = ((self.button_a.1 * self.price.0)
            - (self.button_a.0 * self.price.1))
            / ((self.button_a.1 * self.button_b.0) - (self.button_a.0 * self.button_b.1));
        let button_a_presses =
            (self.price.0 - (self.button_b.0 * button_b_presses)) / self.button_a.0;
        if button_b_presses % 1.0 != 0.0 || button_a_presses % 1.0 != 0.0 {
            return None;
        }
        return Some(button_b_presses + 3.0 * button_a_presses);
    }
}

impl MaschineCollection {
    pub fn correct_values(&mut self) {
        for maschine in self.maschines.iter_mut() {
            maschine.price.0 += 10000000000000.0;
            maschine.price.1 += 10000000000000.0;
        }
    }

    pub fn total_cost(&self) -> f64 {
        let mut cost = 0.0;
        for maschine in &self.maschines {
            if let Some(m_cost) = maschine.get_lowest_price() {
                cost += m_cost;
            }
        }
        cost
    }
}

pub fn run(contents: String) {
    let mut collection = MaschineCollection::from(contents);
    println!("Task 1: {}", collection.total_cost());
    collection.correct_values();
    println!("Task 2: {}", collection.total_cost());
}
