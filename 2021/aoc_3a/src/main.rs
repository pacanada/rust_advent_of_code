use std::{str::Lines, vec};

const LENGTH: usize = 1000;
const WIDTH: usize = 12;

fn solve(lines: Lines)-> i32 {



    let mut matrix = [[0u16; WIDTH]; LENGTH];
    for (i, line) in lines.enumerate() {
        for (j, bit) in line.chars().enumerate() {
            matrix[i][j] = bit.to_digit(10).unwrap() as u16;

        }
    }
    // calculation
    let mut gamma_bits = ["0";WIDTH];
    let mut epsilon_bits = ["0";WIDTH];
    for j in 0..WIDTH {
        let mut sum = 0;
        for i in 0..LENGTH {
            sum+=matrix[i][j];
        }
        println!("{sum}");
        if sum as usize > LENGTH/2 {
            gamma_bits[j]="1";
            epsilon_bits[j]="0";
        } else {
            gamma_bits[j]="0";
            epsilon_bits[j]="1";
        }
       
    }
    let gamma_bits_string: String = gamma_bits.into_iter().collect();
    let epsilon_bits_string: String = epsilon_bits.into_iter().collect();
    //println!("{:?}", i32::from_str_radix(gamma_bits_string, 2).expect("").unwrap();
    println!("{:?}", gamma_bits_string);
    println!("{:?}", epsilon_bits_string);

    //println!("{:?}", matrix);

    //let resut = lines.map(|x| x::from_str_radix(x, 2).unwrap());

    // creating matrix
    // let mut matrix = vec![vec![0; width]; length];
    // for (i, line) in lines.enumerate() {

    //     matrix[i] = line.trim().()).expect("oops");
    //}
    // for i, line in lines.enumerate() {
    //     matrix[]
    // }
    // for column in 0..width {
    //     println!("column {column}");
    //     for line in lines {
    //         //let value = &line[column]
    //         let value = line.chars().nth(column as usize);
    //     }
    // }
    23



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
        assert_eq!(198, solution);
    }
}

