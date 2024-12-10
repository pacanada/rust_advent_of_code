use std::str::Lines;

fn main() {
    let input = include_str!("../../inputs/2024_03_example.txt");
    let solution_1: u32 = solve_part_1(input);
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(input);
    println!("Part 2: {solution_2}");
}


fn solve_part_1(input: &str) -> u32 {
    // for (i, c) in input.chars().enumerate() {
    //     println!("{:?}", input.chars().);
    // }
    // for x in input.as_bytes().windows(4) {
    //     println!("{}", input.as_bytes()[*x]);
    // }
    0
}

fn solve_part_2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let input = include_str!("../../inputs/2024_03_example.txt");
        let solution = solve_part_1(input);
        assert_eq!(161, solution);
    }
    #[test]
    fn question_b() {
        let input = include_str!("../../inputs/2024_03_example.txt").lines();
        let solution = solve_part_2(input);
        assert_eq!(4, solution);
    }
}
