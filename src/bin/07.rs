use std::collections::HashSet;

advent_of_code::solution!(7);

fn is_in_bounds(num_rows: usize, num_cols: usize, r: usize, c: usize) -> bool {
    r < num_rows && r != usize::MAX && c < num_cols && c != usize::MAX
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut s_loc: (usize, usize) = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                s_loc = (i, j);
                break
            }
        }
    }

    let mut cnt = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut q: Vec<(usize, usize)> = vec![s_loc];
    while q.len() > 0 {
        let node = q.pop().unwrap();
        if grid[node.0][node.1] == '^' {
            cnt += 1;
            for dc in [-1, 1] {
                let nc = (node.1 as isize + dc) as usize;
                let nr = node.0 + 1;
                if is_in_bounds(grid.len(), grid[0].len(), nr, nc) && !visited.contains(&(nr, nc)) {
                    visited.insert((nr, nc));
                    q.push((nr, nc));
                }

            }
        } else {
            let nc = node.1;
            let nr = node.0 + 1;
            if is_in_bounds(grid.len(), grid[0].len(), nr, nc) && !visited.contains(&(nr, nc)) {
                visited.insert((nr, nc));
                q.push((nr, nc));
            }
        }
    }


    Some(cnt)
}


pub fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut s_loc: (usize, usize) = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                s_loc = (i, j);
                break
            }
        }
    }

    let mut prev_col_cnts: Vec<usize> = vec![0; grid[0].len()];
    prev_col_cnts[s_loc.1] = 1;
    for prev_r in 0..grid.len() - 1 {
        let mut cur_col_cnts: Vec<usize> = vec![0; grid[0].len()];
        for c in 0..grid[0].len() {
            if grid[prev_r][c] == '^' {
                cur_col_cnts[c-1] += prev_col_cnts[c];
                cur_col_cnts[c+1] += prev_col_cnts[c];
            } else {
                cur_col_cnts[c] += prev_col_cnts[c];
            }
        }
        prev_col_cnts = cur_col_cnts;
    }
    Some(prev_col_cnts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}