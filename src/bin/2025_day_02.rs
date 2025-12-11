
use std::str::Lines;

fn main() {
    let lines = include_str!("../../inputs/2025_02_example.txt");
    let solution_1: u64 = solve_part_1(lines);
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(lines);
    println!("Part 2: {solution_2}");
}


fn solve_part_1(input: &str) -> u64 {
    let mut sum_invalid: u64 = 0;
    for pattern in input.split(",") {
        let mut iter_patter = pattern.split("-");
        let left: u64 = iter_patter.next().unwrap().parse().expect(&format!("Could not parse {pattern}"));
        let right: u64 = iter_patter.next().unwrap().parse().unwrap();
        println!("Considering {left}-{right}");
        for number in left..=right {
            let number_of_digits = number.ilog10()+1;
            if number_of_digits%2==0 {
                let number_as_string = &number.to_string()[..];
                let half_index = (number_of_digits/2) as usize;
                if number_as_string[..half_index ]==number_as_string[half_index..] {
                    println!("invalid {number_as_string}");
                    sum_invalid+=number;
                }

            }
        }
    }
    
    sum_invalid
}
fn is_invalid_2(number: u64)->bool {
    let number_as_string = &number.to_string()[..];
    let number_of_digits = number.ilog10()+1;
    // number_as_string
    for chunk in 1..=number_of_digits/2 {
        number_as_string.chars().collect()
        // for i in 0..=number_of_digits {
        //     if number_as_string[i..i+chunk]!=
        // } 

    }
    false

}
fn solve_part_2(input: &str) -> u32 {
    let mut sum_invalid: u64 = 0;
    for pattern in input.split(",") {
        let mut iter_patter = pattern.split("-");
        let left: u64 = iter_patter.next().unwrap().parse().expect(&format!("Could not parse {pattern}"));
        let right: u64 = iter_patter.next().unwrap().parse().unwrap();
        println!("Considering {left}-{right}");
        for number in left..=right {
            let number_of_digits = number.ilog10()+1;
            if number_of_digits%2==0 {
                let number_as_string = &number.to_string()[..];
                let half_index = (number_of_digits/2) as usize;
                if number_as_string[..half_index ]==number_as_string[half_index..] {
                    println!("invalid {number_as_string}");
                    sum_invalid+=number;
                }

            }
        }
    }
    
    sum_invalid
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2025_02_example.txt");
        let solution = solve_part_1(lines);
        assert_eq!(1227775554, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2025_02_example.txt");
        let solution = solve_part_2(lines);
        assert_eq!(6, solution);
    }
}
