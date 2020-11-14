use std::io::{self, Write};
use text_io::read;

fn flushio() {
    io::stdout().flush().unwrap();
}

fn main() {
    print!("Enter how many lines do you want this triangle to have --> ");
    flushio();
    let n: i32 = read!();
    for i in 0..n{
        for _j in 0..i+1{
            print!("&");
        }
    println!();
    }
}
