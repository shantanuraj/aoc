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

    println!("\nPart 2");
    println!(
        "input_01.txt -- {}",
        solve_part_two(include_str!("../input_01.txt"))
    );
    println!(
        "input_02.txt -- {}",
        solve_part_two(include_str!("../input_02.txt"))
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

enum Direction {
    Increasing,
    Decreasing,
}

// Same as validate_level but with tolerance.
// Tolerates at most one faulty value.
// The level should be valid if the faulty value is removed.
// If the level is valid without the faulty value, the level is considered valid.
// We use backtracking to determine if the level is valid with tolerance.
// On encountering a faulty value, we remove it and check if the level is valid.
// We keep track of the number of faulty values encountered.
// At most we have to go back one number to check if the level is valid.
fn validate_level_with_tolerance(row: &str, tolerance: i32) -> bool {
    let levels: Vec<i32> = row
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let (valid, _) = validate_sequence(&levels, tolerance, None);

    valid
}

fn validate_sequence(
    levels: &[i32],
    tolerance: i32,
    direction: Option<Direction>,
) -> (bool, Option<Direction>) {
    if levels.len() < 3 {
        return (true, direction);
    }

    // Pick first three numbers to determine the direction.
    let n1 = levels[0];
    let n2 = levels[1];
    let n3 = levels[2];

    let follows_direction = match direction {
        Some(Direction::Increasing) => n1 < n2 && n2 < n3,
        Some(Direction::Decreasing) => n1 > n2 && n2 > n3,
        None => n1 < n2 && n2 < n3 || n1 > n2 && n2 > n3,
    };

    let valid_diff = {
        let diff1 = (n2 - n1).abs();
        let diff2 = (n3 - n2).abs();
        diff1 >= 1 && diff1 <= 3 && diff2 >= 1 && diff2 <= 3
    };

    if follows_direction && valid_diff {
        let direction = direction.or_else(|| {
            if n1 < n2 && n2 < n3 {
                Some(Direction::Increasing)
            } else {
                Some(Direction::Decreasing)
            }
        });

        return validate_sequence(&levels[1..], tolerance, direction);
    }

    // We descend into three possible sub-sequences
    // 1. Remove the first number
    // 2. Remove the second number
    // 3. Remove the third number
    // We deduct the tolerance by one for each sub-sequence.
    // If the tolerance is zero, we cannot descend further.

    if tolerance == 0 {
        return (false, None);
    }
    let tolerance = tolerance - 1;

    let levels = levels.to_vec();

    let mut sub_levels = levels.clone();
    sub_levels.remove(0); // Remove the first number
    let (valid, direction) = validate_sequence(&sub_levels, tolerance, direction);
    if valid {
        return (true, direction);
    }

    let mut sub_levels = levels.clone();
    sub_levels.remove(1); // Remove the second number
    let (valid, direction) = validate_sequence(&sub_levels, tolerance, direction);
    if valid {
        return (true, direction);
    }

    let mut sub_levels = levels.clone();
    sub_levels.remove(2); // Remove the third number
    let (valid, direction) = validate_sequence(&sub_levels, tolerance, direction);
    if valid {
        return (true, direction);
    }

    (false, None)
}

fn solve_part_one(input: &str) -> i32 {
    input.lines().filter(|level| validate_level(level)).count() as i32
}

fn solve_part_two(input: &str) -> i32 {
    input
        .lines()
        .filter(|level| validate_level_with_tolerance(level, 1))
        .count() as i32
}
