use std::{str::Lines, vec};

enum Values {
    Rock = 1, //A
    Paper = 2, // B 
    Scissor = 3 // C
}

fn solve_a(lines: Lines)->i32 {
    let mut vec:Vec<i32> = Vec::new();
    for line in lines {
        let a = line.split_once(" ").expect("something went wrong parsing play");
        let mut result = 0;
        println!("{:?}", a);
        match a {
            ("A", "Y") => result = 2 + 6,
            ("A", "X") => result = 1 + 3,
            ("A", "Z") => result = 3 + 0,
            ("B", "Y") => result = 2 + 3,
            ("B", "X") => result = 1 + 0,
            ("B", "Z") => result = 3 + 6,
            ("C", "Y") => result = 2 + 0,
            ("C", "X") => result = 1 + 6,
            ("C", "Z") => result = 3 + 3,
            (&_, _) => todo!(),
        }
        vec.push(result);
    }
   vec.iter().sum()
}

fn solve_b(lines: Lines)->i32 {
    let mut vec:Vec<i32> = Vec::new();
    for line in lines {
        let a = line.split_once(" ").expect("something went wrong parsing play");
        let mut result = 0;
        println!("{:?}", a);
        match a {
            ("A", "Y") => result = 1 + 3, // i need to draw
            ("A", "X") => result = 3 + 0, // i need to lose
            ("A", "Z") => result = 2 + 6, // i need to win
            ("B", "Y") => result = 2 + 3,
            ("B", "X") => result = 1 + 0,
            ("B", "Z") => result = 3 + 6,
            ("C", "Y") => result = 3 + 3,
            ("C", "X") => result = 2 + 0,
            ("C", "Z") => result = 1 + 6,
            (&_, _) => todo!(),
        }
        vec.push(result);
    }
   vec.iter().sum()
}

fn main() {
    let lines_a = include_str!("input.txt").lines();
    let lines_b = include_str!("input.txt").lines();
    let solution_a = solve_a(lines_a);
    let solution_b = solve_b(lines_b);
    println!("Solution a: {solution_a}");
    println!("Solution b: {solution_b}");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve_a(lines);
        assert_eq!(15, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(12, solution);
    
}
}

