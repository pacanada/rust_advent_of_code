use std::{str::Lines, vec};
fn main() {
    let lines = include_str!("../../inputs/2022_10.txt").lines();
    let solution = solve_a(lines);
    println!("{solution}");
}

fn solve_a(lines: Lines) -> i32 {
    let input = lines.map(
        |line| line.split(" ").collect::<Vec<&str>>()
    ).collect::<Vec<Vec<&str>>>();
    //dbg!(input);
    let mut register = 1;
    let mut instruction_count = 0;
    let mut total = 0;
    //let mut evolution = Vec::<(u32, i32)>::new();
    for instruction in input {
        
        
        instruction_count+=1;
        if (instruction_count+20)%40==0 {
            total+=register*instruction_count;

        }
        

        if instruction.len()==2 {
            let  increment = instruction[1].parse::<i32>().unwrap();
            
            instruction_count+=1;
            if (instruction_count+20)%40==0 {
                total+=register*instruction_count;
    
            }
            register+=increment;

        }
    }

    total
    
}

fn solve_b(lines: Lines) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2022_10_example.txt").lines();
        let solution = solve_a(lines);
        assert_eq!(13140, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2022_10_example.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(12, solution);
    }
}
