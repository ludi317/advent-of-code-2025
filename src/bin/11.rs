use std::collections::{HashMap};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let colon_pos = line.find(':').unwrap();
        let key = line[..colon_pos].to_string();
        let values = line[colon_pos+2..].split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(key, values);
    }
    let mut cache: HashMap<String, usize> = HashMap::new() ;
    let ans = dfs("you".to_string(), "out", &mut graph, &mut cache);
    
    Some(ans)
}

fn dfs(node: String, target: &str, graph: &HashMap<String, Vec<String>>, cache: &mut HashMap<String, usize>) -> usize {
    if node == target {
        return 1;
    }
    match cache.get(&node) {
        Some(val) => return *val,
        _ => {},
    }

    let mut ans = 0;
    for next in graph.get(&node).unwrap() {
       ans += dfs(next.clone(), target, graph, cache); 
    }
    cache.insert(node, ans);
    ans
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let colon_pos = line.find(':').unwrap();
        let key = line[..colon_pos].to_string();
        let values = line[colon_pos+2..].split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(key, values);
    }
    let mut cache: HashMap<String, usize> = HashMap::new() ;
    let ans = dfs2("svr".to_string(), "out", &mut graph, &mut cache, 0);

    Some(ans)

}

fn dfs2(node: String, target: &str, graph: &HashMap<String, Vec<String>>, cache: &mut HashMap<String, usize>, mut seen: u8) -> usize {
    if node == target {
        if seen == 3 {
            return 1;
        }
        return 0;
    }
    let key = format!("{}{}", node, seen);
    match cache.get(&key) {
        Some(val) => return *val,
        _ => {},
    }
    
    if node == "fft" {
        seen |= 1;
    } else if node == "dac" {
        seen |= 2;
    }

    let mut ans = 0;
    for next in graph.get(&node).unwrap() {
        ans += dfs2(next.clone(), target, graph, cache, seen);
    }
    cache.insert(key, ans);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
