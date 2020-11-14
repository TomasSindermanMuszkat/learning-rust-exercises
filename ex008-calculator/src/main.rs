use std::io::{self, Write};
use text_io::read;

fn flushio() {
    io::stdout().flush().unwrap();
}

fn main() {
    print!("Enter the first number --> ");
    flushio();
    let a: i32 = read!();
    print!("Enter the operator --> ");
    flushio();
    let operator: char = read!();
    print!("Enter the second number --> ");
    flushio();
    let b: i32 = read!();
    match operator {
        '+' => println!("Your result is: {}", a + b),
        '-' => println!("Your result is: {}", a - b),
        '/' => {
            if b != 0 {
                println!("Your result is: {}", a / b);
            } else {
                println!("Division by 0 is undefined");
            }
        }
        '*' => println!("Your result is: {}", a * b),
        '%' => {
            if b != 0 {
                println!("Your result is: {}", a % b);
            } else {
                println!("Division by 0 is undefined");
            }
        }
        _ => println!("invalid operator"),
    }
}
