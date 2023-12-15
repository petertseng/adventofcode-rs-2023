type Platform = (usize, usize, Vec<u128>, Vec<u128>);

pub fn platform(grid: &str) -> Platform {
    let mut width = None;

    let (blocks, rocks): (Vec<u128>, Vec<u128>) = grid.lines().map(|line| {
        if let Some(prev_width) = width {
            assert_eq!(line.len(), prev_width);
        } else {
            width = Some(line.len());
        }
        let mut bs = 0;
        let mut rs = 0;
        for c in line.chars() {
            bs <<= 1;
            rs <<= 1;
            match c {
                '#' => bs |= 1,
                'O' => rs |= 1,
                '.' => (),
                _ => panic!("bad char {c}"),
            }
        }
        (bs, rs)
    }).unzip();
    // Code would need to be rewritten to handle > 128 bits,
    // by storing each row as a vec of u128.
    // If inspiration is needed, consider the ideas in
    // https://github.com/SLiV9/AdventOfCode2022/blob/main/src/bin/day23/main.rs
    assert!(width.unwrap_or(0) <= 128);
    (blocks.len(), width.unwrap_or(0), blocks, rocks)
}

pub fn loads(platform: Platform) -> (usize, usize) {
    let (height, width, blocks, rocks0) = platform;

    let mut rocks = rocks0.clone();

    loop {
        let mut any_move = false;
        for i in 1..height {
            let can_move_up = rocks[i] & !rocks[i - 1] & !blocks[i - 1];
            any_move = any_move || can_move_up != 0;
            rocks[i] &= !can_move_up;
            rocks[i - 1] |= can_move_up;
        }
        if !any_move {
            break;
        }
    }

    let load1 = load(&rocks, height);

    let left_column = 1_u128 << (width - 1);
    let right_column = 1_u128;

    let mut rocks = rocks0;

    crate::day14_parabolic_reflector_dish::cycle1bil(|_| {
        loop {
            let mut any_move = false;
            for i in 1..height {
                let can_move_up = rocks[i] & !rocks[i - 1] & !blocks[i - 1];
                any_move = any_move || can_move_up != 0;
                rocks[i] &= !can_move_up;
                rocks[i - 1] |= can_move_up;
            }
            if !any_move {
                break;
            }
        }

        loop {
            let mut any_move = false;
            for (r, b) in rocks.iter_mut().zip(blocks.iter()) {
                let can_move_left = *r & !((*r | b) >> 1) & !left_column;
                any_move = any_move || can_move_left != 0;
                *r = *r & !can_move_left | can_move_left << 1;
            }
            if !any_move {
                break;
            }
        }

        loop {
            let mut any_move = false;
            for negi in 1..height {
                let i = height - 1 - negi;
                let can_move_down = rocks[i] & !rocks[i + 1] & !blocks[i + 1];
                any_move = any_move || can_move_down != 0;
                rocks[i] &= !can_move_down;
                rocks[i + 1] |= can_move_down;
            }
            if !any_move {
                break;
            }
        }

        loop {
            let mut any_move = false;
            for (r, b) in rocks.iter_mut().zip(blocks.iter()) {
                let can_move_right = *r & !((*r | b) << 1) & !right_column;
                any_move = any_move || can_move_right != 0;
                *r = *r & !can_move_right | can_move_right >> 1;
            }
            if !any_move {
                break;
            }
        }

        rocks.clone()
    });

    (load1, load(&rocks, width))
}

fn load(rocks: &[u128], height: usize) -> usize {
    rocks.iter().enumerate().map(|(i, &v)| (height - i) * usize::try_from(v.count_ones()).unwrap()).sum()
}
