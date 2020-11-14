fn resize_vector(some_vec: &mut Vec<i32>) -> &mut Vec<i32>{
    if some_vec.len()%2 == 0{
        some_vec.pop();
    }
    some_vec.remove(some_vec.len()/2);
    let mut sum_of_elms = 0;
    for element in some_vec.iter(){
        sum_of_elms += element;
    }
    some_vec.push(sum_of_elms);
    some_vec
}

fn main() {
    let mut a_vec = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}",resize_vector(&mut a_vec));
}
