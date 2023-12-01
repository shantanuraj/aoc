fn main() {
    println!("input_01.txt -- {}", solve(include_str!("../input_01.txt"))); // Previous soluton: 142
    println!("input_02.txt -- {}", solve(include_str!("../input_02.txt"))); // Previous soluton: 54331
    println!("input_03.txt -- {}", solve(include_str!("../input_03.txt")));
}

const WORD_TO_NUM: [(&str, i32); 10] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("ten", 10),
];

fn solve(input: &str) -> i32 {
    let mut total_sum = 0;

    for line in input.lines() {
        let first_number = find_first_number(line).unwrap();
        let last_number = find_last_number(line).unwrap();

        let num = format!("{}{}", first_number, last_number)
            .parse::<i32>()
            .unwrap();

        total_sum += num;
    }

    total_sum
}

fn find_first_number(line: &str) -> Option<i32> {
    if line.is_empty() {
        return None;
    }
    let first_char = line.chars().next().unwrap();
    if first_char.is_numeric() {
        return Some(first_char.to_digit(10).unwrap() as i32);
    }
    for (word, num) in WORD_TO_NUM.iter() {
        if line.starts_with(word) {
            return Some(*num);
        }
    }
    return find_first_number(&line[1..]);
}

fn find_last_number(line: &str) -> Option<i32> {
    if line.is_empty() {
        return None;
    }
    let last_char = line.chars().last().unwrap();
    if last_char.is_numeric() {
        return Some(last_char.to_digit(10).unwrap() as i32);
    }
    for (word, num) in WORD_TO_NUM.iter() {
        if line.ends_with(word) {
            return Some(*num);
        }
    }
    return find_last_number(&line[..line.len() - 1]);
}
