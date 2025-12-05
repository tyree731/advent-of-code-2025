use std::cmp;

struct Ranges {
    ranges: Vec<(u64, u64)>,
}

impl Ranges {
    pub fn new(lines: &Vec<String>) -> Ranges {
        let mut ranges = Vec::new();
        for line in lines {
            let trimmed_line = line.trim();
            let possible_begin_end = trimmed_line.split("-").collect::<Vec<&str>>();
            if possible_begin_end.len() != 2 {
                continue;
            }
            let left = match possible_begin_end[0].parse::<u64>() {
                Ok(v) => v,
                Err(_) => continue,
            };
            let right = match possible_begin_end[1].parse::<u64>() {
                Ok(v) => v,
                Err(_) => continue,
            };
            ranges.push((left, right));
        }
        ranges = Ranges::compress_ranges(&ranges);
        Ranges { ranges }
    }

    pub fn in_range(&self, value: u64) -> bool {
        for range in &self.ranges {
            if range.0 <= value && value <= range.1 {
                return true;
            }
        }
        false
    }

    pub fn all_range_values_count(&self) -> u64 {
        self.ranges.iter().map(|(b, e)| *e + 1 - *b).sum()
    }

    fn compress_ranges(input: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
        if input.len() == 0 {
            return Vec::new();
        }
        let mut result = Vec::new();
        let mut input_copy = input.clone();
        input_copy.sort();
        let mut current = (input_copy[0].0, input_copy[0].1);
        for i in 1..input_copy.len() {
            let first = input_copy[i].0;
            let second = input_copy[i].1;
            if first >= current.0 && first <= (current.1 + 1) {
                current = (current.0, cmp::max(current.1, second));
            } else {
                result.push(current);
                current = (first, second);
            }
        }
        result.push(current);
        result
    }
}

pub fn aoc_5_1(lines: &Vec<String>) -> u64 {
    let (ranges, values) = parse_lines(lines).unwrap();
    values.iter().filter(|v| ranges.in_range(**v)).count() as u64
}

pub fn aoc_5_2(lines: &Vec<String>) -> u64 {
    let (ranges, _values) = parse_lines(lines).unwrap();
    ranges.all_range_values_count()
}

fn parse_lines(lines: &Vec<String>) -> Result<(Ranges, Vec<u64>), String> {
    let mut our_lines = lines
        .iter()
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>();
    if our_lines.len() < 3 {
        return Err("error: not enough lines".to_string());
    } else if lines.first().unwrap().is_empty() {
        our_lines.remove(0);
    } else if lines.last().unwrap().is_empty() {
        our_lines.pop();
    }

    let mut split_index = 0;
    for i in 0..our_lines.len() {
        if our_lines[i].is_empty() {
            split_index = i;
            break;
        }
    }
    if split_index == 0 {
        return Err("unable to find input split".to_string());
    }

    let range_lines = our_lines[0..split_index].to_vec();
    let ranges = Ranges::new(&range_lines);
    let input_lines = our_lines[split_index..our_lines.len()].to_vec();
    let mut inputs = Vec::new();
    for input in input_lines {
        let input_value = match input.parse::<u64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        inputs.push(input_value);
    }
    Ok((ranges, inputs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let test_data = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
        "#;
        let lines = test_data
            .lines()
            .map(|l| l.trim().to_string())
            .collect::<Vec<String>>();
        let (ranges, inputs) = parse_lines(&lines).unwrap();
        assert_eq!(inputs, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_aoc_5_1() {
        let test_data = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
        "#;
        let lines = test_data
            .lines()
            .map(|l| l.trim().to_string())
            .collect::<Vec<String>>();
        let (ranges, inputs) = parse_lines(&lines).unwrap();
        let in_range = [5, 11, 17];
        let not_in_range = [1, 8, 32];
        for in_value in in_range {
            assert!(ranges.in_range(in_value));
        }
        for out_value in not_in_range {
            assert!(!ranges.in_range(out_value));
        }
    }

    #[test]
    fn test_aoc_5_2() {
        let test_data = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
        "#;
        let lines = test_data
            .lines()
            .map(|l| l.trim().to_string())
            .collect::<Vec<String>>();
        let (ranges, _inputs) = parse_lines(&lines).unwrap();
        assert_eq!(ranges.all_range_values_count(), 14);
    }
}
