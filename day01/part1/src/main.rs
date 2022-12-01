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

    let mut biggest_elf = 0;
    let mut current_elf = 0;
    for line in data.lines() {
        print!("Current line: {line}\n");
        if line == ""{
            current_elf = 0;
        }
        else
        {
            current_elf += line.parse::<i32>().unwrap()
        }
        if current_elf > biggest_elf{
            biggest_elf = current_elf;
        }
        println!("Current elf rn: {current_elf}");
        println!("Biggest rn: {biggest_elf}");
    }

    println!("The biggest elf is: {biggest_elf}");
}
