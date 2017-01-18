extern crate itertools;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let lines = input.lines().collect::<Vec<_>>();
    let n_cols = lines.first().unwrap().chars().count();
    
    for col in 0..n_cols {
        let mut chars = lines.iter()
            .map(|line| line.chars().nth(col).unwrap())
            .collect::<Vec<_>>();
            
        chars.sort();

        let least_frequent_char = chars.iter()
            .group_by(|c| *c).into_iter()
            .map(|(key, elems)| (key, elems.count()))
            .min_by_key(|kv| kv.1)
            .unwrap().0;
        print!("{}", least_frequent_char);
    }
    
    println!("");
}
