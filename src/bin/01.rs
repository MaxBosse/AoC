use std::char::from_digit;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total:u32 = 0;

    for line in input.lines() {
        let mut first_digit = '0';
        let mut second_digit = '0';

        for c in line.chars() {
            if c.is_numeric() {
                first_digit = c;
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_numeric() {
                second_digit = c;
                break;
            }
        }

        let num: u32 = format!("{}{}", first_digit, second_digit).parse().unwrap();
        total += num;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut total:u32 = 0;

    for line in input.lines() {
        let mut first_digit = '0';
        let mut second_digit = '0';


        'outer: for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                first_digit = c;
                break;
            }
            for y in 0..numbers.len() {
                let number = numbers[y];
                if i+number.len() > line.len() {
                    continue;
                }

                if line[i..(number.len()+i)].starts_with(number) {
                    first_digit = from_digit((y+1).try_into().unwrap(), 10).unwrap();
                    break 'outer;
                }
            }
        }

        'outer: for (i, c) in line.chars().rev().enumerate() {
            let z = line.chars().count() - i - 1;
            if c.is_numeric() {
                second_digit = c;
                break;
            }
            for y in 0..numbers.len() {
                let number = numbers[y];
                if z+number.len() > line.len() {
                    continue;
                }

                if line[z..(number.len()+z)].starts_with(number) {
                    second_digit = from_digit((y+1).try_into().unwrap(), 10).unwrap();
                    break 'outer;
                }
            }
        }

        let num: u32 = format!("{}{}", first_digit, second_digit).parse().unwrap();
        total += num;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
