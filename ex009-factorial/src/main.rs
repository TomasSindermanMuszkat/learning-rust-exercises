use text_io::read;

fn main() {
    println!("Enter the number to calculate the factorial from:");
    let n:i32 = read!();
    let mut number = n;
    println!();
    if n < 0{
        println!("0");
    }
    else if n == 0{
        println!("1");
    }
    else{
        for i in 2..n{
            number *= i;
        }
    println!("{}", number);
    }
}
