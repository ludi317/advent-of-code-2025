use advent_of_code::get_nums_usize;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let mut ans = 0;
    let mut lines = input.lines();
    let mut ranges: Vec<Vec<usize>> = vec![];
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let nums = get_nums_usize(line);
        ranges.push(vec![nums[0], nums[1]]);
    }

    for line in lines {
        let id = get_nums_usize(line)[0];
        for range in &ranges {
            if id >= range[0] && id <= range[1] {
                ans += 1;
                break;
            }
        }

    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut ans = 0;
    let lines = input.lines();
    let mut ranges: Vec<Vec<usize>> = vec![];
    for line in lines {
        if line.is_empty() {
            break;
        }
        let nums = get_nums_usize(line);
        ranges.push(vec![nums[0], nums[1]]);
    }
    ranges.sort_by_key(|r| r[1]);
    let mut agg = ranges[0].clone();
    for r in &ranges[1..] {
        if r[0] <= agg[1] {
            agg[0] = agg[0].min(r[0]);
            agg[1] = agg[1].max(r[1]);
        } else {
            ans += agg[1] - agg[0] + 1;
            agg = r.clone();
        }
    }
    ans += agg[1] - agg[0] + 1;
    
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
