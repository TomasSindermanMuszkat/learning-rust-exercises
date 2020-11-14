use std::io::{self, Write};
use text_io::read;

fn flushio() {
    io::stdout().flush().unwrap();
}

fn main() {
    print!("Enter the value you want to reduce 3 by 3 till less than 0 --> ");
    flushio();
    let mut x: i32 = read!();
    let mut count = 0;
    while x >= 0 {
        count += 1;
        x -= 3;
    }
    println!("{}", count)
}
