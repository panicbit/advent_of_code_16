#![feature(conservative_impl_trait)]
#[macro_use]
extern crate nom;

use std::char;
use std::str;

fn main() {
    let input = include_str!("../input.txt");

    let n = input.lines()
        .map(|line| Ipv7::from_str(line.trim()).unwrap())
        .filter(Ipv7::supports_ssl)
        .count();

    println!("{}", n);
}

fn is_raw_segment(b: u8) -> bool {
    char::from_u32(b as u32).map(char::is_alphabetic).unwrap_or(false)
}

named!(raw_segment<&str>, do_parse!(
       segment: take_while1_s!(is_raw_segment)
    >> (str::from_utf8(segment).unwrap())
));

named!(normal_segment<Segment>, do_parse!(
    segment: raw_segment
    >> (Segment::Normal(segment))
));

named!(hyper_segment<Segment>, do_parse!(
       tag!("[")
    >> segment: raw_segment
    >> tag!("]")
    >> (Segment::Hyper(segment))
));

named!(ipv7<Ipv7>, do_parse!(
       segments: many1!(alt!(
           normal_segment | hyper_segment
       ))
    >> (Ipv7(segments))
));

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
    fn from_str(s: &'a str) -> Result<Self, nom::Err> {
        ipv7(s.as_bytes()).to_result()
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
