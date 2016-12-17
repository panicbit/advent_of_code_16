use random_data::RandomData;
use std::rc::Rc;
use std::iter::Map;
use iterslide::{Slide, SlideIterator};

pub struct KeyStream {
    random_data: Slide<Map<RandomData, fn(String) -> Rc<String>>, Rc<String>>,
    index: usize,
}

impl KeyStream {
    pub fn with_salt(salt: &'static str) -> KeyStream {
        KeyStream {
            random_data: RandomData::with_salt(salt).map(Rc::new as _).slide(1001),
            index: 0,
        }
    }
}

impl Iterator for KeyStream {
    type Item = Key;
    fn next(&mut self) -> Option<Key> {
        for hashes in &mut self.random_data {
            let current_index = self.index;
            let candidate = &hashes[0];
            let hashes = &hashes[1..];

            let is_key = find_xlet(candidate, 3)
                .map(|triplet|
                    hashes
                    .iter()
                    .any(|hash| find_xlet(hash, 5) == Some(triplet))
                )
                .unwrap_or(false);

            self.index += 1;

            if is_key {
                return Some(Key {
                    index: current_index,
                    data: candidate.clone(),
                })
            }
        }

        unreachable!()
    }
}

fn find_xlet(data: &str, n: usize) -> Option<char> {
    data.chars()
        .slide(n).into_iter()
        .filter(|cs| cs.len() == n)
        .find(|cs| cs.iter().all(|&c| c == cs[0]))
        .map(|cs| cs[0])
}

pub struct Key {
    pub index: usize,
    pub data: Rc<String>,
}

#[test]
fn test_xlet() {
    assert_eq!(find_xlet("cc38887a5", 3), Some('8'));
    assert_eq!(find_xlet("cc3888887a5", 5), Some('8'));
}
