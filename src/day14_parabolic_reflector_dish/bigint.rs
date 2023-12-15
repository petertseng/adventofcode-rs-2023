use rug::Integer;

type Platform = (usize, usize, Integer, Integer);

pub fn platform(grid: &str) -> Platform {
    let mut blocks = Integer::new();
    let mut rocks = Integer::new();

    let mut height = 0;
    let mut width = None;

    for row in grid.lines() {
        height += 1;
        if let Some(prev_width) = width {
            assert_eq!(row.len(), prev_width);
        } else {
            width = Some(row.len());
        }

        for c in row.chars() {
            blocks <<= 1;
            rocks <<= 1;
            match c {
                '#' => blocks |= 1,
                'O' => rocks |= 1,
                '.' => (),
                _ => panic!("bad char {c}"),
            };
        }
    }
    (height, width.unwrap_or(0), blocks, rocks)
}

pub fn loads(platform: Platform) -> (u32, u32) {
    let (height, width, blocks, rocks0) = platform;

    let size = height * width;

    let each_row = (0..height).fold(Integer::new(), |a, _| a << width | 1);
    let each_col = (1_u128 << width) - 1;

    let left_col = Integer::from((1_u128 << (width - 1)) * &each_row);
    let right_col = &each_row;
    let top_row = Integer::from(each_col) << (size - width);
    let bottom_row = Integer::from(each_col);

    // TODO: There's an awful lot of Integer::from here.
    // The type checker seems to indicate that they're necessary,
    // in order to convert from incomplete computations to complete ones,
    // for operators that require complete operands.
    // (try removing one and see what the type checker says)
    // I'd like to confirm whether I'm supposed to write these a different way,
    // or if these are natural and OK to have when working with rug.

    let mut rocks = rocks0.clone();
    loop {
        let can_move_up = &rocks & !(Integer::from(&blocks | &rocks) >> width) & Integer::from(!&top_row);
        if can_move_up == 0 {
            break;
        }
        rocks = rocks & Integer::from(!&can_move_up) | Integer::from(&can_move_up << width);
    }

    let load1 = load(&rocks, width);

    let mut rocks = rocks0;
    crate::day14_parabolic_reflector_dish::cycle1bil(|_| {
        loop {
            let can_move_up = &rocks & !(Integer::from(&blocks | &rocks) >> width) & Integer::from(!&top_row);
            if can_move_up == 0 {
                break;
            }
            rocks = &rocks & Integer::from(!&can_move_up) | Integer::from(&can_move_up << width);
        }
        loop {
            let can_move_left = &rocks & !(Integer::from(&blocks | &rocks) >> 1_u32) & Integer::from(!&left_col);
            if can_move_left == 0 {
                break;
            }
            rocks = &rocks & Integer::from(!&can_move_left) | Integer::from(&can_move_left << 1);
        }
        loop {
            let can_move_down = &rocks & !(Integer::from(&blocks | &rocks) << width) & Integer::from(!&bottom_row);
            if can_move_down == 0 {
                break;
            }
            rocks = &rocks & Integer::from(!&can_move_down) | Integer::from(&can_move_down >> width);
        }
        loop {
            let can_move_right = &rocks & !(Integer::from(&blocks | &rocks) << 1_u32) & Integer::from(!right_col);
            if can_move_right == 0 {
                break;
            }
            rocks = &rocks & Integer::from(!&can_move_right) | Integer::from(&can_move_right >> 1);
        }
        rocks.clone()
    });

    (load1, load(&rocks, width))
}

fn load(rocks: &Integer, width: usize) -> u32 {
    let mut load = 0;
    let mut y = 1;
    let mut rocks = rocks.clone();
    let row = (1_u128 << width) - 1;
    while rocks != 0 {
        load += y * Integer::from(&rocks & row).count_ones().unwrap();
        rocks >>= width;
        y += 1;
    }
    load
}
