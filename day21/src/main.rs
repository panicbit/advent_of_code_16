extern crate regex;

mod scrambler;

use scrambler::{Scrambler, Noop};

fn main() {
    let input = include_str!("../input.txt");
    let mut scrambler: Box<Scrambler> = Box::new(Noop);

    for line in input.lines() {
        let next_scrambler = Scrambler::from_str(line.trim());
        scrambler = Box::new(scrambler.then(next_scrambler));
    }

    let pass = scrambler.scramble_str("abcdefgh");
    println!("{}", pass);
}
