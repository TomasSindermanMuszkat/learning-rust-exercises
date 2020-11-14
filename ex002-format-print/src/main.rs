// best 4 me
fn main(){
    println!("{}\n{}\n{}\n{}\n{}", 1, 22, 333, 4444, 55555);
}

// alt
/* fn main2(){
    println!("{0}\n{1}{1}\n{2}{2}{2}\n{3}{3}{3}{3}\n{4}{4}{4}{4}{4}", 1, 2, 3, 4, 5);
} */

// dumb? by course
/* fn main3(){
    println!("{}", 1);
    println!("{}{}", 2, 2);
    println!("{}{}{}", 3, 3, 3);
    println!("{}{}{}{}", 4, 4, 4, 4);
    println!("{}{}{}{}{}",5, 5, 5, 5, 5);
} */
// dumb alt yet fastest
/* fn main4(){
    println!("{0}", 1);
    println!("{0}{0}", 2);
    println!("{0}{0}{0}", 3);
    println!("{0}{0}{0}{0}", 4);
    println!("{0}{0}{0}{0}{0}", 5);
} */