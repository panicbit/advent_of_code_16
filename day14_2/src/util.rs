use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn hash(data: &[&str]) -> String {
    use ::std::fmt::Write;
    let mut md5 = Md5::new();
    let hash = &mut [0; 16];

    for data in data {
        md5.input(data.as_bytes());
    }
    md5.result(hash);

    let mut hex = String::with_capacity(32);
    for byte in hash {
        write!(hex, "{:02x}", byte).unwrap();
    }

    hex
}

pub fn hash_rounds(data: &str, rounds: usize) -> String {
    let mut hash = self::hash(&[data]);
    for _ in 1..rounds {
        hash = self::hash(&[&hash]);
    }
    hash
}

#[test]
fn test_hash_rounds() {
    assert_eq!(hash_rounds("577571be4de9dcce85a041ba0410f29f", 2016), "a107ff634856bb300138cac6568c0f24");
}
