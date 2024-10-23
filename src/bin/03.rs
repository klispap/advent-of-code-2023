use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn check_part(
    symbols: &HashSet<(usize, usize)>,
    row_idx: usize,
    part_start_idx: usize,
    part_end_idx: usize,
    value: u32,
) -> u32 {
    let mut neighbors: HashSet<(usize, usize)> = HashSet::new();

    print!(
        "CHECK: [{}] ({},{}) : {}",
        row_idx, part_start_idx, part_end_idx, value
    );

    neighbors.clear();

    for x in part_start_idx..=part_end_idx {
        if x > 0 {
            neighbors.insert((row_idx, x - 1));
            neighbors.insert((row_idx + 1, x - 1));
        }
        if row_idx > 0 {
            neighbors.insert((row_idx - 1, x));
            neighbors.insert((row_idx - 1, x + 1));
        }

        if row_idx > 0 && x > 0 {
            neighbors.insert((row_idx - 1, x - 1));
        }
        neighbors.insert((row_idx + 1, x));
        neighbors.insert((row_idx, x + 1));
        neighbors.insert((row_idx + 1, x + 1));
    }

    let check_symbs: HashSet<(usize, usize)> = symbols.intersection(&neighbors).copied().collect();

    if !check_symbs.is_empty() {
        print!(" Useful part!");
        value
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut symbols: HashSet<(usize, usize)> = HashSet::new();

    let mut sum_parts = 0;

    let engine_rows: Vec<&str> = input.split('\n').collect();

    // Collect symbols
    for (row_idx, &row) in engine_rows.iter().enumerate() {
        for (col_idx, x) in row.chars().enumerate() {
            if x != '.' && !x.is_digit(10) {
                // println!("SYMBOL {} : {:?}", x, (row_idx, col_idx));
                symbols.insert((row_idx, col_idx));
            }
        }
    }

    // Collect parts
    for (row_idx, &row) in engine_rows.iter().enumerate() {
        let mut part_start_idx: Option<usize> = None;
        let mut part_end_idx: Option<usize> = None;

        for (col_idx, x) in row.chars().enumerate() {
            // Symbol or period
            if !x.is_digit(10) && part_start_idx.is_some() && part_end_idx.is_some() {
                let value = row
                    .get(part_start_idx.unwrap()..=part_end_idx.unwrap())
                    .unwrap_or_default()
                    .parse::<usize>()
                    .unwrap_or_default();

                sum_parts += check_part(
                    &symbols,
                    row_idx,
                    part_start_idx.unwrap(),
                    part_end_idx.unwrap(),
                    value as u32,
                );

                println!(" {}", sum_parts);

                part_start_idx = None;
            } else if x.is_digit(10) {
                // Digit (part of a number)
                if part_start_idx.is_none() {
                    part_start_idx = Some(col_idx);
                }
                part_end_idx = Some(col_idx);

                if col_idx == row.len() - 1 {
                    let value = row
                        .get(part_start_idx.unwrap()..=part_end_idx.unwrap())
                        .unwrap_or_default()
                        .parse::<usize>()
                        .unwrap_or_default();

                    sum_parts += check_part(
                        &symbols,
                        row_idx,
                        part_start_idx.unwrap(),
                        part_end_idx.unwrap(),
                        value as u32,
                    );

                    println!(" {}", sum_parts);

                    part_start_idx = None;
                }
            }
        }
    }

    println!("SUM: {:?}", sum_parts);

    Some(sum_parts as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
