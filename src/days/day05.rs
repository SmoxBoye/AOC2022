use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
//use std::env;


///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    //env::set_var("RUST_BACKTRACE", "1");
    // Your solution here...
    let _data = read_to_string("input/input05.txt").unwrap();
    let crates = process_input(&_data);
    //println!("{crates:?}");
    let sol1: String = solution1(&mut crates.clone(), &_data);
    let sol2: String = solution2(&mut crates.clone(), &_data);

    (Solution::Str(sol1), Solution::Str(sol2))
}

fn process_input(data: &String) -> Vec<Vec<&str>>{
    let mut columns: Vec<Vec<&str>> = vec![Vec::new(); 9];
    for line in data.lines(){
        let split: Vec<&str> = line.split(" ").collect();
        if split[0] == "1"{
            break;
        }
        for (index, item) in split.iter().enumerate(){
            if *item != "-"{
                columns[index].insert(0, item);
            }
        }
    }
    return columns;
}

fn solution1(crates: &mut Vec<Vec<&str>>, instructions: &String) -> String{
    
    let mut parsed_instructions = Vec::new();
    for line in instructions.lines().skip(10){
        let instruction: Vec<&str> = line
                                .split(' ')
                                .collect();
        //println!("{instruction:?}");
        parsed_instructions.push([instruction[1].parse::<usize>().unwrap(), instruction[3].parse::<usize>().unwrap(), instruction[5].parse::<usize>().unwrap()]);
    }
    //println!("Got here!");
    for instruction in parsed_instructions.iter(){
        //println!("{instruction:?}");
        for _ in 0..instruction[0]{
            let val = crates[instruction[1] - 1].pop().unwrap();
            crates[instruction[2] - 1].push(val);
        }
        // println!("{crates:?}")
    }
    let mut res = Vec::new();
    for item in crates.iter(){
        res.push(item.last().unwrap());
    }
    return format!("{res:?}");
}


fn solution2(crates: &mut Vec<Vec<&str>>, instructions: &String) -> String{
    
    let mut parsed_instructions = Vec::new();
    for line in instructions.lines().skip(10){
        let instruction: Vec<&str> = line
                                .split(' ')
                                .collect();
        //println!("{instruction:?}");
        parsed_instructions.push([instruction[1].parse::<usize>().unwrap(), instruction[3].parse::<usize>().unwrap(), instruction[5].parse::<usize>().unwrap()]);
    }
    //println!("Got here!");
    for instruction in parsed_instructions.iter(){
        //println!("{instruction:?}");
        let mut grab = Vec::new();
        for _ in 0..instruction[0]{
            let val = crates[instruction[1] - 1].pop().unwrap();
            //println!("{val}");
            grab.push(val);
        }
        //println!("{grab:?}");
        for item in grab.iter().rev(){
            crates[instruction[2] - 1].push(*item);
        }
        
        //println!("{crates:?}")
    }
    let mut res = Vec::new();
    for item in crates.iter(){
        res.push(item.last().unwrap());
    }
    return format!("{res:?}");
}