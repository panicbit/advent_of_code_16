extern crate itertools;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let names = input.lines()
        .map(|line| line.trim().parse::<Room>().unwrap())
        .find(|room| room.decrypted_name().contains("northpole"))
        .unwrap();

    println!("{}", names.id());
}

#[derive(Debug)]
struct Room {
    name: String,
    id: u32,
    checksum: String,
}

impl Room {
    fn decrypted_name(&self) -> String {
        self.name.split('-')
            .map(|word| word.chars()
                .map(|c| rotate(c, self.id()))
                .collect::<String>()
            )
            .join(" ")
    }

    fn id(&self) -> u32 {
        self.id
    }
}

fn rotate(ch: char, n: u32) -> char {
    let a = b'a' as u32;
    let ch = ch as u32 - a;
    let ch = (ch + n) % 26;
    (ch + a) as u8 as char
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
