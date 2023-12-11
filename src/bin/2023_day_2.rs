use std::{
    str::{Chars, Lines},
    vec, sync::Arc,
};
fn main() {
    let lines = include_str!("../../inputs/2023_1.txt").lines();
    let solution_1: u32 = solve_a(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_b(lines);
    println!("Part 2: {solution_2}");
}

fn solve_a(lines: Lines) -> u32 {

    // let vec: Vec<u32> = lines
    // .filter_map(|line| {
    //     let number: String = line.chars()
    //         .filter(|c| c.is_digit(10))
    //         .collect();

        
    //         let first = &number[0..1];
    //         let last = &number[number.len()-1..];
    //         format!("{first}{last}").parse::<u32>().ok()
        
    // })
    // .collect();

    let mut vec: Vec<u32> = Vec::new();

    for line in lines {
        let mut number = String::from("");
        for char in line.chars() {
            //dbg!(char);
            if char.is_digit(10) {
                number+=&char.to_string();
            }

        }
        let first = &number[0..1];
        let last = &number[number.len()-1..];
        vec.push(format!("{first}{last}").parse::<u32>().unwrap());
    }
    vec.iter().sum()

    
}

fn solve_b(lines: Lines) -> u32 {


    // other approach to solve part 1
    let vec: Vec<u32> = lines
    .filter_map(|line| {
        //line = line;
        let number: String = line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "th3ee")
                .replace("four", "f4ur")
                .replace("five", "f5ve")
                .replace("six", "s6x")
                .replace("seven", "se7en")
                .replace("eight", "ei8ht")
                .replace("nine", "n9ne")
                .chars()
            .filter(|c| c.is_digit(10))
            .collect();

        
            let first = &number[0..1];
            let last = &number[number.len()-1..];
            format!("{first}{last}").parse::<u32>().ok()
        
    })
    .collect();
    vec.iter().sum()
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2023_1_a_example.txt").lines();
        let solution = solve_a(lines);
        assert_eq!(142, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2023_1_b_example.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(281, solution);
    }
}
