use std::{str::{Lines}};

fn create_vector(txt: &str) -> Vec<u32>{
    let mut out = Vec::new();
    let mut split = txt.split("-");
    let a = split.next().expect("Something went wrong").parse::<u32>().unwrap();
    let b = split.next().expect("Something went wrong").parse::<u32>().unwrap();
    for i in a..b+1 {
        out.push(i);
    } 
    out
}

fn solve_a(lines: Lines)-> u32 {
    let mut result = 0;
    for line in lines {
        let mut split = line.split(",");
        let first= split.next().expect("Something wrong");
        let second = split.next().expect("Something wrong");
        let range_first = create_vector(first);
        let range_second = create_vector(second);
        if range_first.iter().all(|value| range_second.contains(value)) | range_second.iter().all(|value| range_first.contains(value)) {
            result+=1;
        }
    }
    result
}

fn solve_b(lines: Lines)->u32 {
    let mut result = 0;
    for line in lines {
        let mut split = line.split(",");
        let first= split.next().expect("Something wrong");
        let second = split.next().expect("Something wrong");
        let range_first = create_vector(first);
        let range_second = create_vector(second);
        if range_first.iter().any(|value| range_second.contains(value)) {
            result+=1;
        }
    }
    result
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
        assert_eq!(2, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(4, solution);
    
}
}
