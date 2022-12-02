use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;
///////////////////////////////////////////////////////////////////////////////




pub fn solve() -> SolutionPair {
    // Your solution here...
    let _data = read_to_string("input/input02.txt").unwrap();
    let processed_data = pre_process_data(_data);
    let sol1: u64 = solution1(processed_data.clone());
    let sol2: u64 = solution2(processed_data.clone());

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn pre_process_data(data: String)->Vec<(i32, i32)>{
    let translate = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3),
        ("X", 1),
        ("Y", 2),
        ("Z", 3)
    ]);
    let mut processed_data = Vec::new();
    for line in data.lines(){
        let split: Vec<&str> = line.split(" ").collect();
        processed_data.push(
            (
                translate[split[0]],
                translate[split[1]]
            )
        )
    }
    return processed_data;
}

// Functions apparently faster than using hashmaps?????? Saved like 0.06 ms for some reason

fn rock(input: i32) -> u64{
    if input == 1{
        return 3;
    }
        
    if input == 2{
        return 0;
    }
    // Implied input == 3, required to avoid UB
    return 6;
}

fn paper(input: i32) -> u64{
    if input == 1{
        return 6;
    }
        
    if input == 2{
        return 3;
    }
    // Implied input == 3, required to avoid UB
    return 0;
}

fn scissors(input: i32) -> u64{
    if input == 1{
        return 0;
    }
        
    if input == 2{
        return 6;
    }
    // Implied input == 3, required to avoid UB
    return 3;
}

fn calculate_score(data: Vec<(i32, i32)>) -> u64{
    let mut score: u64 = 0;
    for game in data{
        if game.1 == 1{
            score += 1 + rock(game.0)
        }
        if game.1 == 2{
            score += 2 + paper(game.0)
        }
        if game.1 == 3{
            score += 3 + scissors(game.0)
        }
    }
    return score;
}

fn solution1(data: Vec<(i32, i32)>) -> u64{
    return calculate_score(data);
}

// Once again saved 0.02 ms because functions??

fn rock_t(input: i32) -> i32{
    if input == 1{
        return 3;
    }
        
    if input == 2{
        return 1;
    }
    // Implied input == 3, required to avoid UB
    return 2;
}

fn paper_t(input: i32) -> i32{
    if input == 1{
        return 1;
    }
        
    if input == 2{
        return 2;
    }
    // Implied input == 3, required to avoid UB
    return 3;
}

fn scissors_t(input: i32) -> i32{
    if input == 1{
        return 2;
    }
        
    if input == 2{
        return 3;
    }
    // Implied input == 3, required to avoid UB
    return 1;
}

fn solution2(data: Vec<(i32, i32)>) -> u64{
    let mut processed_data = Vec::new();
    for game in data{
        if game.0 == 1{
            processed_data.push((game.0, rock_t(game.1)));
        }
        if game.0 == 2{
            processed_data.push((game.0, paper_t(game.1)));
        }
        if game.0 == 3{
            processed_data.push((game.0, scissors_t(game.1)));
        }
    }

    
    return calculate_score(processed_data);
}
