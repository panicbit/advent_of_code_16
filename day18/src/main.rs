extern crate iterslide;
use iterslide::SlideIterator;
use std::iter::once;
use std::mem::swap;

fn main() {
    let input = ".^^.^^^..^.^..^.^^.^^^^.^^.^^...^..^...^^^..^^...^..^^^^^^..^.^^^..^.^^^^.^^^.^...^^^.^^.^^^.^.^^.^.";
    let n_rows = 40;
    let mut current: Vec<_> = input.chars().map(to_tile).collect();
    let mut next = current.clone();
    let mut n_free = count_safe(&current);

    for _ in 1..n_rows {
        next.clear();
        next.extend(
            once(Safe)
            .chain(current.iter().cloned())
            .chain(once(Safe))
            .slide(3)
            .map(rule)
        );
        n_free += count_safe(&next);
        swap(&mut current, &mut next);
    }

    println!("{}", n_free);
}

fn rule(t: Vec<Tile>) -> Tile {
    match (t[0], t[1], t[2]) {
          (Trap, Trap, Safe)
        | (Safe, Trap, Trap)
        | (Trap, Safe, Safe)
        | (Safe, Safe, Trap) => Trap,
        _ => Safe
    }
}

fn count_safe(row: &[Tile]) -> usize {
    row.iter().filter(|&&t| t == Safe).count()
}

fn to_tile(ch: char) -> Tile {
    match ch {
        '^' => Trap,
        _ => Safe,
    }
}

#[derive(Copy,Clone,PartialEq,Debug)]
enum Tile {
    Safe,
    Trap,
}
use self::Tile::*;
