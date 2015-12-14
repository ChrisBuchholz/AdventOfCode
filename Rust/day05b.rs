use std::io;
use std::io::BufRead;

fn has_double_pair(str: &String) -> bool {
    for i in 0..str.len() - 1 {
        let p = &str[i..i + 2];
        let r = &str[i + 2..str.len()];
        if r.contains(p) {
            return true;
        }
    }
    return false;
}

fn has_sandwiched_letter(str: &String) -> bool {
    for i in 0..str.len() - 2 {
        let t = &str[i..i + 3];
        if t.chars().nth(0) == t.chars().nth(2) {
            return true;
        }
    }
    return false;
}

fn main() {
    let mut nice_strs = 0i32;
    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap() as String;
        if has_double_pair(&line) && has_sandwiched_letter(&line) {
            nice_strs += 1;
        }
    }
    assert_eq!(53, nice_strs);
    println!("{}", nice_strs);
}
