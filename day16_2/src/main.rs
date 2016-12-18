extern crate itertools;

mod dragon;
use dragon::Dragon;

use itertools::Itertools;

fn main() {
    let input = "10011111011011001";
    let dragon = Dragon::new(input, 35651584);
    let chars = dragon.chars().take(35651584);
    let checksum = checksum(chars);
    println!("{}", checksum);
}

fn checksum<I>(iter: I) -> String where I: Iterator<Item=char> {
    let mut sum: String = iter.collect();
    let mut len = sum.chars().count();

    while len % 2 == 0 && len != 0 {
        sum = sum.chars()
            .chunks(2).into_iter()
            .map(|mut pair| pair.next() == pair.next())
            .map(|is_same| if is_same { '1' } else { '0' })
            .collect();

        len = sum.chars().count();
    }

    sum
}
