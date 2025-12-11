
use std::str::Lines;

fn main() {
    let lines = include_str!("../../inputs/2025_02.txt");
    let solution_1: u64 = solve_part_1(lines);
    println!("Part 1: {solution_1}");
    let solution_2: u64 = solve_part_2(lines);
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
fn find_divisors(n: usize)->Vec<usize> {
    let mut divisors= vec![];
    for i in 1..=n/2 {
        if n%i==0{
            divisors.push(i)
        }
        
    }
    divisors

}
fn is_invalid_given_chunk(number_str: &str, chunk: usize)-> bool {
    //println!("chunk {chunk}, length: {}", number_str.len());
    for i in 0..number_str.len()/chunk-1 {

        //println!("{number_str}:{i}");
        if number_str[i*chunk..i*chunk+chunk]!=number_str[i*chunk+chunk..i*chunk+2*chunk] {
            return false
        }
    }
    true

}
fn is_invalid(number: u64)->bool {
    let number_as_string = &number.to_string()[..];
    let number_of_digits = number.ilog10()+1;
    // which chunks should we try given the number_of_digits?
    let divisors = find_divisors(number_of_digits as usize);
    //println!("Number digits: {number_of_digits}, div: {:?}", divisors);
    for chunk in divisors {
        if is_invalid_given_chunk(number_as_string, chunk) {
            //println!("invalid: {number_as_string}");
            return true
        }

    }
    false

}
fn solve_part_2(input: &str) -> u64 {
    let mut sum_invalid: u64 = 0;
    for pattern in input.split(",") {
        let mut iter_patter = pattern.split("-");
        let left: u64 = iter_patter.next().unwrap().parse().expect(&format!("Could not parse {pattern}"));
        let right: u64 = iter_patter.next().unwrap().parse().unwrap();
        //println!("Considering {left}-{right}");
        for number in left..=right {
            if is_invalid(number) {
                println!("invalid {number}");
                sum_invalid+=number;

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
        assert_eq!(4174379265, solution);
    }
}
