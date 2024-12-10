use std::{collections::HashMap, str::Lines};

fn main() {
    let lines = include_str!("../../inputs/2024_02.txt").lines();
    let solution_1: u32 = solve_part_1(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(lines);
    println!("Part 2: {solution_2}");
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut all_dec = true;
    let mut all_inc = true;
    for i in 0..report.len()-1 {
        let diff = report[i+1]- report[i];
        if diff>0 {
            all_dec=false;
        }
        if diff<0 {
            all_inc=false;
        }
        if (diff.abs()>3) | (diff==0) {
            return false;
        }
    }
    (all_dec & !all_inc) | (!all_dec & all_inc)
}
fn is_safe_with_replacement(report: &Vec<i32>) -> bool {
    let report_length = report.len();
    //println!("{:?}", report);
    for i in 0..report_length {
        //println!("{:?}", [&report[..i], &report[i+1..]]);
        // Report: 
        // [7, 6, 4, 2, 1]
        // Combinations checked (if any is safe, the report is safe) -> O(N*T^2)
        // [[], [6, 4, 2, 1]]
        // [[7], [4, 2, 1]]
        // [[7, 6], [2, 1]]
        // [[7, 6, 4], [1]]
        // [[7, 6, 4, 2], []]
        if is_safe(&[&report[..i], &report[i+1..]].concat()) {
            return true;
        }

    }
    false
    

}
fn parse(lines: Lines) -> Vec<Vec<i32>> {
    lines.map(|line| {
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    }).collect()

}

fn solve_part_1(lines: Lines) -> u32 {
    let reports: Vec<Vec<i32>> = parse(lines);
    reports.iter().filter(|&report| is_safe(report)).count() as u32
}

fn solve_part_2(lines: Lines) -> u32 {
    let reports: Vec<Vec<i32>> = parse(lines);
    reports.iter().filter(|&report| is_safe_with_replacement(report)).count() as u32
    //diff.iter().inspect(|&x| println!("{:?}: {}", x,is_safe_with_replacement(x) )).filter(|&report| is_safe_with_replacement(report)).count() as u32
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2024_02_example.txt").lines();
        let solution = solve_part_1(lines);
        assert_eq!(2, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2024_02_example.txt").lines();
        let solution = solve_part_2(lines);
        assert_eq!(4, solution);
    }
    // #[test]
    // fn question_b_report_example_1() {
    //     // 6 6 9 10 11 13
    //    assert!(is_safe_with_replacement(&vec![0,3,1,1,2]));
    //    // 7 7 8 10 11 12 13 11
    //    assert!(!is_safe_with_replacement(&vec![0,1,2,1,1,1,-2]));

    // }
}
