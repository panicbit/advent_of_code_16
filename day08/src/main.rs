#[macro_use(s)]
extern crate ndarray;
extern crate regex;
use ndarray::{Array2,Ix1,Ix2,ArrayViewMut1};
use regex::Regex;

#[cfg(test)]
mod test;

fn main() {
    let input = include_str!("../input.txt");
    let mut lcd = Lcd::new();

    for line in input.lines() {
        let op = Op::from_str(line.trim());
        lcd.modify(op);
    }

    println!("{}", lcd.lit_pixels());
}

#[derive(Debug)]
pub struct Lcd {
    field: Array2<bool>,
}

impl Lcd {
    fn new() -> Self {
        Lcd {
            field: Array2::from_elem((6, 50), false)
        }
    }
    fn modify(&mut self, op: Op) {
        match op {
            Op::Rect { w, h } => {
                for x in 0..w {
                    for y in 0..h {
                        self.field[Ix2(y,x)] = true;
                    }
                }
            },
            Op::RotCol { x, by } => {
                for _ in 0..by {
                    rotate(self.field.column_mut(x))
                }
            }
            Op::RotRow { y, by } => {
                for _ in 0..by {
                    rotate(self.field.row_mut(y))
                }
            }
        }
    }
    fn lit_pixels(&self) -> usize {
        self.field.iter()
            .filter(|is_on| **is_on).count()
    }
}

fn rotate<T: Copy + ::std::fmt::Debug>(mut arr: ArrayViewMut1<T>) {
    // Rotate right by rotating left on reversed array
    let mut arr = arr.slice_mut(s![..;-1]);
    let first = *arr.iter().next().unwrap();
    for i in 1..arr.len() {
        arr[Ix1(i-1)] = arr[Ix1(i)];
    }
    *arr.iter_mut().last().unwrap() = first;
}

#[derive(Debug,PartialEq,Eq)]
pub enum Op {
    Rect { w: usize, h: usize },
    RotRow { y: usize, by: usize },
    RotCol { x: usize, by: usize },
}

impl Op {
    fn from_str(s: &str) -> Self {
        let re = Regex::new(r"([^ ]+) (.+)").unwrap();
        let c = re.captures(s).unwrap();
        let op = &c[1];
        let args = &c[2];

        match op {
            "rect" => {
                let re = Regex::new(r"(\d+)x(\d+)").unwrap();
                let c = re.captures(args).unwrap();
                Op::Rect {
                    w: c[1].parse().unwrap(),
                    h: c[2].parse().unwrap(),
                }
            }
            "rotate" => {
                let re = Regex::new(r"(column|row) (x|y)=(\d+) by (\d+)").unwrap();
                let c = re.captures(args).unwrap();
                let direction = &c[1];
                let index = c[3].parse().unwrap();
                let amount = c[4].parse().unwrap();


                match direction {
                    "row" => Op::RotRow { y: index, by: amount },
                    "column" => Op::RotCol { x: index, by: amount },
                    _ => unreachable!()
                }
            }
            _ => panic!("Unknown op {}", op)
        }
    }
}
