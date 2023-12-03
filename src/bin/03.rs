use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines: HashMap<usize, HashMap<usize, char>> = HashMap::new();

    let mut x = 0;
    for line in input.lines() {
        let mut cur_line = HashMap::new();

        for (y, c) in line.char_indices() {
            cur_line.insert(y, c);
        }

        lines.insert(x, cur_line);
        x += 1;
    }

    let mut total = 0;

    for i in 0..lines.len() {
        let Some(cur_line) = lines.get(&i) else {continue;};
        let mut cur_num = "".to_string();
        let mut num_counts = false;

        for j in 0..cur_line.len() {
            let Some(c) = cur_line.get(&j) else {continue;};

            if c.is_digit(10) {
                cur_num.push(*c);

                // The current number we are creating already neighbors a symbol, no need to check again
                if num_counts { continue; }

                // Check all diagonal chars for symbols
                for k in (j.saturating_sub(1))..=(j+1) {
                    if let Some(checked_c_before) = lines.get(&(i.saturating_sub(1))).and_then(|l| l.get(&k)) {
                        if checked_c_before != &'.' && !checked_c_before.is_digit(10) {
                            num_counts = true;
                        }
                    }
                    if let Some(checked_c_behind) = lines.get(&(i+1)).and_then(|l| l.get(&k)) {
                        if checked_c_behind != &'.' && !checked_c_behind.is_digit(10) {
                            num_counts = true;
                        }
                    }
                }

                // Check before and after the current char in the same line for symbols
                if !num_counts {
                    if let Some(checked_c_before) = lines.get(&i).and_then(|l| l.get(&(j+1))) {
                        if checked_c_before != &'.' && !checked_c_before.is_digit(10) {
                            num_counts = true;
                        }
                    }
                    if let Some(checked_c_behind) = lines.get(&i).and_then(|l| l.get(&(j.saturating_sub(1)))) {
                        if checked_c_behind != &'.' && !checked_c_behind.is_digit(10) {
                            num_counts = true;
                        }
                    }
                }
            } else {
                if cur_num != "" && num_counts{
                    let num: u32 = cur_num.parse().unwrap();
                    total += num;
                }

                cur_num = "".to_string();
                num_counts = false;
            }
        }

        if cur_num != "" && num_counts{
            let num: u32 = cur_num.parse().unwrap();
            total += num;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines: HashMap<usize, HashMap<usize, char>> = HashMap::new();
    let mut total = 0;

    let mut x = 0;
    for line in input.lines() {
        let mut cur_line = HashMap::new();

        for (y, c) in line.char_indices() {
            cur_line.insert(y, c);
        }

        lines.insert(x, cur_line);
        x += 1;
    }

    let mut parts = HashMap::new();

    for i in 0..lines.len() {
        let Some(cur_line) = lines.get(&i) else {continue;};
        let mut cur_num = "".to_string();
        let mut num_counts = false;
        let mut cur_line_parts = HashMap::new();

        for j in 0..cur_line.len() {
            let Some(c) = cur_line.get(&j) else {continue;};

            if c.is_digit(10) {
                cur_num.push(*c);

                // The current number we are creating already neighbors a symbol, no need to check again
                if num_counts { continue; }

                // Check all diagonal chars for symbols
                for k in (j.saturating_sub(1))..=(j+1) {
                    if let Some(checked_c_before) = lines.get(&(i.saturating_sub(1))).and_then(|l| l.get(&k)) {
                        if checked_c_before != &'.' && !checked_c_before.is_digit(10) {
                            num_counts = true;
                        }
                    }
                    if let Some(checked_c_behind) = lines.get(&(i+1)).and_then(|l| l.get(&k)) {
                        if checked_c_behind != &'.' && !checked_c_behind.is_digit(10) {
                            num_counts = true;
                        }
                    }
                }

                // Check before and after the current char in the same line for symbols
                if !num_counts {
                    if let Some(checked_c_before) = lines.get(&i).and_then(|l| l.get(&(j+1))) {
                        if checked_c_before != &'.' && !checked_c_before.is_digit(10) {
                            num_counts = true;
                        }
                    }
                    if let Some(checked_c_behind) = lines.get(&i).and_then(|l| l.get(&(j.saturating_sub(1)))) {
                        if checked_c_behind != &'.' && !checked_c_behind.is_digit(10) {
                            num_counts = true;
                        }
                    }
                }
            } else {
                if cur_num != "" && num_counts{
                    let start_index = j.saturating_sub(cur_num.chars().count()) ;
                    let num: u32 = cur_num.parse().unwrap();

                    for m in start_index..j {
                        cur_line_parts.insert(m, num);
                    }
                }

                cur_num = "".to_string();
                num_counts = false;
            }
        }

        if cur_num != "" && num_counts{
            let start_index = cur_line.len() - cur_num.chars().count();
            let num: u32 = cur_num.parse().unwrap();

            for m in start_index..(cur_line.len()) {
                cur_line_parts.insert(m, num);
            }
        }

        parts.insert(i, cur_line_parts);
    }

    for i in 0..lines.len() {
        let Some(cur_line) = lines.get(&i) else {continue;};

        for j in 0..cur_line.len() {
            let Some(c) = cur_line.get(&j) else {continue;};

            if c == &'*' {
                let mut part_total = vec![];

                for k in (i.saturating_sub(1))..=(i+1) {
                    let mut used_parts = vec![];

                    for m in (j.saturating_sub(1))..=(j+1) {
                        let Some(num) = parts.get(&k).and_then(|lp| lp.get(&m)) else {
                            used_parts = vec![];
                            continue;
                        };
                        
                        if !used_parts.contains(num) {
                            used_parts.push(*num);
                            part_total.push(*num);
                        }
                    }
                }
                
                if part_total.len() == 2 {
                    total += part_total.get(0).unwrap() * part_total.get(1).unwrap();
                    
                }
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
