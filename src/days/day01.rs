use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let data = read_to_string("input/input01.txt").unwrap();
    let sol1: u64 = part1(data.clone());
    let sol2: u64 = part2(data.clone());


    (Solution::U64(sol1), Solution::U64(sol2))
}


fn part1(data: String) -> u64{
    let mut biggest_elf: u64 = 0;
    let mut current_elf: u64 = 0;
    for line in data.lines() {
        //print!("Current line: {line}\n");
        if line == ""{
            current_elf = 0;
        }
        else
        {
            current_elf += line.parse::<u64>().unwrap()
        }
        if current_elf > biggest_elf{
            biggest_elf = current_elf;
        }
        //println!("Current elf rn: {current_elf}");
        //println!("Biggest rn: {biggest_elf}");
    }

    return biggest_elf;
}


fn part2(data: String) -> u64{
    let mut biggest_elf: u64 = 0;
    let mut second_biggest_elf:u64 = 0;
    let mut third_biggest_elf:u64  = 0;
    let mut current_elf:u64  = 0;
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
            current_elf += line.parse::<u64>().unwrap()
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
    //print!("Biggest elf is: {biggest_elf}\n");
    //print!("Second elf is: {second_biggest_elf}\n");
    //print!("Third elf is: {third_biggest_elf}\n");
    let total_snacks = biggest_elf + second_biggest_elf + third_biggest_elf;
    return total_snacks;
}