extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::fmt::Write;

fn main() {
    let input = b"reyedfim";
    let mut pass = String::new();
    let mut md5 = Md5::new();
    let hash = &mut [0; 16];

    for i in 0 .. std::u64::MAX {
        md5.input(input);
        md5.input(i.to_string().as_bytes());
        md5.result(hash);
        if &hash[0..2] == b"\0\0" && hash[2] <= 0x0F {
            write!(pass, "{:x}", hash[2] & 0x0F);
            println!("Progress: {}", pass);
        }
        if pass.len() == 8 {
            break
        }
        md5.reset();
    }

    println!("{}", pass);
}
