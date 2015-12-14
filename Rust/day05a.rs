use std::io;
use std::io::BufRead;

fn has_vowels(str: &String) -> bool {
    let mut count = 0;
    let vowels = "aeiou";
    for s in str.chars() {
        if vowels.contains(s) {
            count += 1;
        }
    }
    return count >= 3;
}

fn has_double(str: &String) -> bool {
    for p in str.as_bytes().windows(2) {
        if p[0] == p[1] {
            return true;
        }
    }
    return false;
}

fn sans_banned(str: &String) -> bool {
    let slice = &str[..];
    let bans = ["ab", "cd", "pq", "xy"];
    return bans.iter().all(|b| !slice.contains(b));
}

fn main() {
    let mut nice_strs = 0;
    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap() as String;
        if has_vowels(&line) && has_double(&line) && sans_banned(&line) {
            nice_strs += 1;
        }
    }
    assert_eq!(258, nice_strs);
    println!("{}", nice_strs);
}
