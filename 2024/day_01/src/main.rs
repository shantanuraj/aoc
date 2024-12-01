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

fn read_sorted_number_pairs(content: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    for line in content.lines() {
        if let Some((first, second)) = line.split_whitespace().collect::<Vec<&str>>().split_first()
        {
            if let (Ok(num1), Some(Ok(num2))) = (
                first.parse::<i32>(),
                second.first().map(|s| s.parse::<i32>()),
            ) {
                first_numbers.push(num1);
                second_numbers.push(num2);
            }
        }
    }

    first_numbers.sort();
    second_numbers.sort();

    (first_numbers, second_numbers)
}

fn solve_part_one(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let (p1, p2) = read_sorted_number_pairs(input);
    for (_, (n1, n2)) in p1.iter().zip(p2.iter()).enumerate() {
        sum += (n1 - n2).abs();
    }
    sum
}

fn read_number_pairs(content: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    for line in content.lines() {
        if let Some((first, second)) = line.split_whitespace().collect::<Vec<&str>>().split_first()
        {
            if let (Ok(num1), Some(Ok(num2))) = (
                first.parse::<i32>(),
                second.first().map(|s| s.parse::<i32>()),
            ) {
                first_numbers.push(num1);
                second_numbers.push(num2);
            }
        }
    }

    (first_numbers, second_numbers)
}

fn solve_part_two(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let (p1, p2) = read_number_pairs(input);

    let mut freq_map = std::collections::HashMap::new();
    for n in p2.iter() {
        let count = freq_map.entry(n).or_insert(0);
        *count += 1;
    }

    for n in p1.iter() {
        sum += n * freq_map.get(&n).unwrap_or(&0);
    }
    sum
}
