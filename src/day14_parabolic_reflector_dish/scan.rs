use std::collections::HashSet;

type Platform = (usize, usize, HashSet<(usize, usize)>, HashSet<(usize, usize)>);

pub fn platform(grid: &str) -> Platform {
    let mut blocks = HashSet::new();
    let mut rocks = HashSet::new();

    let mut height = 0;
    let mut width = None;

    for (y, row) in grid.lines().enumerate() {
        height += 1;
        if let Some(prev_width) = width {
            assert_eq!(row.len(), prev_width);
        } else {
            width = Some(row.len());
        }

        for (x, c) in row.chars().enumerate() {
            match c {
                '#' => blocks.insert((y, x)),
                'O' => rocks.insert((y, x)),
                '.' => true,
                _ => panic!("bad char {c}"),
            };
        }
    }
    (height, width.unwrap_or(0), blocks, rocks)
}

pub fn loads(platform: Platform) -> (usize, usize) {
    let (height, width, blocks, mut rocks) = platform;

    let mut new_rocks = HashSet::new();

    for x in 0..width {
        let mut open = 0;
        for y in 0..height {
            if blocks.contains(&(y, x)) {
                open = y + 1;
            } else if rocks.contains(&(y, x)) {
                new_rocks.insert((open, x));
                open += 1;
            }
        }
    }
    let load1 = new_rocks.iter().map(|&(y, _)| height - y).sum::<usize>();

    crate::day14_parabolic_reflector_dish::cycle1bil(|_| {
        for x in 0..width {
            let mut open = 0;
            for y in 0..height {
                if blocks.contains(&(y, x)) {
                    open = y + 1;
                } else if rocks.contains(&(y, x)) {
                    new_rocks.insert((open, x));
                    open += 1;
                }
            }
        }
        std::mem::swap(&mut rocks, &mut new_rocks);
        new_rocks.clear();

        for y in 0..height {
            let mut open = 0;
            for x in 0..width {
                if blocks.contains(&(y, x)) {
                    open = x + 1;
                } else if rocks.contains(&(y, x)) {
                    new_rocks.insert((y, open));
                    open += 1;
                }
            }
        }
        std::mem::swap(&mut rocks, &mut new_rocks);
        new_rocks.clear();

        for x in 0..width {
            let mut open = height - 1;
            for negy in 0..height {
                let y = height - 1 - negy;
                if blocks.contains(&(y, x)) {
                    open = y - 1;
                } else if rocks.contains(&(y, x)) {
                    new_rocks.insert((open, x));
                    open -= 1;
                }
            }
        }
        std::mem::swap(&mut rocks, &mut new_rocks);
        new_rocks.clear();

        for y in 0..height {
            let mut open = width - 1;
            for negx in 0..width {
                let x = width - 1 - negx;
                if blocks.contains(&(y, x)) {
                    open = x - 1;
                } else if rocks.contains(&(y, x)) {
                    new_rocks.insert((y, open));
                    open -= 1;
                }
            }
        }
        std::mem::swap(&mut rocks, &mut new_rocks);
        new_rocks.clear();

        let mut v: Vec<_> = rocks.iter().cloned().collect();
        v.sort_unstable();

        v
    });

    let load = rocks.iter().map(|&(y, _)| height - y).sum::<usize>();
    (load1, load)
}
