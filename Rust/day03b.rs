use std::io;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let (mut sy, mut sx) = (0i32, 0i32);
    let (mut ry, mut rx) = (0i32, 0i32);
    let mut xy: HashSet<(i32, i32)> = vec![(0i32, 0i32)].into_iter().collect();
    let mut iter = 0i32;
    for d in io::stdin().bytes() {
        let (mut y, mut x) = (&mut sy, &mut sx);
        if iter % 2 != 0 {
            y = &mut ry;
            x = &mut rx;
        }
        match d.unwrap() as char {
            '^' => *y += 1,
            'v' => *y -= 1,
            '>' => *x += 1,
            '<' => *x -= 1,
            _ => {}
        }
        xy.insert((*y, *x));
        iter += 1;
    }
    assert_eq!(xy.len(), 2639);
    println!("{}", xy.len());
}
