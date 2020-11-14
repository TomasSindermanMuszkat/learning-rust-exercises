fn square_arr(array:&mut [i32;5]){
    for i in 0..array.len(){
        array[i] *= array[i]; 
    }
}

fn main() {
    let mut array: [i32;5] = [1, 2, 3, 4, 5];
    square_arr(&mut array);
    println!("{:?}", array);
}
