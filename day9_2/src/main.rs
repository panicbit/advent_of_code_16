#![feature(conservative_impl_trait)]
#[macro_use] extern crate nom;
use std::str;
use nom::is_digit;

fn main() {
    let input = include_str!("../input.txt");
    let data = parse_data(input.trim().as_bytes()).to_result().unwrap();
    let bytes: usize = data.iter().map(|datum| datum.count_bytes()).sum();
    println!("{}", bytes);
}

fn is_uncompressed_data(ch: u8) -> bool {
    ch != b'('
}

named!(uncompressed_data<Datum>, do_parse!(
       data: take_while!(is_uncompressed_data)
    >> ({
        let data = str::from_utf8(data).unwrap();
        Datum::Uncompressed(data)
    })
));

named!(compressed_data<Datum>, do_parse!(
       tag!("(")
    >> len: take_while!(is_digit)
    >> tag!("x")
    >> repeats: take_while!(is_digit)
    >> tag!(")")
    >> len: expr_res!(str::from_utf8(len).unwrap().parse::<usize>())
    >> data: take!(len)
    >> ({
        let data = str::from_utf8(data).unwrap();
        let repeats = str::from_utf8(repeats).unwrap().parse().unwrap();
        Datum::Compressed(data, repeats)
    })
));

named!(datum<Datum>,
    alt!(compressed_data | uncompressed_data)
);

named!(parse_data<Vec<Datum>>, many1!(datum));

#[derive(Debug)]
enum Datum<'a> {
    Uncompressed(&'a str),
    Compressed(&'a str, usize),
}

impl<'a> Datum<'a> {
    fn count_bytes(&self) -> usize {
        match *self {
            Datum::Uncompressed(s) => {
                println!("Uncompressed: {}", s);
                s.len()
            },
            Datum::Compressed(s, repeats) => {
                let data = parse_data(s.as_bytes()).to_result().unwrap();
                repeats * data.iter().map(|datum| datum.count_bytes()).sum::<usize>()
            },
        }
    }
}
