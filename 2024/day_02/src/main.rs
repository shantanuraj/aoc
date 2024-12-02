fn main() {
    println!("\nPart 1");
    println!(
        "input_01.txt -- {}",
        solve_part_one(include_str!("../input_01.txt"))
    );
    println!(
        "input_02.txt -- {}",
        solve_part_one(include_str!("../input_02.txt"))
    );
}

// Expected structure of input:
// n1 n2 [n3 n4 ...]
//
// At least two numbers per line followed by arbitrary number of numbers
// separated by whitespace.
// The pair determines the direction of level either increasing or decreasing.
// If either of the subsequent numbers are not in the same direction, the level
// is considered invalid.
// If the difference between adjacent numbers is less than 1 or greater than 3,
// the level is considered invalid.
// This funciton parses the input row and determines if the level is valid.
fn validate_level(level: &str) -> bool {
    let mut iter = level.split_whitespace();

    let first = iter.next().unwrap().parse::<i32>().unwrap();
    let second = iter.next().unwrap().parse::<i32>().unwrap();

    let is_increasing = first < second;
    let diff = (second - first).abs();
    if diff < 1 || diff > 3 {
        return false;
    }

    let mut prev = second;

    for num in iter {
        let current = num.parse::<i32>().unwrap();

        if (prev < current) != is_increasing {
            return false;
        }

        let diff = (current - prev).abs();
        if diff < 1 || diff > 3 {
            return false;
        }

        prev = current;
    }

    true
}

fn solve_part_one(input: &str) -> i32 {
    input.lines().filter(|level| validate_level(level)).count() as i32
}

