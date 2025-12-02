pub fn aoc_1_1(lines: &Vec<String>) -> i32 {
    let parsed_lines = parse_lines(lines);
    count_zeros(&parsed_lines)
}

pub fn aoc_1_2(lines: &Vec<String>) -> i32 {
    let parsed_lines = parse_lines(lines);
    count_zeros_deux(&parsed_lines)
}

fn parse_lines(lines: &Vec<String>) -> Vec<i32> {
    let mut results = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        } else if line.len() <= 1 {
            continue;
        }
        let sign = if line.chars().nth(0).unwrap() == 'L' {
            -1
        } else {
            1
        };
        match line[1..].parse::<i32>() {
            Ok(num) => results.push(num * sign),
            Err(_) => continue,
        }
    }
    results
}

fn count_zeros(lines: &Vec<i32>) -> i32 {
    let mut dial = 50;
    let mut zeros = 0;
    for line in lines {
        dial = (dial + line) % 100;
        if dial == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn count_zeros_deux(lines: &Vec<i32>) -> i32 {
    let mut dial = 50;
    let mut zeros = 0;
    for line in lines {
        let temp_dial = dial + line;
        if dial > 0 && temp_dial <= 0 {
            zeros += 1;
        }
        if temp_dial <= -100 {
            zeros = zeros + ((-1 * temp_dial) / 100);
        } else if temp_dial >= 100 {
            zeros = zeros + (temp_dial / 100);
        }
        dial = temp_dial.rem_euclid(100);
    }
    zeros
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let test_data = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        "#;
        let lines = test_data
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = parse_lines(&lines);
        assert_eq!(10, result.len());
        assert_eq!(vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82], result);
    }

    #[test]
    fn count_zeros_test() {
        let test_data = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        "#;
        let lines = test_data
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let parsed_lines = parse_lines(&lines);
        let result = count_zeros(&parsed_lines);
        assert_eq!(3, result);
    }

    #[test]
    fn count_zeros_deux_test() {
        let test_data = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        "#;
        let lines = test_data
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let parsed_lines = parse_lines(&lines);
        let result = count_zeros_deux(&parsed_lines);
        assert_eq!(6, result);
    }
}
