use std::io;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let (mut cy, mut cx) = (0i32, 0i32);
    let mut xy = HashSet::new();
    for d in io::stdin().bytes() {
        xy.insert((cy, cx));
        match d.unwrap() as char {
            '^' => cy += 1,
            'v' => cy -= 1,
            '>' => cx += 1,
            '<' => cx -= 1,
            _ => {}
        }
    }
    assert_eq!(xy.len(), 2565);
    println!("{}", xy.len());
}
