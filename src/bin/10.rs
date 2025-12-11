use arrayvec::ArrayVec;
use microlp::{ComparisonOp, LinearExpr, OptimizationDirection, Problem};

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

#[derive(Debug, Default)]
struct LineData {
    target: usize,
    buttons: Vec<usize>,
    buttons_vec: Vec<Vec<usize>>,
    joltage: Vec<usize>,
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

fn parse_joltage(input: &str) -> IResult<&str, Vec<usize>> {
    // Parse {1,2,3} -> vec![1,2,3]
    delimited(
        char('{'),
        separated_list1(
            char(','),
            map(digit1, |s: &str| s.parse::<usize>().unwrap()),
        ),
        char('}'),
    )(input)
}

fn parse_button_mask(input: &str) -> IResult<&str, usize> {
    // Parse (1,2,3) -> bitmask with bits 1,2,3 set
    delimited(
        char('('),
        map(
            separated_list1(
                char(','),
                map(digit1, |s: &str| s.parse::<usize>().unwrap()),
            ),
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

fn parse_button_indices(input: &str) -> IResult<&str, Vec<usize>> {
    // Parse (1,2,3) -> vec![1, 2, 3]
    delimited(
        char('('),
        separated_list1(
            char(','),
            map(digit1, |s: &str| s.parse::<usize>().unwrap()),
        ),
        char(')'),
    )(input)
}

fn parse_line_1(input: &str) -> IResult<&str, LineData> {
    let (input, target) = parse_pattern(input)?;
    let (input, _) = nom::character::complete::space1(input)?;
    let (input, buttons) = many1(terminated(
        parse_button_mask,
        nom::character::complete::space0,
    ))(input)?;

    Ok((
        input,
        LineData {
            target,
            buttons,
            ..Default::default()
        },
    ))
}

fn parse_line_2(input: &str) -> IResult<&str, LineData> {
    let (input, _) = parse_pattern(input)?;
    let (input, _) = nom::character::complete::space1(input)?;
    let (input, buttons_vec) = many1(terminated(
        parse_button_indices,
        nom::character::complete::space0,
    ))(input)?;
    let (input, joltage) = parse_joltage(input)?;

    Ok((
        input,
        LineData {
            buttons_vec,
            joltage,
            ..Default::default()
        },
    ))
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut ans = 0;
    for line in input.lines() {
        let res = parse_line_1(line).unwrap();
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


pub fn part_two(input: &str) -> Option<usize> {
      /*
-    Let n = target length
-    let b = # of buttons
-    
-    1. Create a matrix A (n x b) where each col is 1 group of buttons. set the row to 1 if the idx is present in the group, otherwise 0.
-    for the first line in the example, matrix A looks like:
-
-    (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
-
-    0 0 0 0 1 1
-    0 1 0 0 0 1
-    0 0 1 1 1 0
-    1 1 0 1 0 0
-    
-    2. Target b is a straightforward col vector, 
-     
-     {3,5,4,7}
-     
-     3 
-     5 
-     4 
-     7
-     
-     3. Solve for Ax = b, where the sum of x is minimized and x is all positive integers. 
-     x_i represents the number of time button group i was pressed.
-     
-     */
    let mut ans = 0usize;
    for line in input.lines() {
        let res = parse_line_2(line).unwrap();
        let target = res.1.joltage;
        let button_groups = res.1.buttons_vec;

        // 1. Create the ILP problem (minimize total presses)
        let mut problem = Problem::new(OptimizationDirection::Minimize);

        // 2. Find an upper bound for button presses (max target value)
        let max_presses = *target.iter().max().unwrap() as i32;

        // 3. Create integer variables for each button (press counts)
        let vars: Vec<microlp::Variable> = (0..button_groups.len())
            .map(|_| problem.add_integer_var(1.0, (0, max_presses)))
            .collect();

        // 4. Add constraints: For each position, sum of relevant button presses == target value
        for (idx, &target_val) in target.iter().enumerate() {
            // Build the expression for this position
            let mut expr = LinearExpr::empty();

            // Check each button to see if it affects this position
            for (btn_idx, button_indices) in button_groups.iter().enumerate() {
                if button_indices.contains(&idx) {
                    expr.add(vars[btn_idx], 1.0);
                }
            }
            // Add the equality constraint
            problem.add_constraint(expr, ComparisonOp::Eq, target_val as f64);
        }

        // 5. Solve the problem and add the result to the total
        ans += problem.solve().unwrap().objective().round() as usize;
    }

    Some(ans)
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
        assert_eq!(result, Some(33));
    }
}
