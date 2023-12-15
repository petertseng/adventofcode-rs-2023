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

    let mut open_end_in_col = vec![0; width];
    let mut open_start_in_col = vec![0; width];

    for &(y0, x) in &rocks {
        let mut y = y0;
        while y > open_start_in_col[x] && !blocks.contains(&(y - 1, x)) {
            y -= 1;
        }
        if y == open_start_in_col[x] {
            y = open_end_in_col[x];
        } else {
            // by the while condition above:
            assert!(blocks.contains(&(y - 1, x)));
        }
        open_end_in_col[x] = y + 1;
        open_start_in_col[x] = y0;
        new_rocks.push((y, x));
    }

    let load1 = new_rocks.iter().map(|&(y, _)| height - y).sum::<usize>();

    new_rocks.clear();

    crate::day14_parabolic_reflector_dish::cycle1bil(|_| {
        let mut open_end_in_col = vec![0; width];
        let mut open_start_in_col = vec![0; width];
        // no need to sort here:
        // rocks come in sorted order by reading order
        // or previous iteration sorted them
        for &(y0, x) in &rocks {
            let mut y = y0;
            while y > open_start_in_col[x] && !blocks.contains(&(y - 1, x)) {
                y -= 1;
            }
            if y == open_start_in_col[x] {
                y = open_end_in_col[x];
            } else {
                assert!(blocks.contains(&(y - 1, x)));
            }
            open_end_in_col[x] = y + 1;
            open_start_in_col[x] = y0;
            new_rocks.push((y, x));
        }
        std::mem::swap(&mut rocks, &mut new_rocks);
        new_rocks.clear();

        let mut open_end_in_row = vec![0; height];
        let mut open_start_in_row = vec![0; height];
        rocks.sort_unstable_by_key(|&(_, x)| x);
        for &(y, x0) in &rocks {
            let mut x = x0;
            while x > open_start_in_row[y] && !blocks.contains(&(y, x - 1)) {
                x -= 1;
            }
            if x == open_start_in_row[y] {
                x = open_end_in_row[y];
            } else {
                assert!(blocks.contains(&(y, x - 1)));
            }
            open_end_in_row[y] = x + 1;
            open_start_in_row[y] = x0;
            new_rocks.push((y, x));
        }
        std::mem::swap(&mut rocks, &mut new_rocks);
        new_rocks.clear();

        let mut open_end_in_col = vec![height - 1; width];
        let mut open_start_in_col = vec![height - 1; width];
        rocks.sort_unstable_by_key(|&(y, _)| y);
        for &(y0, x) in rocks.iter().rev() {
            let mut y = y0;
            while y < open_start_in_col[x] && !blocks.contains(&(y + 1, x)) {
                y += 1;
            }
            if y == open_start_in_col[x] {
                y = open_end_in_col[x];
            } else {
                assert!(blocks.contains(&(y + 1, x)));
            }
            open_end_in_col[x] = y - 1;
            open_start_in_col[x] = y0;
            new_rocks.push((y, x));
        }
        std::mem::swap(&mut rocks, &mut new_rocks);
        new_rocks.clear();

        let mut open_end_in_row = vec![width - 1; height];
        let mut open_start_in_row = vec![width - 1; height];
        rocks.sort_unstable_by_key(|&(_, x)| x);
        for &(y, x0) in rocks.iter().rev() {
            let mut x = x0;
            while x < open_start_in_row[y] && !blocks.contains(&(y, x + 1)) {
                x += 1;
            }
            if x == open_start_in_row[y] {
                x = open_end_in_row[y];
            } else {
                assert!(blocks.contains(&(y, x + 1)));
            }
            open_end_in_row[y] = x - 1;
            open_start_in_row[y] = x0;
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
