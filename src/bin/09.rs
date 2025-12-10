use advent_of_code::get_nums_usize;

advent_of_code::solution!(9);

#[derive(Debug, Copy, Clone)]
struct Point {
    col: usize,
    row: usize,
}

impl From<Vec<usize>> for Point {
    fn from(value: Vec<usize>) -> Self {
        Self {
            col: value[0],
            row: value[1],
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut ans = 0;
    let points: Vec<Point> = input
        .lines()
        .map(|line| get_nums_usize(line).into())
        .collect();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let d_row = points[i].row.abs_diff(points[j].row) + 1;
            let d_col = points[i].col.abs_diff(points[j].col) + 1;
            ans = ans.max(d_row * d_col);
        }
    }
    Some(ans)
}
fn area(x: usize, y: usize, u: usize, v: usize) -> usize {
    (u - x + 1) * (v - y + 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    // Read and parse input
    let red: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let coords: Vec<usize> = line
                .split(',')
                .map(|s| s.trim().parse().expect("Invalid number"))
                .collect();
            (coords[0], coords[1])
        })
        .collect();

    // Generate all rectangle diagonals
    let mut diags: Vec<(usize, usize, usize, usize)> = Vec::new();
    for i in 0..red.len() {
        for j in i + 1..red.len() {
            let (x1, y1) = red[i];
            let (x2, y2) = red[j];
            let (x_min, y_min) = (x1.min(x2), y1.min(y2));
            let (x_max, y_max) = (x1.max(x2), y1.max(y2));
            diags.push((x_min, y_min, x_max, y_max));
        }
    }

    // Sort diags by area in descending order
    diags.sort_by(|a, b| area(b.0, b.1, b.2, b.3).cmp(&area(a.0, a.1, a.2, a.3)));


    // Generate lines (pairwise) - including wrap-around to first element
    let mut lines: Vec<(usize, usize, usize, usize)> = Vec::new();
    for i in 0..red.len() {
        let (x1, y1) = red[i];
        let (x2, y2) = red[(i + 1) % red.len()]; // Wrap around
        let (x_min, y_min) = (x1.min(x2), y1.min(y2));
        let (x_max, y_max) = (x1.max(x2), y1.max(y2));
        lines.push((x_min, y_min, x_max, y_max));
    }

    // Find the first diagonal that doesn't intersect any line
    for &(x, y, u, v) in &diags {
        let mut intersects = false;

        for &(p, q, r, s) in &lines {
            if p < u && q < v && r > x && s > y {
                intersects = true;
                break;
            }
        }

        if !intersects {
           return Some(area(x, y, u, v));
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
