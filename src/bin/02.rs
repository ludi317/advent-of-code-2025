use advent_of_code::get_nums_usize;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let nums = get_nums_usize(input);
    let mut i = 0;
    let mut ans = 0;
    while i < nums.len() {
        let left = nums[i];
        let right = nums[i + 1];
        let left_num_digits = num_digits(left);
        let mut a;
        let mut left_mask = 10usize.pow((left_num_digits / 2) as u32);
        if left_num_digits % 2 == 0 {
            a = left / left_mask;
            if a < left % left_mask {
                a += 1;
            }
        } else {
            a = left_mask;
            left_mask *= 10;
        }

        let mut b;
        let right_num_digits = num_digits(right);
        let mut right_mask = 10usize.pow((right_num_digits / 2) as u32);
        if right_num_digits % 2 == 0 {
            b = right / right_mask;
            if b > right % right_mask {
                b -= 1
            }
        } else {
            b = right_mask - 1;
        }
        if a <= b {
            if left_mask != right_mask {
                panic!("left_mask != right_mask")
            }
            // sum of [a, b]
            let n = b - a + 1;
            let sum = (n * (a + b)) / 2;
            ans += sum * (1 + left_mask);
        }

        i += 2;
    }

    Some(ans)
}

pub fn num_digits(x: usize) -> usize {
    x.ilog10() as usize + 1
}

pub fn part_two(input: &str) -> Option<usize> {
    let nums = get_nums_usize(input);
    let mut i = 0;
    let mut ans = 0;
    while i < nums.len() {
        let left = nums[i];
        let right = nums[i + 1];
        for j in left..=right {
            let str_j = j.to_string();
            for num_reps in 2..=str_j.len() {
                if str_j.len() % num_reps == 0 {
                    let rep_len = str_j.len() / num_reps;
                    let part = &str_j[0..rep_len];
                    if str_j == part.repeat(num_reps) {
                        ans += j;
                        break;
                    }
                }
            }
        }
        i += 2;
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
