advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let games_string = input.to_string();

    let games: Vec<&str> = games_string.split('\n').collect();

    let (max_red, max_green, max_blue): (u32, u32, u32) = (12, 13, 14);

    let mut games_total = 0;

    'game: for game in games {
        if game.is_empty() {
            continue;
        }

        let id: &str = game.split(&[' ', ':'][..]).collect::<Vec<&str>>()[1];

        let rounds: Vec<&str> = game.split(&[':', ';'][..]).collect();

        for round in rounds {
            let mut balls = 0;
            let (mut red, mut blue, mut green) = (0, 0, 0);

            for pat in round.split(&[' ', ','][..]) {
                if let Ok(x) = pat.parse::<u32>() {
                    balls = x;
                    continue;
                }

                if pat == "green" {
                    green = balls;
                } else if pat == "blue" {
                    blue = balls;
                } else if pat == "red" {
                    red = balls;
                }

                balls = 0;
            }

            if red > max_red || blue > max_blue || green > max_green {
                continue 'game;
            }
        }

        games_total += id.parse::<u32>().unwrap();
    }

    if games_total > 0 {
        Some(games_total)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let games_string = input.to_string();

    let games: Vec<&str> = games_string.split('\n').collect();

    let mut games_power = 0;

    'game: for game in games {
        if game.is_empty() {
            continue;
        }

        let rounds: Vec<&str> = game.split(&[':', ';'][..]).collect();

        let (mut min_red, mut min_blue, mut min_green) = (0, 0, 0);

        for round in rounds {
            let mut balls = 0;
            let (mut red, mut blue, mut green) = (0, 0, 0);

            for pat in round.split(&[' ', ','][..]) {
                if let Ok(x) = pat.parse::<u32>() {
                    balls = x;
                    continue;
                }

                if pat == "green" {
                    green = balls;
                } else if pat == "blue" {
                    blue = balls;
                } else if pat == "red" {
                    red = balls;
                }

                balls = 0;
            }

            if red > min_red {
                min_red = red;
            }
            if blue > min_blue {
                min_blue = blue;
            }
            if green > min_green {
                min_green = green;
            }
        }

        games_power += min_blue * min_red * min_green;
    }

    if games_power > 0 {
        Some(games_power)
    } else {
        None
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
