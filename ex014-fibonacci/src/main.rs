use std::io::{self, Write};
use text_io::read;

fn flushio(){
    io::stdout().flush().unwrap();
}

fn fib(n: i32) -> i32 {
    if n < 2{
        n
    }
    else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    print!("Enter the number of the fibonacci sequence you want --> ");
    flushio();
    let fibo_number: i32 = read!();
    println!("{}", fib(fibo_number));
}
