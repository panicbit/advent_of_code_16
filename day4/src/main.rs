#![feature(ordering_chaining)]
extern crate itertools;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let rooms: u32 = input.lines()
        .map(|line| line.trim().parse::<Room>().unwrap())
        .filter(Room::is_real)
        .map(|room| room.id())
        .sum();

    println!("{:?}", rooms);
}

#[derive(Debug)]
struct Room {
    name: String,
    id: u32,
    checksum: String,
}

impl Room {
    fn is_real(&self) -> bool {
        let mut chars = self.name.split('-')
            .flat_map(|segment| segment.chars())
            .collect::<Vec<_>>();

        chars.sort();

        let mut occurrences: Vec<(char, usize)> = chars.iter()
            .group_by(|c| *c).into_iter()
            .map(|(c,cs)| (*c, cs.count()))
            .collect();

        occurrences.sort_by(|&(c1, n1), &(c2, n2)|
            n2.cmp(&n1).then(c1.cmp(&c2))
        );

        let checksum: String = occurrences.iter()
            .take(5)
            .map(|&(c,_)| c)
            .collect();

        self.checksum == checksum
    }

    fn id(&self) -> u32 {
        self.id
    }
}

impl ::std::str::FromStr for Room {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, s) = s.split_at(s.len()-11);

        Ok(Room {
            name: name.into(),
            id: s[1..4].parse().unwrap(),
            checksum: s[5..10].into(),
        })
    }
}
