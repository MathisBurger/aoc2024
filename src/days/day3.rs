pub fn run(content: String) {
    let one_line = content.replace("\n", "");

    let mul_indexes = get_mul_indexes(one_line.clone());

    println!("Part1: {}", get_sum(one_line.clone(), mul_indexes));

    let cond_indexes = get_conditional_indexes(one_line.clone());

    println!("Part2: {}", get_sum(one_line.clone(), cond_indexes));
}

fn get_sum(one_line: String, mul_indexes: Vec<usize>) -> u64 {
    let mut sum: u64 = 0;

    for index in mul_indexes {
        let result = lexer(&one_line, index);
        if let Some(additor) = result {
            sum += additor as u64;
        }
    }
    sum
}

fn get_conditional_indexes(content: String) -> Vec<usize> {
    let all_indexes = get_mul_indexes(content.clone());
    let dos: Vec<usize> = content
        .match_indices("do()")
        .map(|(index, _)| index + 4)
        .collect();
    let donts: Vec<usize> = content
        .match_indices("don't()")
        .map(|(index, _)| index + 4)
        .collect();

    let mut result: Vec<usize> = Vec::new();
    let mut enabled = true;

    let mut do_iter = dos.into_iter().peekable();
    let mut dont_iter = donts.into_iter().peekable();

    for index in all_indexes {
        // Check if we should toggle the enabled state based on "do"
        while let Some(&do_index) = do_iter.peek() {
            if index >= do_index {
                enabled = true;
                do_iter.next(); // Move to the next "do"
            } else {
                break;
            }
        }

        // Check if we should toggle the enabled state based on "dont"
        while let Some(&dont_index) = dont_iter.peek() {
            if index >= dont_index {
                enabled = false;
                dont_iter.next(); // Move to the next "dont"
            } else {
                break;
            }
        }

        // Add the index to the result if enabled
        if enabled {
            result.push(index);
        }
    }

    result
}

fn get_mul_indexes(content: String) -> Vec<usize> {
    content
        .match_indices("mul(")
        .map(|(index, _)| index + 4)
        .collect()
}

const VALID_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn lexer(content: &String, start_index: usize) -> Option<u32> {
    let mut current_index = start_index;

    let mul1_option = get_number(&mut current_index, content);
    if mul1_option.is_none() {
        return None;
    }
    let mul1 = mul1_option.unwrap();

    let c = content.chars().nth(current_index);
    if c.is_none() {
        return None;
    }
    let vc = c.unwrap();
    if vc != ',' {
        return None;
    }
    current_index += 1;
    let mul2_option = get_number(&mut current_index, content);
    if mul2_option.is_none() {
        return None;
    }
    let mul2 = mul2_option.unwrap();

    let c2 = content.chars().nth(current_index);
    if c2.is_none() {
        return None;
    }
    let vc2 = c2.unwrap();
    if vc2 != ')' {
        return None;
    }

    return Some(mul1 * mul2);
}

fn get_number(current_index: &mut usize, content: &String) -> Option<u32> {
    let mut mul_len = 0;
    let mut mul = String::new();
    loop {
        if mul_len > 3 {
            return None;
        }
        let c = content.chars().nth(*current_index);
        if c.is_none() {
            return None;
        }
        let vc = c.unwrap();
        if !VALID_CHARS.contains(&vc) {
            if (vc == ',' || vc == ')') && mul_len > 0 {
                break;
            }
            return None;
        }
        mul = format!("{}{}", mul, vc);
        mul_len += 1;
        *current_index += 1;
    }
    return Some(mul.parse::<u32>().unwrap());
}
