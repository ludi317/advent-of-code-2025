use advent_of_code::{Point, UnionFind, get_nums_usize};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let points: Vec<Point> = input
        .lines()
        .map(|line| get_nums_usize(line).into())
        .collect();
    let mut heap = BinaryHeap::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = points[i].dist(&points[j]);
            heap.push(Reverse((distance, i, j)));
        }
    }
    let mut uf = UnionFind::new(points.len());
    // let cnt = 1000;
    let cnt = 10;
    for _ in 0..cnt {
        let Reverse((_, i, j)) = heap.pop().unwrap();
        uf.union(i, j);
    }
    let mut sizes = uf.get_size().clone();
    sizes.sort();
    let ans = sizes.iter().rev().take(3).product();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let points: Vec<Point> = input
        .lines()
        .map(|line| get_nums_usize(line).into())
        .collect();
    let mut heap = BinaryHeap::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = points[i].dist(&points[j]);
            heap.push(Reverse((distance, i, j)));
        }
    }
    let mut cnt = 0;
    let mut uf = UnionFind::new(points.len());
    loop {
        let Reverse((_, i, j)) = heap.pop().unwrap();
        if uf.union(i, j) {
            cnt += 1
        }
        if cnt == points.len() - 1 {
            return Some(points[i].x * points[j].x);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
