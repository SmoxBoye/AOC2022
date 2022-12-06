use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, iter::Enumerate, ops::Index};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let _data = read_to_string("input/input06.txt").unwrap();
    let sol1: u64 = solution1(&_data, 4);
    let sol2: u64 = solution1(&_data, 14);

    (Solution::U64(sol1), Solution::U64(sol2))
}


// Accedentally wrote the solution so it can handle multiple lines and im too lazy to undo it
fn solution1(data: &String, n:usize) -> u64{
    let mut count: u64 = 0;
    for line in data.lines(){
        let byte_line = line.as_bytes();
        'outer: for i in 0..line.len()-n{
            for k in i..i+n-1{
                for l in k+1..i+n{
                    if byte_line[k] == byte_line[l]{
                        continue 'outer;
                    }
                }
            }
            count += (i + n) as u64 ;
            break;
        }
    }
    return count;
}
