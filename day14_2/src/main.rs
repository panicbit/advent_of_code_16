extern crate crypto;
extern crate iterslide;

mod util;
mod random_data;
mod key_stream;
use key_stream::KeyStream;

fn main() {
    let input = "zpqevtbw";
    let key = KeyStream::with_salt(input).take(64).last().unwrap();

    println!("{}", key.index);
}
