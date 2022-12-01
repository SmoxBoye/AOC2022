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
    let mut second_biggest_elf = 0;
    let mut third_biggest_elf = 0;
    let mut current_elf = 0;
    for line in data.lines() {
        //print!("Current line: {line}\n");
        if line == ""{
            if current_elf > biggest_elf{
                third_biggest_elf = second_biggest_elf;
                second_biggest_elf = biggest_elf;
                biggest_elf = current_elf;
            }
            else
            {
                if current_elf > second_biggest_elf{
                    third_biggest_elf = second_biggest_elf;
                    second_biggest_elf = current_elf;
                }
                else {
                    if current_elf > third_biggest_elf{
                        third_biggest_elf = current_elf;
                    }
                }
            }
            current_elf = 0;
        }
        else
        {
            current_elf += line.parse::<i32>().unwrap()
        }
        
        //println!("Current elf rn: {current_elf}");
        //println!("Biggest rn: {biggest_elf}");
    }
    if current_elf > biggest_elf{
        third_biggest_elf = second_biggest_elf;
        second_biggest_elf = biggest_elf;
        biggest_elf = current_elf;
    }
    else
    {
        if current_elf > second_biggest_elf{
            third_biggest_elf = second_biggest_elf;
            second_biggest_elf = current_elf;
        }
        else {
            if current_elf > third_biggest_elf{
                third_biggest_elf = current_elf;
            }
        }
    }
    print!("Biggest elf is: {biggest_elf}\n");
    print!("Second elf is: {second_biggest_elf}\n");
    print!("Third elf is: {third_biggest_elf}\n");
    let total_snacks = biggest_elf + second_biggest_elf + third_biggest_elf;
    println!("The biggest elf is: {total_snacks}");
}
