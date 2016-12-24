pub mod linked;
pub mod swap;
pub mod rotate;
pub mod rotate_base;
pub mod reverse;
pub mod mov;

pub use self::linked::Linked;
pub use self::swap::{SwapPos, SwapVal};
pub use self::rotate::Rotate;
pub use self::rotate_base::RotateBase;
pub use self::reverse::Reverse;
pub use self::mov::Move;

use regex::Regex;

pub trait Scrambler {
    fn scramble(&self, arr: &mut [char]);
    fn unscramble(&self, arr: &mut [char]);
    fn then<T: Scrambler>(self, scrambler: T) -> Linked<Self, T> where
        Self: Sized
    {
        Linked(self, scrambler)
    }
    fn scramble_str(&self, str: &str) -> String {
        let mut str = str.chars().collect::<Vec<char>>();
        self.scramble(&mut str);
        str.into_iter().collect()
    }
    fn unscramble_str(&self, str: &str) -> String {
        let mut str = str.chars().collect::<Vec<char>>();
        self.unscramble(&mut str);
        str.into_iter().collect()
    }
}

impl Scrambler {
    pub fn from_str(s: &str) -> Box<Scrambler> {
        let re = Regex::new("([^ ]+ [^ ]+).*").unwrap();
        let c = re.captures(s).unwrap();

        match &c[1] {
            "swap position" => Box::new(SwapPos::from_str(s)),
            "swap letter" => Box::new(SwapVal::from_str(s)),
            "rotate left" => Box::new(Rotate::from_str(s)),
            "rotate right" => Box::new(Rotate::from_str(s)),
            "rotate based" => Box::new(RotateBase::from_str(s)),
            "reverse positions" => Box::new(Reverse::from_str(s)),
            "move position" => Box::new(Move::from_str(s)),
            _ => panic!("Unknown scramble rule: {}", s)
        }
    }
}

pub struct Noop;
impl Scrambler for Noop {
    fn scramble(&self, _arr: &mut [char]) {}
    fn unscramble(&self, _arr: &mut [char]) {}
}

impl Scrambler for Box<Scrambler> {
    fn scramble(&self, arr: &mut [char]) {
        (**self).scramble(arr);
    }
    fn unscramble(&self, arr: &mut [char]) {
        (**self).unscramble(arr);
    }
}
