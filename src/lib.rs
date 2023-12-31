pub mod day14_parabolic_reflector_dish;

use std::env;
use std::fs;

#[macro_export]
macro_rules! tests {
    (
        $($test_func:ident {
            $( $test_name:ident( $( $param:expr ),* ); )+
        })+
    ) => {
        $(
            $(
                #[test]
                fn $test_name() {
                    $test_func($( $param ),* )
                }
            )+
        )+
    }
}

pub fn numbers<T>(s: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.split(|c: char| c != '-' && !c.is_ascii_digit())
        .filter_map(|d| {
            if d.is_empty() {
                None
            } else {
                Some(
                    d.parse::<T>()
                        .expect("numbers should only be called on strings containing numbers"),
                )
            }
        })
        .collect()
}

pub fn read_input_lines<T, F>(f: F) -> Vec<T>
where
    F: FnMut(&str) -> T,
{
    read_input_file().lines().map(f).collect()
}

pub fn read_input_file() -> String {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "/dev/stdin".to_string());
    fs::read_to_string(filename).expect("caller should provide a valid filename in ARGV or stdin")
}
