//use adventofcode::day14_parabolic_reflector_dish::scan::{loads, platform};
//use adventofcode::day14_parabolic_reflector_dish::sort_rocks_naive::{loads, platform};
//use adventofcode::day14_parabolic_reflector_dish::sort_rocks_skip::{loads, platform};

/*
#[cfg(not(feature = "bigint"))]
use adventofcode::day14_parabolic_reflector_dish::u128_row::{loads, platform};
#[cfg(feature = "bigint")]
use adventofcode::day14_parabolic_reflector_dish::bigint::{loads, platform};
*/

// since u128_row is the fastest, I'll just use it all the time.
use adventofcode::day14_parabolic_reflector_dish::u128_row::{loads, platform};

fn main() {
    let grid = adventofcode::read_input_file();
    let plat = platform(&grid);
    let (load0, load1bil) = loads(plat);

    println!("{load0}");
    println!("{load1bil}");
}
