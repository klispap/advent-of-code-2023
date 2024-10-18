use std::collections::VecDeque;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let x = input
        .to_string()
        .split_ascii_whitespace()
        .fold(0, |acc, x| {
            let deque: VecDeque<char> = x.chars().filter(|x| x.is_digit(10)).collect();

            // Construct the calibration value
            let mut value = String::new();
            value.push(deque.front().copied().unwrap_or_default());
            value.push(deque.back().copied().unwrap_or_default());

            acc + value.parse::<u32>().unwrap_or_default()
        });

    if x == 0 {
        None
    } else {
        Some(x)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let x = input
        .to_string()
        .split_ascii_whitespace()
        .fold(0, |acc, x| {
            let deque: VecDeque<char> = x
                .chars()
                .enumerate()
                .filter_map(|(idx, w)| {
                    if w.is_digit(10) {
                        return Some(w);
                    } else if x[idx..].starts_with("one") {
                        return Some('1');
                    } else if x[idx..].starts_with("two") {
                        return Some('2');
                    } else if x[idx..].starts_with("three") {
                        return Some('3');
                    } else if x[idx..].starts_with("four") {
                        return Some('4');
                    } else if x[idx..].starts_with("five") {
                        return Some('5');
                    } else if x[idx..].starts_with("six") {
                        return Some('6');
                    } else if x[idx..].starts_with("seven") {
                        return Some('7');
                    } else if x[idx..].starts_with("eight") {
                        return Some('8');
                    } else if x[idx..].starts_with("nine") {
                        return Some('9');
                    } else {
                        None
                    }
                })
                .collect();

            // Construct the calibration value
            let mut value = String::new();
            value.push(deque.front().copied().unwrap_or_default());
            value.push(deque.back().copied().unwrap_or_default());

            acc + value.parse::<u32>().unwrap_or_default()
        });

    if x == 0 {
        None
    } else {
        Some(x)
    }
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
