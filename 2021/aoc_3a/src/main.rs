use std::{str::Lines};

const LENGTH: usize = 1000; //12 ;
const WIDTH: usize = 12; //5;

fn solve(lines: Lines)-> i32 {


    // parsing into matrix
    let mut matrix = [[0f32; LENGTH]; WIDTH];
    for (i, line) in lines.enumerate() {
        for (j, bit) in line.chars().enumerate() {
            matrix[j][i] = bit.to_digit(10).unwrap() as f32;

        }
    }
    // calculation
    let mut gamma_string = String::new();
    let mut epsilon_string = String::new();
    for a in matrix.iter() {
        // it will panic for sum of integers
        let sum: f32 = a.iter().sum();
        if sum as usize > LENGTH/2 {
            gamma_string.push_str("1");
            epsilon_string.push_str("0");
        } else {
            gamma_string.push_str("0");
            epsilon_string.push_str("1")}

    }
    let gamma_int = i32::from_str_radix(&gamma_string, 2).expect("Not a binary number");
    let epsilon_int = i32::from_str_radix(&epsilon_string, 2).expect("Not a binary number");
    // solution
    gamma_int*epsilon_int
}

// other approach
    // let mut gamma_bits_string = "";
    // for j in 0..WIDTH {
    //     let mut sum = 0;
    //     for i in 0..LENGTH {
    //         sum+=matrix[i][j];
    //     }
    //     println!("{sum}");
    //     if sum as usize > LENGTH/2 {
    //         gamma_bits[j]="1";
    //     } else {
    //         gamma_bits[j]="0";
    //     }

    // }

    // let mut gamma_bits = ["0";WIDTH];
    // let mut epsilon_bits = ["0";WIDTH];
    // for j in 0..WIDTH {
    //     let mut sum = 0;
    //     for i in 0..LENGTH {
    //         sum+=matrix[i][j];
    //     }
    //     println!("{sum}");
    //     if sum as usize > LENGTH/2 {
    //         gamma_bits[j]="1";
    //         epsilon_bits[j]="0";
    //     } else {
    //         gamma_bits[j]="0";
    //         epsilon_bits[j]="1";
    //     }
       
    // }
    // let gamma_bits_string: String = gamma_bits.into_iter().collect();
    // let epsilon_bits_string: String = epsilon_bits.into_iter().collect();
    // //println!("{:?}", i32::from_str_radix(gamma_bits_string, 2).expect("").unwrap();
    // println!("{:?}", gamma_bits_string);
    // println!("{:?}", epsilon_bits_string);

    // let gamma_int = i32::from_str_radix(&gamma_bits_string, 2).expect("Not a binary number!");
    // let epsilon_int = i32::from_str_radix(&epsilon_bits_string, 2).expect("Not a binary number!");
    

    // println!("{}", gamma_int*epsilon_int);
    // gamma_int*epsilon_int



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
        assert_eq!(198, solution);
    }
}

