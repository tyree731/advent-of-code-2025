#[derive(Debug, PartialEq)]
enum GridItem {
    EMPTY,
    PAPER,
}

pub fn aoc_4_1(lines: &Vec<String>) -> i32 {
    let grid = parse_lines(lines);
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            count += solve_1(&grid, i, j);
        }
    }
    count
}

pub fn aoc_4_2(lines: &Vec<String>) -> i32 {
    let mut grid = parse_lines(lines);
    let mut count = 0;
    let mut i = 0;
    let mut j = 0;
    let mut position_adjusted = false;
    while i < grid.len() {
        while j < grid[i].len() {
            position_adjusted = false;
            let result = solve_1(&grid, i, j);
            if result == 0 {
                j += 1;
                continue;
            }
            count += 1;
            grid[i][j] = GridItem::EMPTY;
            if i > 0 {
                i = i - 1;
                position_adjusted = true;
            }
            if j > 0 {
                j = j - 1;
                position_adjusted = true;
            }
            if position_adjusted {
                break;
            } else {
                j += 1;
            }
        }
        if position_adjusted {
            continue;
        } else {
            i += 1;
            j = 0;
        }
    }
    count
}

fn solve_1(grid: &Vec<Vec<GridItem>>, i: usize, j: usize) -> i32 {
    if i >= grid.len() {
        return 0;
    } else if j >= grid[i].len() {
        return 0;
    }
    let result = match grid[i][j] {
        GridItem::EMPTY => 0,
        GridItem::PAPER => {
            if count_papers(&grid, i, j) < 4 {
                1
            } else {
                0
            }
        }
    };
    result
}

fn count_papers(grid: &Vec<Vec<GridItem>>, i: usize, j: usize) -> i32 {
    let mut count = 0;
    for k in i as isize - 1..i as isize + 2 {
        if k < 0 || k >= grid.len() as isize {
            continue;
        }
        for l in j as isize - 1..j as isize + 2 {
            if l < 0 || l >= grid[i].len() as isize {
                continue;
            } else if k == i as isize && l == j as isize {
                continue;
            }
            match grid[k as usize][l as usize] {
                GridItem::PAPER => count += 1,
                GridItem::EMPTY => (),
            }
        }
    }
    count
}

fn parse_lines(lines: &Vec<String>) -> Vec<Vec<GridItem>> {
    let mut result: Vec<Vec<GridItem>> = Vec::new();
    for line in lines {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }
        let mut row: Vec<GridItem> = Vec::new();
        for char in line.chars() {
            if char == '.' {
                row.push(GridItem::EMPTY);
            } else if char == '@' {
                row.push(GridItem::PAPER);
            } else {
                panic!("unknown char: {}", char);
            }
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
        let test_data = vec!["..@@.@@@@.".to_string(), "@@@.@.@.@@".to_string()];
        let grid = parse_lines(&test_data);
        assert_eq!(
            grid,
            vec![
                vec![
                    GridItem::EMPTY,
                    GridItem::EMPTY,
                    GridItem::PAPER,
                    GridItem::PAPER,
                    GridItem::EMPTY,
                    GridItem::PAPER,
                    GridItem::PAPER,
                    GridItem::PAPER,
                    GridItem::PAPER,
                    GridItem::EMPTY
                ],
                vec![
                    GridItem::PAPER,
                    GridItem::PAPER,
                    GridItem::PAPER,
                    GridItem::EMPTY,
                    GridItem::PAPER,
                    GridItem::EMPTY,
                    GridItem::PAPER,
                    GridItem::EMPTY,
                    GridItem::PAPER,
                    GridItem::PAPER,
                ],
            ]
        );
    }

    #[test]
    fn test_aoc_4_1() {
        let test_data = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
        "#;
        let lines = test_data
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = aoc_4_1(&lines);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_aoc_4_2() {
        let test_data = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
        "#;
        let lines = test_data
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = aoc_4_2(&lines);
        assert_eq!(result, 43);
    }
}
