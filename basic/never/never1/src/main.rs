// never型は絶対に返らない

use std::io;

pub fn chk(p1: u32) -> Result<T> {
    if p1 == 2 {
        Ok(None)
    } else {
        Err(None)
    }
}

fn main() {
    println!("Hello, world!");
}
