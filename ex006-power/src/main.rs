fn main() {
    let a = 2;
    let b = 2;
    let result = i8::pow(a, 3) + i8::pow(b, 3) + 3*a*b*(a+b);
    println!("{}", result);
}
