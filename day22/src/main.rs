extern crate regex;
use std::collections::HashMap;

mod node;
use node::Node;

fn main() {
    let input = include_str!("../input.txt");
    let nodes = input.lines().skip(2)
        .map(Node::from_str)
        .map(|node| ((node.x, node.y), node))
        .collect::<HashMap<_,_>>();

    let width = nodes.keys().map(|&(x,_)| x).max().unwrap();
    let height = nodes.keys().map(|&(_,y)| y).max().unwrap();
    let viable_pairs = nodes.values().flat_map(|a|
        nodes.values().filter(move |b|
               a.used != 0
            && a != *b
            && a.used <= b.available
        )
    ).count();

    println!("Grid: {}x{}", width, height);
    println!("Viable pairs: {}", viable_pairs);
}
