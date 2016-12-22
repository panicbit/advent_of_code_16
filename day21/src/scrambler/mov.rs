use super::Scrambler;
use super::rotate::{Rotate, Dir};
use regex::Regex;

pub struct Move(pub usize, pub usize);

impl Scrambler for Move {
    fn scramble(&self, arr: &mut [char]) {
        if self.0 < self.1 {
            Rotate(Dir::Left, 1).scramble(&mut arr[self.0..self.1+1]);
        } else {
            Rotate(Dir::Right, 1).scramble(&mut arr[self.1..self.0+1]);
        }
    }
}

impl Move {
    pub fn from_str(s: &str) -> Self {
        let re = Regex::new(r"move position (\d+) to position (\d+)").unwrap();
        let c = re.captures(s).unwrap();

        Move(c[1].parse().unwrap(), c[2].parse().unwrap())
    }
}

#[test]
fn test_move() {
    assert_eq!(Move(1, 4).scramble_str("bcdea"), "bdeac");
    assert_eq!(Move(3, 0).scramble_str("bdeac"), "abdec");
}
