use std::str::Lines;

fn main() {
    let input = include_str!("../../inputs/2024_03.txt");
    let solution_1: u32 = solve_part_1(input);
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(input);
    println!("Part 2: {solution_2}");
}
#[derive(Debug)]
struct Mul {
    a: u32,
    b: u32,
}
impl Mul {
    fn eval(self) -> u32 {
        self.a * self.b
    }
}
fn parse_numbers(pair_of_numbers: &str) -> Option<Mul> {
    let vec_numbers: Vec<&str> = pair_of_numbers.split(",").collect();
    if vec_numbers.len() != 2 {
        return None;
    }
    if (vec_numbers[0].len() > 3) | (vec_numbers[1].len() > 3) {
        return None;
    }
    if let Ok(a) = vec_numbers[0].parse::<u32>() {
        if let Ok(b) = vec_numbers[1].parse::<u32>() {
            return Some(Mul { a, b });
        }
    }
    None
}
fn parse_candidate(rest: &str) -> Option<Mul> {
    // Take what is inside parenthesis
    if let Some(idx_paren) = rest.find(")") {
        return parse_numbers(&rest[..idx_paren]);
    }
    None
}

fn solve_part_1(input: &str) -> u32 {
    // Rust high level parsing is not very convenient
    // let mult_chars = vec!['m', 'u', 'l', '('];
    // //let a: Vec<&str> = input.split("mul(").map(||)
    // for chunk in input.as_bytes().windows(4) {
    //     let chunk = chunk.iter().map(|&c| c as char).collect::<Vec<char>>().eq(&mult_chars);
    //     println!("{:?}", chunk.iter().map(|&c| c as char).collect::<Vec<char>>());

    // }
    let mut rest = input;
    let mut sol = 0;
    // starting with pos
    loop {
        let idx = rest.find("mul(");
        match idx {
            Some(i) => {
                rest = &rest[i + 4..];
                //println!("{:?} {:?}", i, rest);
                if let Some(mul) = parse_candidate(rest) {
                    //println!("{:?}", mul);
                    sol += mul.eval();
                }
            }
            None => break,
        }
    }

    sol
}
fn is_enabled(pre_mul: &str) -> Option<bool> {
    // what is the last do/dont of the preceeding str of a mult operation?
    let pre_mul_len = pre_mul.len();
    for i in 0..pre_mul_len {
        //println!("{:?}", &pre_mul[..pre_mul_len-i]);
        // what is going on with the ': don't(), seems to be different in mac
        if pre_mul[..pre_mul_len - i].ends_with("don't()") {
            return Some(false);
        } else if pre_mul[..pre_mul_len - i].ends_with("do()") {
            return Some(true);
        }
    }

    None
}
fn solve_part_2(input: &str) -> u32 {
    let mut rest = input;
    let mut sol = 0;
    let mut enabled = true;
    // starting with pos
    loop {
        let idx_mul = rest.find("mul(");
        match idx_mul {
            Some(idx_mul) => {
                if let Some(is_enabled) = is_enabled(&rest[..idx_mul]) {
                    enabled = is_enabled;
                }

                rest = &rest[idx_mul + 4..];

                if let Some(mul) = parse_candidate(rest) {
                    // println!("{:?}", mul);
                    if enabled {
                        sol += mul.eval();
                    }
                }

                //println!("{:?} {:?}", i, rest);
            }
            None => break,
        }
    }

    sol
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
        let input = include_str!("../../inputs/2024_03_example_do.txt");
        let solution = solve_part_2(input);
        assert_eq!(48, solution);
    }
}
