use hex::ToHex;
use crypto::md5::Md5;
use crypto::digest::Digest;

pub struct RandomData {
    salt: &'static str,
    index: usize,
}

impl RandomData {
    pub fn with_salt(salt: &'static str) -> RandomData {
        RandomData {
            salt: salt,
            index: 0,
        }
    }
}

impl Iterator for RandomData {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let mut md5 = Md5::new();
        let hash = &mut [0; 16];

        md5.input(self.salt.as_bytes());
        md5.input(self.index.to_string().as_bytes());
        md5.result(hash);

        self.index += 1;

        Some(hash.to_hex())
    }
}
