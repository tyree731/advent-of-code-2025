#[derive(Debug, PartialEq)]
enum Type {
    EMPTY,
    SPLITTER,
    START,
}

#[derive(Debug, PartialEq)]
struct Node {
    ty: Type,
}

impl Node {
    pub fn new(ch: char) -> Node {
        Node {
            ty: Node::char_to_type(ch),
        }
    }

    fn char_to_type(ch: char) -> Type {
        if ch == '^' {
            return Type::SPLITTER;
        } else if ch == 'S' {
            return Type::START;
        }
        Type::EMPTY
    }
}

pub fn aoc_7_1(lines: &Vec<String>) -> u64 {
    let grid = parse_lines(lines).unwrap();
    let mut current_beams = Vec::new();
    for j in 0..grid[0].len() {
        match grid[0][j].ty {
            Type::START => current_beams.push(true),
            _ => current_beams.push(false),
        }
    }
    let mut splits = 0;
    for i in 1..grid.len() {
        for j in 0..grid[i].len() {
            if !current_beams[j] {
                continue;
            }
            match grid[i][j].ty {
                Type::SPLITTER => {
                    let mut split = true;
                    if j != 0 {
                        if !current_beams[j - 1] {
                            split = true;
                            current_beams[j - 1] = true;
                        }
                    }
                    if j != current_beams.len() - 1 {
                        if !current_beams[j + 1] {
                            split = true;
                            current_beams[j + 1] = true;
                        }
                    }
                    current_beams[j] = false;
                    if split {
                        splits += 1;
                    }
                }
                _ => continue,
            }
        }
    }
    splits
}

pub fn aoc_7_2(lines: &Vec<String>) -> u64 {
    let grid = parse_lines(lines).unwrap();
    let mut grid_timelines = vec![vec![0u64; grid[0].len()]; grid.len()];
    let mut start_i = 0;
    let mut start_j = 0;
    for i in (0..grid.len()).rev() {
        let mut splitter_indices = Vec::new();
        for j in 0..grid[i].len() {
            match grid[i][j].ty {
                Type::EMPTY | Type::START => {
                    if grid[i][j].ty == Type::START {
                        start_i = i;
                        start_j = j;
                    }
                    if i != grid.len() - 1 {
                        grid_timelines[i][j] = grid_timelines[i + 1][j]
                    } else {
                        grid_timelines[i][j] = 1;
                    }
                }
                Type::SPLITTER => {
                    splitter_indices.push(j);
                }
            }
        }
        for j in splitter_indices {
            grid_timelines[i][j] = 0;
            if j != 0 {
                grid_timelines[i][j] += grid_timelines[i][j - 1]
            }
            if j != grid[i].len() - 1 {
                grid_timelines[i][j] += grid_timelines[i][j + 1]
            }
        }
    }
    grid_timelines[start_i][start_j]
}

fn parse_lines(lines: &Vec<String>) -> Result<Vec<Vec<Node>>, String> {
    if lines.is_empty() {
        return Err("empty lines provided".to_string());
    }
    let trimmed_lines = lines
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    let line_length = trimmed_lines[0].len();
    if !trimmed_lines.iter().all(|l| l.len() == line_length) {
        return Err("line lengths do not match".to_string());
    }
    let grid = trimmed_lines
        .iter()
        .map(|l| l.chars().map(|c| Node::new(c)).collect::<Vec<Node>>())
        .collect::<Vec<Vec<Node>>>();
    Ok(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let test_data = r#"
.......S.......
...............
.......^.......
        "#;
        let lines = test_data
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        let grid = parse_lines(&lines).unwrap();
        assert_eq!(grid.len(), 3);
        assert_eq!(
            grid[0],
            vec![
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::START },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY }
            ]
        );
        assert_eq!(
            grid[1],
            vec![
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY }
            ]
        );
        assert_eq!(
            grid[2],
            vec![
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::SPLITTER },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY },
                Node { ty: Type::EMPTY }
            ]
        );
    }

    #[test]
    fn test_aoc_7_1() {
        let test_data = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
        "#;
        let lines = test_data
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        let result = aoc_7_1(&lines);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_aoc_7_2() {
        let test_data = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
        "#;
        let lines = test_data
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        let result = aoc_7_2(&lines);
        assert_eq!(result, 40);
    }
}
