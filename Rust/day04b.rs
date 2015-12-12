// cargo-deps: rust-crypto="0.2.34"

extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let mut hasher = Md5::new();
    let mut iter = 0;
    let mut result = [0; 16];
    loop {
        hasher.input(b"yzbqklnj");
        hasher.input(iter.to_string().as_bytes());
        hasher.result(&mut result);
        hasher.reset();
        if result[..3] == [0; 3] {
            break;
        }
        iter += 1;
    }
    assert_eq!(iter, 9962624);
    println!("{}", iter);
}
