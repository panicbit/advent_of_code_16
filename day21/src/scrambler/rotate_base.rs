use super::{Scrambler,Rotate};
use super::rotate::Dir::Right;
use regex::Regex;

pub struct RotateBase(char);

impl super::Scrambler for RotateBase {
    fn scramble(&self, arr: &mut [char]) {
        let index = arr.iter().enumerate().find(|&(_, &elem)| elem == self.0)
            .map(|(index,_)| index);
        if let Some(index) = index {
            Rotate(Right, index+1).scramble(arr);
            if index >= 4 {
                Rotate(Right, 1).scramble(arr);
            }
        }
    }
}

impl RotateBase {
    pub fn from_str(s: &str) -> Self {
        let re = Regex::new(r"rotate based on position of letter ([a-z])").unwrap();
        let c = re.captures(s).unwrap();
        let letter = c[1].chars().next().unwrap();

        RotateBase(letter)
    }
}

#[test]
fn test_rotate_base() {
    assert_eq!(RotateBase('b').scramble_str("abdec"), "ecabd");
    assert_eq!(RotateBase('d').scramble_str("ecabd"), "decab");
}
