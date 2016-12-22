use super::Scrambler;
use regex::Regex;

pub struct Rotate(pub Dir, pub usize);

impl Scrambler for Rotate {
    fn scramble(&self, arr: &mut [char]) {
        for _ in 0..self.1 {
            rotate(arr, self.0);
        }
    }
}

fn rotate(arr: &mut [char], dir: Dir) {
    match dir {
            Dir::Left => {
                let first = arr[0];
                for i in 1..arr.len() {
                    arr[i-1] = arr[i];
                }
                if let Some(last) = arr.last_mut() {
                    *last = first;
                }
            }
            Dir::Right => {
                let last = arr.last().cloned();
                for i in 1..arr.len() {
                    let i = arr.len() - 1 - i;
                    arr[i+1] = arr[i];
                }
                if let Some(last) = last {
                    arr[0] = last;
                }
            }
        }
}

#[derive(Copy,Clone)]
pub enum Dir {
    Left,
    Right,
}

impl Dir {
    pub fn from_str(s: &str) -> Self {
        match s {
            "left" => Dir::Left,
            "right" => Dir::Right,
            _ => panic!("unknown dir: {}", s)
        }
    }
}

impl Rotate {
    pub fn from_str(s: &str) -> Self {
        let re = Regex::new(r"rotate (left|right) (\d+) steps?").unwrap();
        let c = re.captures(s).unwrap();
        let dir = Dir::from_str(&c[1]);

        Rotate(dir, c[2].parse().unwrap())
    }
}

#[test]
fn test_rotate() {
    assert_eq!(Rotate(Dir::Left, 1).scramble_str("abcde"), "bcdea");
}
