extern crate itertools;
use itertools::Itertools;
fn main() {
    let input = 3004953;
    let mut elves: Vec<u32> = (1..input+1).collect();

    while elves.len() > 1 {
        let mut to_skip = elves.len() % 2;
        elves = elves
            .into_iter()
            .chunks(2).into_iter()
            .map(|mut pair| pair.nth(0).unwrap())
            .skip(to_skip)
            .collect();
    }

    println!("{:?}", elves[0]);
}
