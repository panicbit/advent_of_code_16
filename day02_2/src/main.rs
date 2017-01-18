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

const FIELD: [char; 25] = [
    ' ', ' ' ,'1', ' ', ' ',
    ' ', '2' ,'3', '4', ' ',
    '5', '6' ,'7', '8', '9',
    ' ', 'A' ,'B', 'C', ' ',
    ' ', ' ' ,'D', ' ', ' ',
];

struct Keypad {
    x: usize,
    y: usize,
}

impl Keypad {
    fn new() -> Self {
        // Start at 5
        Keypad {
            x: 0,
            y: 2,
        }
    }

    fn key(&self) -> char {
        FIELD[self.y * 5 + self.x]
    }

    fn up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
        if self.key() == ' ' {
            self.y += 1;
        }
    }

    fn down(&mut self) {
        if self.y < 4 {
            self.y += 1;
        }
        if self.key() == ' ' {
            self.y -= 1;
        }
    }


    fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
        if self.key() == ' ' {
            self.x += 1;
        }
    }

    fn right(&mut self) {
        if self.x < 4 {
            self.x += 1;
        }
        if self.key() == ' ' {
            self.x -= 1;
        }
    }

}
