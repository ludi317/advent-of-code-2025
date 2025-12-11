use nom::{
    IResult,
    branch::alt,
    character::complete::{char, digit1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
};
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

#[derive(Debug)]
struct LineData {
    target: usize,
    buttons: Vec<usize>,
}

fn parse_pattern(input: &str) -> IResult<&str, usize> {
    delimited(
        char('['),
        map(many1(alt((char('.'), char('#')))), |chars: Vec<char>| {
            let mut bits = 0usize;
            for (i, c) in chars.iter().enumerate() {
                if *c == '#' {
                    bits |= 1 << i;
                }
            }
            bits
        }),
        char(']'),
    )(input)
}

fn parse_button_mask(input: &str) -> IResult<&str, usize> {
    // Parse (1,2,3) -> bitmask with bits 1,2,3 set
    delimited(
        char('('),
        map(
            separated_list1(char(','), map(digit1, |s: &str| s.parse::<usize>().unwrap())),
            |indices: Vec<usize>| {
                let mut mask = 0usize;
                for &idx in &indices {
                    mask |= 1 << idx;
                }
                mask
            },
        ),
        char(')'),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, LineData> {
    let (input, target) = parse_pattern(input)?;
    let (input, _) = nom::character::complete::space1(input)?;
    let (input, buttons) = many1(terminated(
        parse_button_mask,
        nom::character::complete::space0,
    ))(input)?;

    Ok((input, LineData { target, buttons }))
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut ans = 0;
    for line in input.lines() {
        let res = parse_line(line).unwrap();
        let target = res.1.target;
        let buttons = res.1.buttons;
        let mut deq: VecDeque<usize> = VecDeque::new();
        deq.push_back(0);
        let mut level = 0;
        let mut visited: HashSet<usize> = HashSet::new();
        'outer: loop {
            let length = deq.len();
            for _ in 0..length {
                let node = deq.pop_front().unwrap();
                // println!("{:?} {:?}", node, target);
                if node == target {
                    ans += level;
                    break 'outer;
                }
                for b in &buttons {
                    let new_node = node ^ b;
                    if !visited.contains(&new_node) {
                        visited.insert(new_node);
                        deq.push_back(new_node);
                    }
                }
            }
            level += 1;
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
