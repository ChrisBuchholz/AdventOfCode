use std::io;
use std::io::BufRead;

fn main() {
    let mut feet = 0i32;
    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let nums = l.unwrap()
                    .split('x')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
        let (l, w, h) = (nums[0], nums[1], nums[2]);
        let sides = vec![l * w, w * h, h * l];
        let smallest = sides.iter().min().unwrap();
        feet += smallest + sides.iter().fold(0, |acc, &x| acc + 2 * x);
    }
    assert_eq!(feet, 1598415);
    println!("{}", feet);
}
