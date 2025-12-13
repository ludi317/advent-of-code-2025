use advent_of_code::get_nums_usize;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let mut ans = 0;
    for line in input.lines() {
        if line.contains("x") {
            let nums = get_nums_usize(line);
            let capacity = nums[0] * nums[1];
            let used: usize = nums[2..].iter().sum::<usize>() * 9;
            if capacity >= used {
                ans += 1;
            }
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
        // assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
