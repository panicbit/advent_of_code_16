fn main() {
    let input = include_str!("../input.txt");
    let n = input.lines().map(|line|
        line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>()
    )
    .filter(|t| is_triangle(&t))
    .count();

    println!("{}", n);
}

fn is_triangle(t: &[u32]) -> bool {
       t.len() == 3
    && t[0] + t[1] > t[2]
    && t[1] + t[2] > t[0]
    && t[2] + t[0] > t[1]
}
