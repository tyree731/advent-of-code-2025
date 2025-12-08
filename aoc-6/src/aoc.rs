#[derive(Clone, Copy, Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

struct Problem {
    op: Operation,
    values: Vec<i64>,
}

pub fn aoc_6_1(lines: &Vec<String>) -> i64 {
    let problems = parse_lines(lines).unwrap();
    let mut sum = 0;
    for problem in problems {
        if problem.op == Operation::Add {
            sum += problem.values.iter().sum::<i64>();
        } else if problem.op == Operation::Multiply {
            sum += problem.values.iter().product::<i64>();
        }
    }
    sum
}

pub fn aoc_6_2(lines: &Vec<String>) -> i64 {
    let problems = parse_lines_deux(lines).unwrap();
    let mut sum = 0;
    for problem in problems {
        if problem.op == Operation::Add {
            sum += problem.values.iter().sum::<i64>();
        } else if problem.op == Operation::Multiply {
            sum += problem.values.iter().product::<i64>();
        }
    }
    sum
}

fn parse_lines(lines: &Vec<String>) -> Result<Vec<Problem>, String> {
    if lines.is_empty() {
        return Err("input is empty".to_string());
    }
    let mut tokens: Vec<Vec<String>> = Vec::new();
    for line in lines {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }
        let line_tokens = trimmed_line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        tokens.push(line_tokens);
    }
    let problem_count = tokens.first().unwrap().len();
    let problem_size = tokens.len();
    if !tokens.iter().all(|v| v.len() == problem_count) {
        return Err("not all problem counts are the same length".to_string());
    }

    let mut problems = Vec::new();
    for i in 0..problem_count {
        let mut problem_tokens = Vec::new();
        for j in 0..problem_size {
            problem_tokens.push(tokens[j][i].to_string());
        }
        let value_strings = problem_tokens[0..problem_tokens.len() - 1].to_vec();
        let values = value_strings
            .iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let op_string = problem_tokens[problem_tokens.len() - 1].to_string();
        let op = if op_string == "*" {
            Operation::Multiply
        } else {
            Operation::Add
        };
        problems.push(Problem { op, values });
    }
    Ok(problems)
}

fn parse_lines_deux(lines: &Vec<String>) -> Result<Vec<Problem>, String> {
    if lines.is_empty() {
        return Err("input is empty".to_string());
    }
    for i in 10..0 {
        println!("{}", i);
    }
    let filtered_lines = lines.iter().filter(|l| !l.is_empty()).collect::<Vec<_>>();
    let mut transposed_lines = Vec::new();
    for j in (0..filtered_lines[0].len()).rev() {
        let mut line = String::new();
        for i in 0..filtered_lines.len() {
            line.push(filtered_lines[i].chars().nth(j).unwrap());
        }
        transposed_lines.push(line);
    }
    let mut values = Vec::new();
    let mut problems = Vec::new();
    for line in transposed_lines {
        let trimmed_line = line.trim().to_string();
        if trimmed_line.is_empty() {
            continue;
        }
        let op_string = trimmed_line.chars().nth(trimmed_line.len() - 1).unwrap();
        if op_string == '*' || op_string == '+' {
            let op = if op_string == '*' {
                Operation::Multiply
            } else {
                Operation::Add
            };
            values.push(
                trimmed_line[0..trimmed_line.len() - 1]
                    .trim()
                    .parse::<i64>()
                    .unwrap(),
            );
            let problem = Problem {
                op: op,
                values: values.clone(),
            };
            problems.push(problem);
            values.clear();
        } else {
            values.push(trimmed_line.parse::<i64>().unwrap());
        }
    }
    Ok(problems)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let test_data = r#"
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
        "#;
        let lines = test_data
            .lines()
            .map(|l| l.trim().to_string())
            .collect::<Vec<String>>();
        let problems = parse_lines(&lines).unwrap();
        assert_eq!(problems.len(), 4);
        assert_eq!(problems[0].op, Operation::Multiply);
        assert_eq!(problems[1].op, Operation::Add);
        assert_eq!(problems[2].op, Operation::Multiply);
        assert_eq!(problems[3].op, Operation::Add);
        assert_eq!(problems[0].values, vec![123, 45, 6]);
        assert_eq!(problems[1].values, vec![328, 64, 98]);
        assert_eq!(problems[2].values, vec![51, 387, 215]);
        assert_eq!(problems[3].values, vec![64, 23, 314]);
    }

    #[test]
    fn test_aoc_6_1() {
        let test_data = r#"
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
        "#;
        let lines = test_data
            .lines()
            .map(|l| l.trim().to_string())
            .collect::<Vec<String>>();
        let result = aoc_6_1(&lines);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_parse_lines_deux() {
        let lines = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        let problems = parse_lines_deux(&lines).unwrap();
        assert_eq!(problems.len(), 4);
        assert_eq!(problems[0].op, Operation::Add);
        assert_eq!(problems[1].op, Operation::Multiply);
        assert_eq!(problems[2].op, Operation::Add);
        assert_eq!(problems[3].op, Operation::Multiply);
        assert_eq!(problems[0].values, vec![4, 431, 623]);
        assert_eq!(problems[1].values, vec![175, 581, 32]);
        assert_eq!(problems[2].values, vec![8, 248, 369]);
        assert_eq!(problems[3].values, vec![356, 24, 1]);
    }

    #[test]
    fn test_aoc_6_2() {
        let lines = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        let result = aoc_6_2(&lines);
        assert_eq!(result, 3263827);
    }
}
