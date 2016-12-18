extern crate regex;
use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    
    let mut discs: Vec<_> = input.lines().map(|line| Disc::from_str(line.trim())).collect();
    discs.push(Disc {
        positions: 11,
        position: 0,
    });
    let time = calc_time(&discs);

    println!("{}", time);
}

fn calc_time(discs: &[Disc]) -> usize {
    'try: for init_elapsed in 0.. {
        let mut elapsed = init_elapsed+1;
        for disc in discs {
            if !disc.after(elapsed).falls_through()  {
                continue 'try;
            }
            elapsed += 1;
        }

        return init_elapsed;
    }

    panic!("Impossible :O");
}

struct Disc {
    positions: usize,
    position: usize,
}

impl Disc {
    fn from_str(s: &str) -> Self {
        let re = Regex::new(r"Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+)").unwrap();
        let c = re.captures(s).unwrap();
        Disc {
            positions: c[1].parse().unwrap(),
            position: c[2].parse().unwrap(),
        }
    }

    fn after(&self, elapsed: usize) -> Self {
        Disc {
            positions: self.positions,
            position: (self.position + elapsed) % self.positions,
        }
    }

    fn falls_through(&self) -> bool {
        self.position == 0
    }
}
