use std::{str::Lines, vec};


fn solve_a(lines: Lines)->i32 {
    for line in lines {
        let half_index = line.len();
        println!(line[0..half_index]);
        println!("{:?}", half_index);
    }
    1
}

fn solve_b(lines: Lines)->i32 {
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
        assert_eq!(157, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(12, solution);
    
}
}

