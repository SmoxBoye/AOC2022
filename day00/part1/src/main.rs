use std::fs::File;
use std::io::prelude::*;


fn read_input()-> String{
    let mut file = File::open("input.txt")
        .expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
    .expect("Error while reading file");
    return data;
}

fn main() {
    println!("Hello, world!");

    let data = read_input();
    println!("{data}");
}
