use std::{str::{Lines}, collections::HashSet};

struct File {
    name: String,
    size: u32,
}

struct Tree {
    path: String,
    trees: Vec<Tree>,
    files: Vec<File>,
}
impl Tree {

    
}


fn solve_a(lines: Lines)-> u32 {
    for line in lines {
        println!("{:?}", line);
    }
1
}

fn solve_b(lines: Lines)->u32 {
1
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
        assert_eq!(7, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(19, solution);
    
}
}