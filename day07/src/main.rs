#[macro_use]
extern crate nom;

use std::char;
use std::str;

fn main() {
    let input = include_str!("../input.txt");

    let n = input.lines()
        .map(|line| Ipv7::from_str(line.trim()).unwrap())
        .filter(Ipv7::supports_tls)
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
    fn has_abba(&self) -> bool {
        let segment = match *self {
            Segment::Normal(s) => s,
            Segment::Hyper(s) => s,
        };
        
        segment.as_bytes()
            .windows(4)
            .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
    }
}

#[derive(Debug,PartialEq)]
struct Ipv7<'a>(Vec<Segment<'a>>);

impl<'a> Ipv7<'a> {
    fn from_str(s: &'a str) -> Result<Self, nom::Err> {
        ipv7(s.as_bytes()).to_result()
    }

    fn supports_tls(&self) -> bool {
           self.0.iter().filter(|s|  s.is_hyper()).all(|s| !s.has_abba())
        && self.0.iter().filter(|s| !s.is_hyper()).any(|s|  s.has_abba())
    }
}
