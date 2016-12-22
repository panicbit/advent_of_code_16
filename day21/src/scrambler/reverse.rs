use super::Scrambler;
use std::cmp::min;
use regex::Regex;

pub struct Reverse(pub usize, pub usize);

impl Scrambler for Reverse {
    fn scramble(&self, arr: &mut [char]) {
        if arr.len() == 0 {
            return;
        }
        let from = min(self.0, arr.len());
        let to = min(self.1+1, arr.len());
        
        arr[from..to].reverse();
    }
}

impl Reverse {
    pub fn from_str(s: &str) -> Self {
        let re = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
        let c = re.captures(s).unwrap();

        Reverse(c[1].parse().unwrap(), c[2].parse().unwrap())
    }
}

#[test]
fn test_reverse() {
    assert_eq!(Reverse(0, 4).scramble_str("edcba"), "abcde");
}
