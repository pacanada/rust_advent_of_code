
use std::{str::Lines};

const LENGTH: usize = 12; //1000; //12 ;
const WIDTH: usize = 5; //12; //5;

fn get_oxygen_rating(matrix) -> i32 {
    // calculation

    for number in matrix {
        println!("{number}");
    }
    23


}

fn solve(lines: Lines)-> i32 {


    // parsing into matrix
    let mut matrix = [[0f32; LENGTH]; WIDTH];
    for (i, line) in lines.enumerate() {
        for (j, bit) in line.chars().enumerate() {
            matrix[j][i] = bit.to_digit(10).unwrap() as f32;

        }
    }
    get_oxygen_rating(matrix)
} 
fn main() {
    let lines = include_str!("input.txt").lines();
    let solution: i32 = solve(lines);
    println!("{solution}")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve(lines);
        assert_eq!(230, solution);
    }
}