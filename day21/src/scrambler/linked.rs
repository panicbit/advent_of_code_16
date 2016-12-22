use super::Scrambler;

pub struct Linked<T: Scrambler, U: Scrambler>(pub T, pub U);

impl<T: Scrambler, U: Scrambler> Scrambler for Linked<T, U> {
    fn scramble(&self, arr: &mut [char]) {
        self.0.scramble(arr);
        self.1.scramble(arr);
    }
}

#[test]
fn test_link() {
    use super::{SwapPos, SwapVal};
    assert_eq!(SwapPos(4, 0).then(SwapVal('d', 'b')).scramble_str("abcde"), "edcba");
}
