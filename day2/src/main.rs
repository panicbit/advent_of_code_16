fn main() {
    let input = include_str!("../input.txt");
    let mut pad = Keypad::new();

    for line in input.lines() {
        for dir in line.trim().chars() {
            match dir {
                'U' => pad.up(),
                'D' => pad.down(),
                'L' => pad.left(),
                'R' => pad.right(),
                _ => panic!("Unknown dir: {}", dir),
            }
        }
        
        print!("{}", pad.key());
    }

    println!("");
}

struct Keypad {
    x: u8,
    y: u8,
}

impl Keypad {
    fn new() -> Self {
        // Start at 5
        Keypad {
            x: 1,
            y: 1,
        }
    }

    fn key(&self) -> u8 {
        self.y * 3 + self.x + 1
    }

    fn up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    fn down(&mut self) {
        if self.y < 2 {
            self.y += 1
        }
    }

    fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn right(&mut self) {
        if self.x < 2 {
            self.x += 1
        }
    }
}
