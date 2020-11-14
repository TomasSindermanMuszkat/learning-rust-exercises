use std::io::{self, Write};
use text_io::read;

fn flushio() {
    io::stdout().flush().unwrap();
}

fn test_divisibility_by_3_4(n: i32) -> i32{
    if n%3 == 0 && n%4 == 0{
        return 0;
    }
    else if n%3 == 0{
        return 1;
    }
    else if n%4 == 0{
        return 2;
    }
    else {return -1}
}

fn main() {
    print!("Insert the number for which to check if its divisible by 3 or 4 --> ");
    flushio();
    let number = read!();
    print!("{}", test_divisibility_by_3_4(number))

}
