advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    let mut ans = 0;
    for line in input.lines() {
        let mut first = 0usize;
        let mut second = 0usize;
        let chars: Vec<char> = line.chars().collect();
        let length = chars.len();
        for (i, c) in chars.into_iter().enumerate() {
            let digit = c.to_digit(10).unwrap() as usize;

            if i == 0 {
                first = first.max(digit);
            } else if i == length - 1 {
                second = second.max(digit);
            } else {
                if digit > first {
                    first = digit;
                    second = 0;
                } else if digit > second {
                    second = second.max(digit);
                }
            }
        }
        ans += first * 10 + second;
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let n = 12;
    let mut ans = 0;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let length = chars.len();
        let mut st: Vec<usize> = vec![];
        for (i, c) in chars.into_iter().enumerate() {
            let digit = c.to_digit(10).unwrap() as usize;
            while st.len() > 0 && digit > st[st.len() -1] && (length - i) + st.len() - 1 >= n {
                st.pop();
            }
            st.push(digit);
        }
        let mut pow = 1;
        let mut sub_ans = 0;
        for i in (0..n).rev() {
            sub_ans += st[i] * pow;
            pow *= 10;
        }
        ans += sub_ans;
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
