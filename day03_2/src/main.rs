extern crate itertools;
extern crate multizip;
use itertools::Itertools;
use multizip::zip3;

fn main() {
    let input = include_str!("../input.txt");

    let n = input.lines()
        .chunks(3).into_iter()
        .flat_map(|mut lines|
            zip3(
                lines.next().unwrap().split_whitespace().map(|n| n.parse::<u32>().unwrap()),
                lines.next().unwrap().split_whitespace().map(|n| n.parse::<u32>().unwrap()),
                lines.next().unwrap().split_whitespace().map(|n| n.parse::<u32>().unwrap()),
            )
        )
        .filter(is_triangle)
        .count();

    println!("{:?}", n);
}

fn is_triangle(&(a, b, c): &(u32, u32, u32)) -> bool {
       a + b > c
    && b + c > a
    && c + a > b
}
