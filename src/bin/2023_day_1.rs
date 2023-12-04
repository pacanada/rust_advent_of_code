use std::{
    str::{Chars, Lines},
    vec,
};
fn main() {
    let lines = include_str!("../../inputs/2023_1.txt").lines();
    let solution = solve_a(lines);
    println!("{solution}");
}

fn solve_a(lines: Lines) -> u32 {
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
        //let num = number.chars().enumerate().map(|(i, x)| )
        vec.push(format!("{first}{last}").parse::<u32>().unwrap());
    }
    vec.iter().sum()

    
}

fn solve_b(lines: Lines) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2023_1_example.txt").lines();
        let solution = solve_a(lines);
        assert_eq!(142, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2023_1_example.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(12, solution);
    }
}
