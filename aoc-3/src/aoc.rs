const JOLTAGE_LENGTH: usize = 12;

pub fn aoc_3_1(lines: &Vec<String>) -> u64 {
    let parsed_lines = parse_lines(lines);
    let mut sum = 0;
    for line in parsed_lines {
        if line.len() < 2 {
            continue;
        }
        let mut first = line[0];
        let mut second = line[1];
        for i in 2..line.len() {
            let digit = line[i];
            if second > first {
                first = second;
                second = digit;
            } else if digit > second {
                second = digit;
            }
        }
        sum += (first * 10) + second;
    }
    sum
}

pub fn aoc_3_2(lines: &Vec<String>) -> u64 {
    let parsed_lines = parse_lines(lines);
    let mut sum = 0;
    for line in parsed_lines {
        if line.len() < JOLTAGE_LENGTH {
            continue;
        }
        let mut joltage: Vec<u64> = line.to_vec();
        joltage.resize(JOLTAGE_LENGTH, 0);
        for i in JOLTAGE_LENGTH..line.len() {
            let mut shifted_down = false;
            for j in 0..JOLTAGE_LENGTH - 1 {
                if joltage[j] < joltage[j + 1] {
                    shifted_down = true;
                    for k in j..JOLTAGE_LENGTH - 1 {
                        joltage[k] = joltage[k + 1];
                    }
                    break;
                }
            }
            let digit = line[i];
            if shifted_down {
                joltage[JOLTAGE_LENGTH - 1] = digit;
            } else if digit > joltage[joltage.len() - 1] {
                joltage[JOLTAGE_LENGTH - 1] = digit;
            }
        }
        let mut value = 0;
        for i in 0..JOLTAGE_LENGTH {
            value += 10u64.pow(JOLTAGE_LENGTH as u32 - 1 - i as u32) * joltage[i];
        }
        sum += value;
    }
    sum
}

fn parse_lines(lines: &Vec<String>) -> Vec<Vec<u64>> {
    let mut result: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        let trimmed_line = line.trim();
        if trimmed_line.len() == 0 {
            continue;
        }
        let mut row: Vec<u64> = Vec::new();
        for char in trimmed_line.chars() {
            let digit = match char.to_digit(10) {
                Some(d) => d as u64,
                None => continue,
            };
            row.push(digit);
        }
        result.push(row);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let test_input = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];
        let parsed_lines = parse_lines(&test_input);
        assert_eq!(parsed_lines.len(), 4);
        assert_eq!(
            parsed_lines[0],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            parsed_lines[1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]
        );
        assert_eq!(
            parsed_lines[2],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]
        );
        assert_eq!(
            parsed_lines[3],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]
        );
    }

    #[test]
    fn test_aoc_3_1() {
        let test_input = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];
        let result = aoc_3_1(&test_input);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_aoc_3_2() {
        let test_input = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];
        let result = aoc_3_2(&test_input);
        assert_eq!(result, 3121910778619);
    }
}
