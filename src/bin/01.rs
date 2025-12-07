advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<isize> {
    let mut dial = 50;
    let mut ans = 0;
    for line in input.lines() {
        let (sign, number_str) = line.split_at(1);
        let sign = match sign {
            "L" => -1,
            "R" => 1,
            _ => unreachable!(),
        };
        let num: isize = number_str.parse().unwrap();
        dial += sign * num;
        dial %= 100;
        if dial == 0 {
            ans += 1;
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut dial = 50;
    let mut ans = 0;
    let n = 100;
    for line in input.lines() {
        let (sign, number_str) = line.split_at(1);
        let sign = match sign {
            "L" => -1,
            "R" => 1,
            _ => unreachable!(),
        };
        let num: isize = number_str.parse().unwrap();
        let (a, b) = if sign > 0 {
            (dial + 1, dial + num)
        } else {
            (dial - num, dial - 1)
        };
        // count multiples of n in [a, b]
        ans += b.div_euclid(n) - (a-1).div_euclid(n);
        dial += sign * num;
        dial %= 100;
    }
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
        assert_eq!(result, Some(6));
    }
}
