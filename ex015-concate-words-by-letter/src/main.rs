use std::io::{self, Write};
use text_io::read;

fn flushio(){
    io::stdout().flush().unwrap();
}

fn main(){
    print!("Enter the char to use to filter the text by starting word --> ");
    flushio();
    let filter: char = read!();
    print!("Enter the text to filter --> ");
    flushio();
    let text: String = read!("\n{}\n");
    let mut output = "".to_string();
    for word in text.split_whitespace(){
        if word.starts_with(filter){
            output.push_str(word);
            output.push(' ');
        }
    }
    output.pop();
    println!("{}", output);
}