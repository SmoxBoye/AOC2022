use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let _data = read_to_string("input/input01.txt").unwrap();
    let vdata = vectorize_data(_data);
    let sol1: u64 = part1(vdata.clone());
    let sol2: u64 = part2(vdata.clone());


    (Solution::U64(sol1), Solution::U64(sol2))
}

fn vectorize_data(data: String) -> Vec<u64> {
    let mut current_elf: u64 = 0;
    let mut elf_vector = Vec::new();
    for line in data.lines(){
        if line == ""{
            elf_vector.push(current_elf);
            current_elf = 0;
        }
        else {
            current_elf += line.parse::<u64>().unwrap()
        }
    }
    elf_vector.push(current_elf);
    elf_vector.sort();
    return elf_vector;
}


fn part1(data: Vec<u64>) -> u64{
    let d_len = data.len();
    return data[d_len - 1];
}


fn part2(data: Vec<u64>) -> u64{
    let d_len = data.len();
    return data[d_len - 1] + data[d_len -2] + data[d_len - 3];
}