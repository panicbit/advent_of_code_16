extern crate itertools;

fn main() {
    let input = 3004953;
    let mut elves: Vec<u32> = (1..input+1).collect();

    while elves.len() > 1 {
        let mut i = 0;
        while i < elves.len() {
            let target = (i + (elves.len() / 2)) % elves.len();

            elves.remove(target);

            if elves.len() % 1000 == 0 {
                println!("{}", elves.len());
            }

            if target > i {
                i += 1;
            }
        }
    }

    println!("{}", elves[0]);
}
