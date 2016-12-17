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
        use ::std::fmt::Write;
        let mut md5 = Md5::new();
        let hash = &mut [0; 16];

        md5.input(self.salt.as_bytes());
        md5.input(self.index.to_string().as_bytes());
        md5.result(hash);

        let mut hex = String::with_capacity(32);
        for byte in hash {
            write!(hex, "{:02x}", byte).unwrap();
        }

        self.index += 1;

        Some(hex)
    }
}
