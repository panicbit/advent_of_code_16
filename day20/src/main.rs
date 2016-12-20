extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::{min,max};

fn main() {
    let input = include_str!("../input.txt");
    let mut blacklist = HashMap::new();

    for line in input.lines() {
        let range = ip_range_from_str(line);
        let overlaps = blacklist.iter()
            .map(|(&from, &to)| (from, to))
            .filter(|&range2| do_overlap(range, range2))
            .collect::<Vec<_>>();

        for overlap in &overlaps {
            blacklist.remove(&overlap.0);
        }

        let range = overlaps.into_iter().fold(range, merge_ranges);
        blacklist.insert(range.0, range.1);
    }

    let mut ip = 0;
    while ip < u32::max_value() {
        match blacklist.get(&ip) {
            Some(to) => ip = to + 1,
            None => break,
        }
    }

    println!("{}", ip);
}

fn ip_range_from_str(s: &str) -> (u32,u32) {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let c = re.captures(s.trim()).unwrap();
    (
        c[1].parse().unwrap(),
        c[2].parse().unwrap(),
    )
}

fn do_overlap(x: (u32, u32), y: (u32, u32)) -> bool {
    !(x.1 < y.0 || x.0 > y.1)
}

fn merge_ranges(x: (u32, u32), y: (u32, u32)) -> (u32, u32) {
    (
        min(x.0, y.0),
        max(x.1, y.1),
    )
}
