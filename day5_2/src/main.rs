extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let input = b"reyedfim";
    let mut pass: [Option<char>; 8] = [None, None, None, None, None, None, None, None];
    let mut md5 = Md5::new();
    let hash = &mut [0; 16];

    for i in 0 .. std::u64::MAX {
        md5.input(input);
        md5.input(i.to_string().as_bytes());
        md5.result(hash);
        md5.reset();
        if &hash[0..2] == b"\0\0" && hash[2] <= 0x0F {
            let pos = match format!("{:x}", hash[2] & 0x0F).parse() {
                Ok(pos @ 0...7) => pos,
                _ => continue,
            };
            let ch = format!("{:x}", hash[3]).as_bytes()[0] as char;
            if pass[pos].is_some() {
                continue
            }
            pass[pos] = Some(ch);
            println!("Progress: {:?}", pass);
        }
        if pass.iter().all(|c| c.is_some()) {
            break
        }
    }

    println!("{}", pass.iter().cloned().flat_map(|c| c).collect::<String>());
}