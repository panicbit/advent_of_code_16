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
        x * 2_u32.pow(n) + geo_series(n-1, 2)
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
pub enum DragonIter<'a> {
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

fn geo_series(to: u32, k: u32) -> u32 {
    (k.pow(to+1) - 1) / (k - 1)
}

fn conv(ch: char) -> char {
    match ch {
        '0' => '1',
        '1' => '0',
        _   => ch,
    }
}
