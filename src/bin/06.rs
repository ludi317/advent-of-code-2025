use advent_of_code::get_nums_usize;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let mut ans = 0;
    let mut all_nums: Vec<Vec<usize>> = vec![];
    let mut ops: Vec<char> = vec![];
    for line in input.lines() {
        let nums = get_nums_usize(line);
        if nums.len() == 0 {
            ops = line.chars().filter(|c| *c != ' ').collect();
            break
        }
        all_nums.push(nums);
    }
    for (i, op) in ops.iter().enumerate() {
        match op {
            '+' => {
                for vec in &all_nums {
                    ans += vec[i];
                }
            }
            '*' => {
                let mut sub_ans = 1;
                for vec in &all_nums {
                    sub_ans *= vec[i];
                }
                ans += sub_ans;
            }
            _ => unreachable!()
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut ans = 0;
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let length = lines.len();
    let mut i = 0;
    let ops: &Vec<char> = &lines[length - 1];
    while i < ops.len() {
        if ops[i] != ' ' {
            let mut sub_ans: usize;
            let mult: bool;
            if ops[i] == '*' {
                sub_ans = 1;
                mult = true;
            } else {
                sub_ans = 0;
                mult = false;
            }

            while i < ops.len() {
                let mut n = 0usize;
                for j in 0..length - 1 {
                    match &lines[j][i].to_digit(10) {
                        None => {}
                        Some(d) => {
                            n *= 10;
                            n += *d as usize;
                        }
                    }
                }
                // println!("n {}", n);

                if n == 0 {
                    break
                }
                if mult {
                    sub_ans *= n;
                } else {
                    sub_ans += n;
                }
                i += 1;
            }
            ans += sub_ans;
            i += 1;
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
