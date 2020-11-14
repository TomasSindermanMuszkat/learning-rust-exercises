// explicit
fn main() {
    let name_age:(&str, i8, &str, i8, &str, i8) = ("Alex", 21, "Abe", 22, "Anna", 23);
    println!("{}:{}, {}:{}, {}:{}", name_age.0, name_age.1, name_age.2, name_age.3, name_age.4, name_age.5,)
}

// implicit
fn implicit() {
    let name_age = ("Alex", 21, "Abe", 22, "Anna", 23);
    println!("{}:{}, {}:{}, {}:{}", name_age.0, name_age.1, name_age.2, name_age.3, name_age.4, name_age.5,)
}
