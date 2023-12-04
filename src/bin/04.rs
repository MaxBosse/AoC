use std::collections::VecDeque;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<i32> {
    let mut total = 0;
    for line in input.lines() {
        let line_split: Vec<&str> = line.split(": ").collect();
        let mut line_winnings = 0;

        let number_split: Vec<&str> = line_split.get(1).unwrap().split(" | ").collect();
        let winning_numbers: Vec<&str> = number_split.get(0).unwrap().split(" ").collect();
        let my_numbers: Vec<&str> = number_split.get(1).unwrap().split(" ").collect();

        for number in my_numbers {
            if number == "" { continue; }

            if winning_numbers.contains(&number) {
                if line_winnings == 0 {
                    line_winnings = 1;
                } else {
                    line_winnings *= 2;
                }
            }
        }
        total += line_winnings;
    }
    


    Some(total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut total = 0;
    let mut card_amount = VecDeque::new();

    for line in input.lines() {
        let line_split: Vec<&str> = line.split(": ").collect();
        let mut line_winnings = 0;

        let number_split: Vec<&str> = line_split.get(1).unwrap().split(" | ").collect();
        let winning_numbers: Vec<&str> = number_split.get(0).unwrap().split(" ").collect();
        let my_numbers: Vec<&str> = number_split.get(1).unwrap().split(" ").collect();

        for number in my_numbers {
            if number == "" { continue; }

            if winning_numbers.contains(&number) {
                line_winnings += 1;
            }
        }

        if let Some(extra_cards) = card_amount.pop_front() {
            total += extra_cards;

            for _y in 0..extra_cards {
                for i in 0..line_winnings {
                    if let Some(elem) = card_amount.get_mut(i) {
                        *elem += 1;
                    } else {
                        card_amount.insert(i, 1);
                    }
                }        
            }
        }

        total += 1;
        for i in 0..line_winnings {
            if let Some(elem) = card_amount.get_mut(i) {
                *elem += 1;
            } else {
                card_amount.insert(i, 1);
            }
        }
    }
    


    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
