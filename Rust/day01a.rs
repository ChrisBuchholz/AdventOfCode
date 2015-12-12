use std::io;
use std::io::Read;

fn main() {
    let mut floor = 0i32;
    for d in io::stdin().bytes() {
        floor += match d.unwrap() as char {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    }
    println!("{}", floor);
}
