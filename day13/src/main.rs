use std::collections::{HashMap, HashSet};

const INPUT: isize = 1364;
fn main() {
    let start = (1,1);
    let goal = (31,39);
    let path = a_star(start, goal);
    println!("{:?}", path.len()-1);
}

fn is_wall((x, y): (isize, isize)) -> bool {
    let n = x*x + 3*x + 2*x*y + y + y*y + INPUT;
    let ones = n.count_ones();
    ones % 2 != 0
}

fn a_star(start: (isize, isize), goal: (isize, isize)) -> Vec<(isize,isize)> {
    let mut closed_set = HashSet::new();
    let mut open_set = HashSet::new();
    open_set.insert(start);
    let mut came_from = HashMap::<(isize,isize), (isize, isize)>::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    g_score.insert(start, 0);
    f_score.insert(start, Some(ESTIMATED_COST));

    while !open_set.is_empty() {
        let current = *open_set.iter().min_by_key(|pos| f_score.get(pos).cloned().unwrap_or(None)).unwrap();
        if current == goal {
            return reconstruct_path(came_from, current);
        }

        open_set.remove(&current);
        closed_set.insert(current);

        let (x, y) = current;
        let neighbors = [
                        (x  , y-1),
            (x-1, y  ), (x  , y  ), (x+1, y  ),
                        (x  , y+1),
        ];
        let neighbors = neighbors.iter().filter(|&&(x, y)| x >= 0 && y >= 0 && !is_wall((x,y)));
        for &neighbor in neighbors {
            if closed_set.contains(&neighbor) {
                continue
            }
            let tentative_g_score = g_score[&current] + dist(current, neighbor);
            if !open_set.contains(&neighbor) {
                open_set.insert(neighbor);
            }
            else if tentative_g_score >= g_score[&neighbor] {
                continue
            }
            came_from.insert(neighbor, current);
            g_score.insert(neighbor, tentative_g_score);
            f_score.insert(neighbor, Some(g_score[&neighbor] + ESTIMATED_COST));
        }
    }
    panic!("Failure");
}

const ESTIMATED_COST: isize = 1;

fn dist(a: (isize, isize), b: (isize, isize)) -> isize {
    let dx = (a.0+b.0)/2;
    let dy = (a.1+b.1)/2;
    let sum = dx*dx + dy*dy;
    (sum as f32).sqrt().trunc() as isize
}

fn reconstruct_path(came_from: HashMap<(isize,isize), (isize, isize)>, mut current: (isize, isize)) -> Vec<(isize, isize)> {
    let mut path = vec![current];
    while came_from.keys().any(|&pos| pos == current) {
        current = came_from[&current];
        path.push(current);
    }
    path
}
