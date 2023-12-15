#[cfg(feature = "bigint")]
pub mod bigint;

pub mod scan;
pub mod sort_rocks_naive;
pub mod sort_rocks_skip;
pub mod u128_row;

use std::collections::{HashMap, HashSet};

pub fn cycle1bil<T, F>(mut f: F)
where
    F: FnMut(usize) -> T,
    T: std::hash::Hash + Eq,
{
    let mut t = 0;
    let target = 1_000_000_000;
    let mut seen = HashMap::new();

    while t < target {
        let v = f(t);
        if let Some(tprev) = seen.get(&v) {
            let cycle_len = t - tprev;
            t += (target - 1 - t) / cycle_len * cycle_len;
        }
        seen.insert(v, t);
        t += 1;
    }
    assert_eq!(t, target, "overshot target???");
}

pub fn print_grid(blocks: &[(usize, usize)], rocks: &[(usize, usize)]) {
    let miny = blocks.iter().chain(rocks.iter()).map(|&(y, _)| y).min().unwrap_or(0);
    let maxy = blocks.iter().chain(rocks.iter()).map(|&(y, _)| y).max().unwrap_or(0);
    let minx = blocks.iter().chain(rocks.iter()).map(|&(_, x)| x).min().unwrap_or(0);
    let maxx = blocks.iter().chain(rocks.iter()).map(|&(_, x)| x).max().unwrap_or(0);

    let blocks: HashSet<_> = blocks.iter().collect();
    let rocks: HashSet<_> = rocks.iter().collect();

    for y in miny..=maxy {
        for x in minx..=maxx {
            if blocks.contains(&(y, x)) {
                print!("#");
            } else if rocks.contains(&(y, x)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
