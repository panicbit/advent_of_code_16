use util;

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
        let hash = util::hash(&[self.salt, &self.index.to_string()]);
        self.index += 1;
        Some(util::hash_rounds(&hash, 2016))
    }
}
