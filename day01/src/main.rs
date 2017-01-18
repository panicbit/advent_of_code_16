fn main() {
    let input = include_str!("../input.txt").trim();
    let mut pos = (0, 0);
    let mut dir = Dir::N;

    for instr in input.split(", ") {
        let rot = instr.as_bytes()[0];
        let steps = instr[1..].parse::<i32>().unwrap();

        match rot {
            b'L' => dir.rotate_left(),
            b'R' => dir.rotate_right(),
            _ => unreachable!(),
        }

        pos = dir.go(pos, steps as i32);
    }

    let dist = pos.0.abs() + pos.1.abs();

    println!("{:?}", dist);
}

enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn go(&self, (x, y): (i32, i32), steps: i32) -> (i32, i32) {
        match *self {
            Dir::N => (x       , y+steps),
            Dir::E => (x+steps , y      ),
            Dir::S => (x       , y-steps),
            Dir::W => (x-steps , y      ),
        }
    }

    fn rotate_left(&mut self) {
        *self = match *self {
            Dir::N => Dir::W,
            Dir::W => Dir::S,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
        }
    }

    fn rotate_right(&mut self) {
        self.rotate_left();
        self.rotate_left();
        self.rotate_left();
    }
}
