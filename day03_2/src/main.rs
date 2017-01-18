extern crate itertools;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    let mut col3 = Vec::new();

    for line in input.lines() {
        let mut values = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
        col1.push(values.next().unwrap());
        col2.push(values.next().unwrap());
        col3.push(values.next().unwrap());
    }

    let n = triangles_in_column(&col1) + triangles_in_column(&col2) + triangles_in_column(&col3);
    println!("{}", n);
}

fn triangles_in_column(col: &[u32]) -> usize {
    col.iter().cloned().chunks(3).into_iter()
    .map(|sides| sides.collect::<Vec<_>>())
    .filter(|triangle| is_triangle(&triangle))
    .count()
}

fn is_triangle(t: &[u32]) -> bool {
       t.len() == 3
    && t[0] + t[1] > t[2]
    && t[1] + t[2] > t[0]
    && t[2] + t[0] > t[1]
}
