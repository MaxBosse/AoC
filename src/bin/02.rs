use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let re = Regex::new(r"^Game ([0-9]+): (.*)$").unwrap();

    'outer: for line in input.lines() {
        let Some((_full, [game_id, games])) = re.captures(line).map(|c| c.extract()) else { continue; };

        let game_num: u32 = game_id.parse().unwrap();
        let mut cur_group_num = 0;

        for game in games.split(";") {
            for group in game.split(",") {
                for pair in group.split(" ") {
                    if pair == "" {
                        continue;
                    }
                    
                    if cur_group_num == 0 {
                        cur_group_num = pair.parse().unwrap();
                        continue;
                    }

                    match pair {
                        "green" => {
                            if cur_group_num > max_green {
                                continue 'outer;
                            }
                        },
                        "red" => {
                            if cur_group_num > max_red {
                                continue 'outer;
                            }
                        },
                        "blue" => {
                            if cur_group_num > max_blue {
                                continue 'outer;
                            }
                        }
                        _ => println!("Unknown pair: {}", pair),
                    }
                }
                cur_group_num = 0;
            }
        }

        total += game_num;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;

    let re = Regex::new(r"^Game ([0-9]+): (.*)$").unwrap();

    for line in input.lines() {
        let Some((_full, [_game_id, games])) = re.captures(line).map(|c| c.extract()) else { continue; };

        let mut cur_group_num = 0;

        for game in games.split(";") {
            for group in game.split(",") {
                for pair in group.split(" ") {
                    if pair == "" {
                        continue;
                    }
                    
                    if cur_group_num == 0 {
                        cur_group_num = pair.parse().unwrap();
                        continue;
                    }

                    match pair {
                        "green" => {
                            if cur_group_num > max_green {
                                max_green = cur_group_num;
                            }
                        },
                        "red" => {
                            if cur_group_num > max_red {
                                max_red = cur_group_num;
                            }
                        },
                        "blue" => {
                            if cur_group_num > max_blue {
                                max_blue = cur_group_num;
                            }
                        }
                        _ => println!("Unknown pair: {}", pair),
                    }
                }
                cur_group_num = 0;
            }
        }

        total += max_blue * max_green * max_red;
        max_red = 0;
        max_green = 0;
        max_blue = 0;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
