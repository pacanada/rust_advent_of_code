use std::str::Lines;

fn main() {
    let lines = include_str!("../../inputs/2025_03.txt").lines();
    let solution_1: u32 = solve_part_1(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(lines);
    println!("Part 2: {solution_2}");
}

fn parse_vec_from_line(line: &str)-> Vec<u32> {
    let v = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
    v
}
fn solve_part_1(lines: Lines) -> u32 {
    // brute force
    let mut max_accum = 0;
    for line in lines {
        let mut max = 0;
        let v = parse_vec_from_line(line);
        for (i, first_digit) in v.iter().enumerate() {
            for second_digit in &v[i+1..] {
                let num = first_digit*10+second_digit;
                if num>max {
                    max=num;
                }

            }
        }
        println!("{max}: {:?}", v);
        max_accum+=max;
        }

    max_accum
}

fn solve_part_2(lines: Lines) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2025_03_example.txt").lines();
        let solution = solve_part_1(lines);
        assert_eq!(357, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2025_03_example.txt").lines();
        let solution = solve_part_2(lines);
        assert_eq!(4, solution);
    }
}
