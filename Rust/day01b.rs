use std::io;
use std::io::Read;

fn main() {
    let (mut floor, mut pos) = (0i32, 0i32);
    for d in io::stdin().bytes() {
        pos += 1;
        floor += match d.unwrap() as char {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor == -1 {
            break;
        }
    }
    println!("{}", pos);
}
