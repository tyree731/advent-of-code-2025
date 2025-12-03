use rayon::prelude::*;

pub fn aoc_2_1(contents: &String) -> i64 {
    let ranges = parse_contents(contents);
    let mut sum = 0;
    for range in ranges {
        let (begin, end) = range;
        for id in begin..end + 1 {
            if invalid_id(id) {
                sum += id;
            }
        }
    }
    sum
}

#[allow(dead_code)]
pub fn aoc_2_2(contents: &String) -> i64 {
    let ranges = parse_contents(contents);
    let mut sum = 0;
    for range in ranges {
        let (begin, end) = range;
        for id in begin..end + 1 {
            if invalid_id_deux(id) {
                sum += id;
            }
        }
    }
    sum
}

pub fn aoc_2_2_2(contents: &String) -> i64 {
    let ranges = parse_contents(contents);
    let sum = ranges
        .par_iter()
        .map(|range| {
            let (begin, end) = range;
            let mut sum = 0;
            for id in *begin..*end + 1 {
                if id == 0 {
                    continue;
                } else if id < 0 {
                    sum += id;
                    continue;
                }
                let digits = 1 + id.ilog10();
                if digits == 1 {
                    continue;
                }
                let mut good = true;
                for pattern_length in 1..(digits / 2) + 1 {
                    if digits % pattern_length != 0 {
                        continue;
                    }
                    let cutoff = 10i64.pow(pattern_length);
                    let initial = id % cutoff;
                    let mut leftover = id / cutoff;
                    let mut loop_finished = true;
                    for _i in 1..digits / pattern_length {
                        if leftover % cutoff != initial {
                            loop_finished = false;
                            break;
                        }
                        leftover /= cutoff;
                    }
                    if loop_finished {
                        good = false;
                        break;
                    }
                }
                if !good {
                    sum += id;
                    continue;
                }
            }
            return sum;
        })
        .sum();
    sum
}

fn parse_contents(contents: &String) -> Vec<(i64, i64)> {
    let trimmed_contents = contents.trim();
    let mut results: Vec<(i64, i64)> = Vec::new();
    let possible_ranges = trimmed_contents.split(",");
    for range in possible_ranges {
        let possible_numbers = range.split("-").collect::<Vec<&str>>();
        if possible_numbers.len() != 2 {
            continue;
        }
        let left = match possible_numbers[0].parse::<i64>() {
            Ok(x) => x,
            Err(_) => continue,
        };
        let right = match possible_numbers[1].parse::<i64>() {
            Ok(x) => x,
            Err(_) => continue,
        };
        if left > right {
            continue;
        }
        results.push((left, right));
    }
    results
}

fn invalid_id(id: i64) -> bool {
    if id == 0 {
        return false;
    } else if id < 0 {
        return true;
    }
    let digits = 1 + id.ilog10();
    if (digits % 2) == 1 {
        return false;
    }
    let cutoff = 10i64.checked_pow(digits / 2).expect("invalid digits");
    let left = id / cutoff;
    let right = id % cutoff;
    left == right
}

fn invalid_id_deux(id: i64) -> bool {
    if id == 0 {
        return false;
    } else if id < 0 {
        return true;
    }
    let digits = 1 + id.ilog10();
    if digits == 1 {
        return false;
    }
    for pattern_length in 1..digits {
        if digits % pattern_length != 0 {
            continue;
        }
        let id_segments = segment_number(id, pattern_length);
        let first = id_segments.first().unwrap();
        if id_segments.iter().all(|x| x == first) {
            return true;
        }
    }
    false
}

fn segment_number(num: i64, digits: u32) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut leftover = num;
    let cutoff = 10i64.checked_pow(digits).expect("invalid digits");
    while leftover >= cutoff {
        result.push(leftover % cutoff);
        leftover = leftover / cutoff;
    }
    result.push(leftover);
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ids() {
        let ids = vec![
            1, 2, 3, 10, 12, 98, 100, 1009, 1011, 222220, 38593856, 38593862, 1188511880,
        ];
        for id in ids {
            assert!(!invalid_id(id));
        }
    }
    macro_rules! test_invalid_id {
        ($name:ident, $i:expr) => {
            #[test]
            fn $name() {
                assert!(invalid_id($i));
            }
        };
    }
    test_invalid_id!(invalid_id_11, 11);
    test_invalid_id!(invalid_id_99, 99);
    test_invalid_id!(invalid_id_1010, 1010);
    test_invalid_id!(invalid_id_222222, 222222);
    test_invalid_id!(invalid_id_446446, 446446);
    test_invalid_id!(invalid_id_38593859, 38593859);
    test_invalid_id!(invalid_id_1188511885, 1188511885);

    #[test]
    fn test_parse_contents() {
        let test_data = r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
        "#;
        let result = parse_contents(&test_data.to_string());
        assert_eq!(
            result,
            vec![
                (11, 22),
                (95, 115),
                (998, 1012),
                (1188511880, 1188511890),
                (222220, 222224),
                (446443, 446449),
                (38593856, 38593862),
                (565653, 565659),
                (2121212118, 2121212124)
            ]
        );
    }

    #[test]
    fn test_aoc_2_1() {
        let test_data = r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
        "#;
        assert_eq!(aoc_2_1(&test_data.to_string()), 1227775554);
    }

    #[test]
    fn test_valid_ids_deux() {
        let ids = vec![
            1, 2, 3, 10, 12, 98, 100, 1009, 1011, 222220, 38593856, 38593862, 1188511880,
        ];
        for id in ids {
            assert!(!invalid_id_deux(id));
        }
    }
    macro_rules! test_invalid_id_deux {
        ($name:ident, $i:expr) => {
            #[test]
            fn $name() {
                assert!(invalid_id_deux($i));
            }
        };
    }
    test_invalid_id_deux!(invalid_id_deux_11, 11);
    test_invalid_id_deux!(invalid_id_deux_99, 99);
    test_invalid_id_deux!(invalid_id_deux_111, 111);
    test_invalid_id_deux!(invalid_id_deux_1010, 1010);
    test_invalid_id_deux!(invalid_id_deux_222222, 222222);
    test_invalid_id_deux!(invalid_id_deux_446446, 446446);
    test_invalid_id_deux!(invalid_id_deux_565656, 565656);
    test_invalid_id_deux!(invalid_id_deux_38593859, 38593859);
    test_invalid_id_deux!(invalid_id_deux_824824824, 824824824);
    test_invalid_id_deux!(invalid_id_deux_1188511885, 1188511885);
    test_invalid_id_deux!(invalid_id_deux_2121212121, 2121212121);
}
