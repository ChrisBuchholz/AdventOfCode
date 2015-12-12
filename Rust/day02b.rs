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
        let per = vec![l + l + w + w, h + h + w + w, l + l + h + h];
        feet += (l * w * h) + *per.iter().min().unwrap();
    }
    println!("{}", feet);
}
