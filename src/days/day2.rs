#[derive(PartialEq, Clone, Copy)]
enum ChangeState {
    Increase,
    Decrease,
}

pub fn run(input: String) {
    let lines: Vec<String> = input
        .split("\n")
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();

    let reports: Vec<Vec<u8>> = lines
        .iter()
        .map(|l| {
            let values: Vec<u8> = l.split(" ").map(|v| v.parse::<u8>().unwrap()).collect();
            values
        })
        .collect();

    let mut valid_reports_count_task1 = 0;

    for report in &reports {
        if check_valid(report) {
            valid_reports_count_task1 += 1;
        }
    }

    println!("Valid reports: {}", valid_reports_count_task1);

    let mut valid_reports_count_task2 = 0;

    for dirty_report in reports {
        let report = eliminate_bad_value(&dirty_report);

        let status = get_trend(&report);
        let mut valid = true;
        for i in 1..report.len() {
            let diff = report.get(i - 1).unwrap().abs_diff(*report.get(i).unwrap());

            if diff < 1 || diff > 3 {
                valid = false;
                break;
            }

            if report.get(i - 1).unwrap() < report.get(i).unwrap() {
                if status == ChangeState::Decrease {
                    valid = false;
                    break;
                }
            } else {
                if status == ChangeState::Increase {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            valid_reports_count_task2 += 1;
        }
    }

    println!("Task2 valid reports: {}", valid_reports_count_task2);
}

fn check_valid(report: &Vec<u8>) -> bool {
    let status = get_trend(report);
    let mut valid = true;
    for i in 1..report.len() {
        let diff = report.get(i - 1).unwrap().abs_diff(*report.get(i).unwrap());

        if diff < 1 || diff > 3 {
            valid = false;
            break;
        }

        if report.get(i - 1).unwrap() < report.get(i).unwrap() {
            if status == ChangeState::Decrease {
                valid = false;
                break;
            }
        } else {
            if status == ChangeState::Increase {
                valid = false;
                break;
            }
        }
    }
    valid
}

fn get_trend(report: &Vec<u8>) -> ChangeState {
    let mut inc = 0;
    let mut dec = 0;

    for i in 1..report.len() {
        if report.get(i - 1).unwrap() < report.get(i).unwrap() {
            inc += 1;
        } else {
            dec += 1;
        }
    }

    if inc > dec {
        return ChangeState::Increase;
    }
    ChangeState::Decrease
}

fn eliminate_bad_value(report: &Vec<u8>) -> Vec<u8> {
    for i in 0..report.len() {
        let mut cloned = report.clone();
        cloned.remove(i);
        if check_valid(&cloned) {
            return cloned;
        }
    }
    return report.clone();
}
