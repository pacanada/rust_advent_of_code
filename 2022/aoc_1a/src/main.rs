use std::{str::Lines, vec};
fn solve_a(lines: Lines)->i32 {
    let mut vec:Vec<i32> = Vec::new();
    let mut calories_per_elf = 0;

    for line in lines {
        if line.is_empty() {
            vec.push(calories_per_elf);
            calories_per_elf = 0;
        } else {
            let a: i32 = line.parse().unwrap();
            calories_per_elf += a;
        }
    }
    // last row
    vec.push(calories_per_elf);
    // what is the maximum
    let solution = vec.iter().max().expect("Something went worng");
    *solution
}

fn solve_b(lines: Lines)->i32 {
    let mut vec:Vec<i32> = Vec::new();
    let mut calories_per_elf = 0;

    for line in lines {
        if line.is_empty() {
            vec.push(calories_per_elf);
            calories_per_elf = 0;
        } else {
            let a: i32 = line.parse().unwrap();
            calories_per_elf += a;
        }
    }
    // last row
    vec.push(calories_per_elf);

    // what is the 3 maximum
    vec.sort();
    vec.reverse();
    //vec.sort_by(|a,b| b.cmp(a));
    println!("{:?}", vec);
    let mut solution = 0;
    for a in &vec[0..3] {
        solution += a;
        println!("{a}")
    }
    solution
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
        assert_eq!(24000, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(45000, solution);
    
}
}
