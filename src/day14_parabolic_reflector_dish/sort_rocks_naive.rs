use std::collections::HashSet;

type Platform = (usize, usize, HashSet<(usize, usize)>, Vec<(usize, usize)>);

pub fn platform(grid: &str) -> Platform {
    let mut blocks = HashSet::new();
    let mut rocks = vec![];

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
                '#' => { blocks.insert((y, x)); },
                'O' => rocks.push((y, x)),
                '.' => (),
                _ => panic!("bad char {c}"),
            };
        }
    }
    (height, width.unwrap_or(0), blocks, rocks)
}

pub fn loads(platform: Platform) -> (usize, usize) {
    let (height, width, blocks, mut rocks) = platform;

    let mut new_rocks = vec![];

    let mut placed = HashSet::new();

    rocks.sort_unstable_by_key(|&(y, _)| y);
    for &(mut y, x) in &rocks {
        while y > 0 && !placed.contains(&(y - 1, x)) && !blocks.contains(&(y - 1, x)) {
            y -= 1;
        }
        placed.insert((y, x));
        new_rocks.push((y, x));
    }

    let load1 = new_rocks.iter().map(|&(y, _)| height - y).sum::<usize>();

    new_rocks.clear();

    crate::day14_parabolic_reflector_dish::cycle1bil(|_| {
        placed.clear();
        // no need to sort here:
        // rocks come in sorted order by reading order
        // or previous iteration sorted them
        for &(mut y, x) in &rocks {
            while y > 0 && !placed.contains(&(y - 1, x)) && !blocks.contains(&(y - 1, x)) {
                y -= 1;
            }
            placed.insert((y, x));
            new_rocks.push((y, x));
        }
        std::mem::swap(&mut rocks, &mut new_rocks);
        new_rocks.clear();

        placed.clear();
        rocks.sort_unstable_by_key(|&(_, x)| x);
        for &(y, mut x) in &rocks {
            while x > 0 && !placed.contains(&(y, x - 1)) && !blocks.contains(&(y, x - 1)) {
                x -= 1;
            }
            placed.insert((y, x));
            new_rocks.push((y, x));
        }
        std::mem::swap(&mut rocks, &mut new_rocks);
        new_rocks.clear();

        placed.clear();
        rocks.sort_unstable_by_key(|&(y, _)| y);
        for &(mut y, x) in rocks.iter().rev() {
            while y + 1 < height && !placed.contains(&(y + 1, x)) && !blocks.contains(&(y + 1, x)) {
                y += 1;
            }
            placed.insert((y, x));
            new_rocks.push((y, x));
        }
        std::mem::swap(&mut rocks, &mut new_rocks);
        new_rocks.clear();

        placed.clear();
        rocks.sort_unstable_by_key(|&(_, x)| x);
        for &(y, mut x) in rocks.iter().rev() {
            while x + 1 < width && !placed.contains(&(y, x + 1)) && !blocks.contains(&(y, x + 1)) {
                x += 1;
            }
            placed.insert((y, x));
            new_rocks.push((y, x));
        }
        std::mem::swap(&mut rocks, &mut new_rocks);
        new_rocks.clear();

        // must sort for repeat to be found!
        rocks.sort_unstable();
        rocks.clone()
    });

    let load = rocks.iter().map(|&(y, _)| height - y).sum::<usize>();
    (load1, load)
}
