use num::{Bounded, Zero};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::mem;

fn remove<T: Eq>(v: &mut VecDeque<T>, e: &T) -> bool {
    if let Some((index, _)) = v.iter().enumerate().find(|&(_, x)| x == e) {
        v.remove(index);
        true
    } else {
        false
    }
}

pub fn fringe<N, C, FN, IN, FH, FS>(start: &N,
                                    neighbours: FN,
                                    heuristic: FH,
                                    success: FS)
                                    -> Option<(Vec<N>, C)>
    where N: Eq + Hash + Clone,
          C: Bounded + Zero + Ord + Copy,
          FN: Fn(&N) -> IN,
          IN: IntoIterator<Item = (N, C)>,
          FH: Fn(&N) -> C,
          FS: Fn(&N) -> bool
{
    let mut now = VecDeque::new();
    let mut later = VecDeque::new();
    let mut cache = HashMap::new();
    let mut flimit = heuristic(start);
    now.push_back(start.clone());
    cache.insert(start.clone(), (Zero::zero(), vec![start.clone()]));

    loop {
        if now.is_empty() {
            return None;
        }
        let mut fmin = C::max_value();
        while let Some(node) = now.pop_front() {
            let (g, path) = cache[&node].clone();
            let f = g + heuristic(&node);
            if f > flimit {
                if f < fmin {
                    fmin = f;
                }
                later.push_back(node);
                continue;
            }
            if success(&node) {
                return Some((path, g));
            }
            for (neighbour, cost) in neighbours(&node) {
                let g_neighbour = g + cost;
                if let Some(&(old_g, _)) = cache.get(&neighbour) {
                    if old_g <= g_neighbour {
                        continue;
                    }
                }
                remove(&mut later, &neighbour) || remove(&mut now, &neighbour);
                now.push_front(neighbour.clone());
                let mut neighbour_path = path.clone();
                neighbour_path.push(neighbour.clone());
                cache.insert(neighbour.clone(), (g_neighbour, neighbour_path));
            }
        }
        mem::swap(&mut now, &mut later);
        flimit = fmin;
    }
}
