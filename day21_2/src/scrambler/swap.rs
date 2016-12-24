use scrambler::Scrambler;
use regex::Regex;
pub struct SwapPos(pub usize, pub usize);

impl Scrambler for SwapPos {
    fn scramble(&self, arr: &mut [char]) {
        arr.swap(self.0, self.1);
    }
    fn unscramble(&self, arr: &mut [char]) {
        self.scramble(arr);
    }
}

pub struct SwapVal(pub char, pub char);

impl super::Scrambler for SwapVal {
    fn scramble(&self, arr: &mut [char]) {
        for elem in arr {
            if *elem == self.0 {
                *elem = self.1;
            }
            else if *elem == self.1 {
                *elem = self.0;
            }
        }
    }
    fn unscramble(&self, arr: &mut [char]) {
        self.scramble(arr);
    }
}

impl SwapPos {
    pub fn from_str(s: &str) -> Self {
        let re = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
        let c = re.captures(s).unwrap();

        SwapPos(c[1].parse().unwrap(), c[2].parse().unwrap())
    }
}

impl SwapVal {
    pub fn from_str(s: &str) -> Self {
        let re = Regex::new(r"swap letter ([a-z]) with letter ([a-z])").unwrap();
        let c = re.captures(s).unwrap();
        let ch1 = c[1].chars().next().unwrap();
        let ch2 = c[2].chars().next().unwrap(); 

        SwapVal(ch1, ch2)
    }
}

#[test]
fn test_swap_val() {
    assert_eq!(SwapPos(4, 0).scramble_str("abcde"), "ebcda");
    assert_eq!(SwapVal('d', 'b').scramble_str("ebcda"), "edcba");
}
