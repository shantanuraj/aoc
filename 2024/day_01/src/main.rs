use std::collections::HashMap;

type NumberPair = (i32, i32);

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

fn read_number_pairs(
    content: &str,
    sort: bool,
) -> Result<Vec<NumberPair>, Box<dyn std::error::Error>> {
    let mut pairs: Vec<NumberPair> = content
        .lines()
        .filter_map(|line| {
            let mut iter = line.split_whitespace();
            match (iter.next(), iter.next()) {
                (Some(first), Some(second)) => first
                    .parse::<i32>()
                    .ok()
                    .and_then(|n1| second.parse::<i32>().ok().map(|n2| (n1, n2))),
                _ => None,
            }
        })
        .collect();

    if sort {
        let (mut n1, mut n2): (Vec<i32>, Vec<i32>) = pairs.iter().cloned().unzip();

        n1.sort_unstable();
        n2.sort_unstable();

        pairs = n1.iter().zip(n2.iter()).map(|(&n1, &n2)| (n1, n2)).collect();
    }

    Ok(pairs)
}

fn solve_part_one(input: &str) -> i32 {
    read_number_pairs(input, true)
        .map(|pairs| pairs.iter().map(|(n1, n2)| (n1 - n2).abs()).sum())
        .unwrap_or(0)
}

fn solve_part_two(input: &str) -> i32 {
    let pairs = match read_number_pairs(input, false) {
        Ok(p) => p,
        Err(_) => return 0,
    };

    let freq_map: HashMap<i32, i32> = pairs
        .iter()
        .map(|(_, n2)| *n2)
        .fold(HashMap::new(), |mut acc, n| {
            *acc.entry(n).or_insert(0) += 1;
            acc
        });

    pairs
        .iter()
        .map(|(n1, _)| {
            let frequency = freq_map.get(n1).unwrap_or(&0);
            n1 * frequency
        })
        .sum()
}
