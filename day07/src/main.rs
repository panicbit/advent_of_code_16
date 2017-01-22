use std::str;

fn main() {
    let input = include_str!("../input.txt");

    let n = input.lines()
        .map(|line| Ipv7::from_str(line.trim()))
        .filter(Ipv7::supports_tls)
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

    fn supports_tls(&self) -> bool {
           self.0.iter().filter(|s|  s.is_hyper()).all(|s| !s.has_abba())
        && self.0.iter().filter(|s| !s.is_hyper()).any(|s|  s.has_abba())
    }
}
