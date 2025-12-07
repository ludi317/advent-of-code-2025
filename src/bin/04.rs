advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '@' {
                continue;
            }
            let mut cnt = 0;
            for di in -1isize..=1 {
                for dj in -1isize..=1 {
                    let ni = (i as isize + di) as usize;
                    let nj = (j as isize + dj) as usize;
                    if (di == 0 && dj == 0)
                        || ni == usize::MAX
                        || ni == grid.len()
                        || nj == usize::MAX
                        || nj == grid[0].len()
                        || grid[ni][nj] != '@'
                    {
                        continue;
                    }
                    cnt += 1;
                }
            }
            if cnt < 4 {
                ans += 1;
            }
        }
    }
    Some(ans)
}

fn hash(row: usize, col: usize, ncol: usize) -> usize {
    row * ncol + col
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut ans = 0;
    let mut cnts: Vec<usize> = vec![0; grid.len() * grid[0].len()];
    let mut q: Vec<usize> = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '@' {
                continue;
            }
            let mut cnt = 0;
            for di in -1isize..=1 {
                for dj in -1isize..=1 {
                    let ni = (i as isize + di) as usize;
                    let nj = (j as isize + dj) as usize;
                    if (di == 0 && dj == 0)
                        || ni == usize::MAX
                        || ni == grid.len()
                        || nj == usize::MAX
                        || nj == grid[0].len()
                        || grid[ni][nj] != '@'
                    {
                        continue;
                    }
                    cnt += 1;
                }
            }
            let h = hash(i, j, grid[0].len());
            cnts[h] = cnt;
            if cnt < 4 {
                q.push(h)
            }
        }
    }

    while q.len() > 0 {
        let h = q.pop().unwrap();
        ans += 1;

        let (i, j) = (h / grid[0].len(), h % grid[0].len());
        for di in -1isize..=1 {
            for dj in -1isize..=1 {
                let ni = (i as isize + di) as usize;
                let nj = (j as isize + dj) as usize;
                if (di == 0 && dj == 0)
                    || ni == usize::MAX
                    || ni == grid.len()
                    || nj == usize::MAX
                    || nj == grid[0].len()
                    || grid[ni][nj] != '@'
                {
                    continue;
                }
                let h = hash(ni, nj, grid[0].len());
                cnts[h] -= 1;
                if cnts[h] == 3 {
                    q.push(h);
                }
            }
        }
    }

    Some(ans)
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
        assert_eq!(result, Some(43));
    }
}
