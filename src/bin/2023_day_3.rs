use std::str::Lines;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Symbol {
    position: Position,
}

#[derive(Debug)]
struct Digit {
    position: Position,
    digit: u32,
}




fn main() {
    let lines = include_str!("../../inputs/2023_3_a_example.txt").lines();
    let solution_1: u32 = solve_a(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_b(lines);
    println!("Part 2: {solution_2}");
}

fn solve_a(lines: Lines) -> u32 {
    let mut digits: Vec<Digit> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    let y_last_digit = 0;
    for (x, line) in lines.enumerate() {
        for (y, char) in line.chars().enumerate() {
            match char {
                '.' => (),
                c if c.is_digit(10) => digits.push(Digit {
                    position: Position { x, y },
                    digit: c.to_digit(10).unwrap(),
                }),
                _ => symbols.push(Symbol {
                    position: Position { x, y },
                }),
            }
            //println!("{} {} {}", x, y, char);
        }
    }
    dbg!(digits);
    dbg!(symbols);

    1
}

fn solve_b(lines: Lines) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2023_3_a_example.txt").lines();
        let solution = solve_a(lines);
        assert_eq!(4361, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2023_3_b_example.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(2286, solution);
    }
}
