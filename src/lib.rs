pub mod template;

// Use this file to add helper functions and additional modules.

pub fn get_nums_usize(s: &str) -> Vec<usize> {
    let mut nums: Vec<usize> = vec![];
    let mut num: usize = 0;
    let mut num_found = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = 10 * num + (c.to_digit(10).unwrap() as usize);
            num_found = true;
        } else if num_found {
            nums.push(num);
            num  = 0;
            num_found = false;
        }
    }
    if num_found {
        nums.push(num);
    }
    nums
}

// parses a sequence of numbers
pub fn get_nums(s: &str) -> Vec<isize> {
    let mut nums: Vec<isize> = vec![];
    let mut num: isize = 0;
    let mut sign = 1;
    let mut num_found = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = 10 * num + c.to_digit(10).unwrap() as isize;
            num_found = true;
        } else if c == '-' {
            sign = -1;
        } else if num_found {
            nums.push(num * sign);
            num = 0;
            sign = 1;
            num_found = false;
        }
    }
    if num_found {
        nums.push(num * sign);
    }
    nums
}



// parses a sequence of numbers
pub fn get_nums_f64(s: &str) -> Vec<f64> {
    let mut nums: Vec<f64> = vec![];
    let mut num: f64 = 0.;
    let mut sign = 1f64;
    let mut num_found = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = 10. * num + c.to_digit(10).unwrap() as f64;
            num_found = true;
        } else if c == '-' {
            sign = -1f64;
        } else if num_found {
            nums.push(num * sign);
            num = 0.;
            sign = 1f64;
            num_found = false;
        }
    }
    if num_found {
        nums.push(num * sign);
    }
    nums
}

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    y: usize,
    z: usize,
}

impl Point {
    pub fn dist(&self, other: &Point) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        dx * dx + dy * dy + dz * dz
    }
}

impl From<Vec<usize>> for Point {
    fn from(vec: Vec<usize>) -> Self {
        Point {
            x: vec[0],
            y: vec[1],
            z: vec[2],
        }
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n]
        }
    } 
    
    pub fn get_leader(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.get_leader(self.parent[x]);
        }
        self.parent[x]
    }
    
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let x_leader = self.get_leader(x);
        let y_leader = self.get_leader(y);
        
        if x_leader != y_leader {
            self.size[x_leader] += self.size[y_leader];
            self.size[y_leader] = 0;
            
            self.parent[y_leader] = x_leader;
            return true;
        }
        false
    }
    
    pub fn get_size(&self) -> &Vec<usize> {
        &self.size
    }
}