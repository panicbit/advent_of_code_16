use std::ops::Index;

use std::iter::{DoubleEndedIterator, once};
use std::str::Chars;
use std::iter::{Map, Rev, Once, Chain};

pub struct Dragon {
    iteration: u32,
    data: String,
}

impl Dragon {
    pub fn new(init: &str, min_len: u32) -> Dragon {
        let mut dragon = Dragon {
            iteration: 1,
            data: init.to_string(),
        };

        while dragon.len() < min_len {
            dragon.iteration += 1;
        }

        dragon
    }

    pub fn len(&self) -> u32 {
        let x = self.data.len() as u32;
        let n = self.iteration;
        x * 2_u32.pow(n) + geo_series(0, n-1, 2)
    }

    pub fn chars<'a>(&'a self) -> DragonIter<'a> {
        let mut a = DragonIter::Head(self.data.chars());

        for _ in 0..self.iteration {
            let b = Box::new(DragonIter::Tail(Box::new(a.clone()).rev().map(conv)));
            a = DragonIter::Glue(Box::new(a).chain(once('0')).chain(b));
        }

        a
    }
}

#[derive(Clone)]
enum DragonIter<'a> {
    Head(Chars<'a>),
    Tail(Map<Rev<Box<DragonIter<'a>>>, Conv>),
    Glue(Chain<Chain<Box<DragonIter<'a>>, Once<char>>, Box<DragonIter<'a>>>),
}

impl<'a> Iterator for DragonIter<'a> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        match *self {
            DragonIter::Head(ref mut iter) => iter.next(),
            DragonIter::Tail(ref mut iter) => iter.next(),
            DragonIter::Glue(ref mut iter) => iter.next(),
        }
    }
}

impl<'a> DoubleEndedIterator for DragonIter<'a> {
    fn next_back(&mut self) -> Option<char> {
        match *self {
            DragonIter::Head(ref mut iter) => iter.next_back(),
            DragonIter::Tail(ref mut iter) => iter.next_back(),
            DragonIter::Glue(ref mut iter) => iter.next_back(),
        }
    }
}

type Conv = fn (char) -> char;

fn geo_series(from: u32, to: u32, k: u32) -> u32 {
    (k.pow(to+1) - 1) / (k - 1)
}

#[test]
fn test_dragon_len() {
    assert_eq!(Dragon::new("10000", 23).len(), 23);
    assert_eq!(Dragon::new("10000", 47).len(), 47);

    assert_eq!(Dragon::new("10000", 22).len(), 23);
    assert_eq!(Dragon::new("10000", 46).len(), 47);

    assert_eq!(Dragon::new("10000", 24).len(), 47);
    assert_eq!(Dragon::new("10000", 48).len(), 95);
}

#[test]
fn test_dragon() {
    let d = |n| {
        let dragon = Dragon::new("1000", n as u32);
        dragon.chars().collect::<String>()
    };
    assert_eq!(d(0), "100001110");
    assert_eq!(d(9), "100001110");
    assert_eq!(d(10), "1000011100100011110" );
    assert_eq!(d(19), "1000011100100011110" );
    assert_eq!(d(20), "100001110010001111001000011101100011110");
    assert_eq!(d(39), "100001110010001111001000011101100011110");
}

fn conv(ch: char) -> char {
    match ch {
        '0' => '1',
        '1' => '0',
        _   => ch,
    }
}
