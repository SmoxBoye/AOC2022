use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::env;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    env::set_var("RUST_BACKTRACE", "1");
    // Your solution here...
    let _data = read_to_string("input/input03.txt").unwrap();
    //println!("{_data}");

    let char_vector = pre_process_data(&_data);
    //println!("{char_vector:?}");
    let sol1: u64 = solution1(&char_vector);
    let sol2: u64 = solution2(&_data);

    (Solution::U64(sol1), Solution::U64(sol2))
}



fn pre_process_data(data: &String) -> Vec<char>{
    let mut items: Vec<char> = Vec::new();
    for line in data.lines(){
        let middle = line.len() / 2;
        let left = &line[..middle];
        let right = &line[middle..];

        'outer: for item1 in left.chars(){
            for item2 in right.chars(){
                if item1 == item2{
                    items.push(item1);
                    break 'outer;
                }
            }
        }
    }
    return items;
}


fn solution1(data: &Vec<char>) -> u64{
    let mut priority: u64 = 0;
    for item in data.iter(){
        let value = *item as u64;
        if value < 91{
            let val = (value % 64) + 26;
            priority += val;
        }
        else{
            let val = value % 96;
            priority += val;
        }
    }
    return priority;
}


fn solution2(data: &String) -> u64{
    let mut priority: u64 = 0;
    let row_count = {
        let mut c = 0;
        for _ in data.lines(){
            c +=1;
        }
        c/3
    };
    let mut iters = 0;
    let mut data_iter = data.lines();
    while iters < row_count{
        let elf1 = data_iter.next().unwrap();
        let elf2 = data_iter.next().unwrap();
        let elf3 = data_iter.next().unwrap();

        // Screams in O(n^3)
        'outer: for item1 in elf1.chars()
        {
            for item2 in elf2.chars()
            {
                for item3 in elf3.chars()
                {
                    if item1 == item2 && item2 == item3{
                        let value = item1 as u64;
                        if value < 91{
                            let val = (value % 64) + 26;
                            priority += val;
                        }
                        else{
                            let val = value % 96;
                            priority += val;
                        }
                        break 'outer;
                    }
                }
            }
        }
        iters += 1;
    }
    


    return priority;
}