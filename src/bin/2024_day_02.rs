use std::{collections::HashMap, str::Lines};

fn main() {
    let lines = include_str!("../../inputs/2024_02_example.txt").lines();
    let solution_1: u32 = solve_part_1(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(lines);
    println!("Part 2: {solution_2}");
}

fn build_diff_vector(lines: Lines) -> Vec<Vec<i32>> {
    let diff: Vec<Vec<i32>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .map(|x: Vec<i32>| x.windows(2).map(|y| y[1] - y[0]).collect())
        .collect();
    diff
}

fn is_safe(report: &Vec<i32>) -> bool {
    report.iter().all(|x| (*x > 0) & (*x <= 3)) | report.iter().all(|x| (*x < 0) & (*x >= -3))
}
fn is_safe_with_replacement(report: &Vec<i32>) -> bool {
    println!("{:?}", report);
    let mut carry =0;
    let mut sign = report[0] > 0;
    let mut skipped_one = false;
    for x in report[1..report.len()].iter() {
        let mut element = (*x).clone();
        element+=carry;
        carry=0;
        if (sign != (element > 0)) | (element.abs() > 3) | (element==0) {
            if !skipped_one {
                carry = element;
                skipped_one = true;
                sign = element>0;
                continue;
            }
            else {return false}

        }

    }
    true

}

fn solve_part_1(lines: Lines) -> u32 {
    let diff = build_diff_vector(lines);
    diff.iter().filter(|&report| is_safe(report)).count() as u32
}

fn solve_part_2(lines: Lines) -> u32 {
    let diff = build_diff_vector(lines);
    diff.iter().filter(|&report| is_safe_with_replacement(report)).count() as u32
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
