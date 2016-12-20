fn main() {
    let input = 3004953;
    let elves: Vec<u32> = (1..input+1).collect();
    let mut elves = SplitVec::from_vec(elves);

    while elves.len() > 1 {
        let mut i = 0;
        while i < elves.len() {
            let target = (i + (elves.len() / 2)) % elves.len();

            elves.remove(target);

            if elves.len() % 1_000 == 0 {
                println!("{}", elves.len());
            }

            if target > i {
                i += 1;
            }
        }
    }

    println!("Elf {} has all the presents!", elves[0]);
}

#[derive(Debug)]
struct SplitVec<T> {
    vecs: Vec<Vec<T>>
}

impl<T> SplitVec<T> {
    fn from_vec(mut vec: Vec<T>) -> Self {
        let max_size = 10_000;
        let mut vecs = Vec::new();
        while vec.len() > max_size {
            let rest = vec.split_off(max_size);
            vec.shrink_to_fit();
            vecs.push(vec);
            vec = rest;
        }
        vecs.push(vec);

        SplitVec {
            vecs: vecs,
        }
    }
}

impl<T> SplitVec<T> {
    fn find_vec(&self, mut index: usize) -> (usize, usize) {
        for (veci, vec) in self.vecs.iter().enumerate() {
            if index < vec.len() {
                return (veci, index);
            }
            index -= vec.len();
        }

        panic!("Index out of bounds")
    }
    fn remove(&mut self, index: usize) {
        let (veci, index) = self.find_vec(index);
        self.vecs[veci].remove(index);

        if self.vecs[veci].len() == 0 {
            self.vecs.remove(veci);
        }
    }
    fn len(&self) -> usize {
        self.vecs.iter().map(Vec::len).sum()
    }
}

impl<T> ::std::ops::Index<usize> for SplitVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        let (veci, index) = self.find_vec(index);
        &self.vecs[veci][index]
    }
}
