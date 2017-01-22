#![feature(conservative_impl_trait)]
use std::str;

fn main() {
    let input = include_str!("../input.txt");

    let n = input.lines()
        .map(|line| Ipv7::from_str(line.trim()))
        .filter(Ipv7::supports_ssl)
        .count();

    println!("{}", n);
}

#[derive(Debug,PartialEq)]
enum Segment<'a> {
    Normal(&'a str),
    Hyper(&'a str),
}

impl<'a> Segment<'a> {
    fn is_hyper(&self) -> bool {
        match *self {
            Segment::Hyper(..) => true,
            _ => false,
        }
    }
    fn abas(&self) -> impl Iterator<Item=&'a [u8]> {
        let segment = match *self {
            Segment::Normal(s) => s,
            Segment::Hyper(s) => s,
        };
        
        segment.as_bytes()
            .windows(3)
            .filter(|w| w[0] == w[2] && w[0] != w[1])
    }
}

fn to_bab(s: &[u8]) -> [u8; 3] {
    [s[1], s[0], s[1]]
}

#[derive(Debug,PartialEq)]
struct Ipv7<'a>(Vec<Segment<'a>>);

impl<'a> Ipv7<'a> {
    fn from_str(mut s: &'a str) -> Ipv7 {
        let mut segments = Vec::new();

        while !s.is_empty() {
            if s.starts_with('[') {
                let end = s.find(']').expect("closing ']'");
                segments.push(Segment::Hyper(&s[1..end]));
                s = &s[end+1..];
            } else {
                let end = s.find('[').unwrap_or(s.len());
                segments.push(Segment::Normal(&s[..end]));
                s = &s[end..];
            }
        }
            
        Ipv7(segments)
    }

    fn supports_ssl(&self) -> bool {
        let abas = self.0.iter()
            .filter(|s| !s.is_hyper())
            .flat_map(|s| s.abas())
            .collect::<Vec<_>>();
        let babs = self.0.iter()
            .filter(|s| s.is_hyper())
            .flat_map(|s| s.abas())
            .map(to_bab)
            .collect::<Vec<_>>();

        babs.iter().any(|bab| abas.contains(&&bab[..]))
    }
}
