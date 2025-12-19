use std::str::Lines;

fn main() {
    let lines = include_str!("../../inputs/2025_06.txt").lines();
    let solution_1: u64 = solve_part_1(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(lines);
    println!("Part 2: {solution_2}");
}

fn solve_part_1(lines: Lines) -> u64 {
    let mut v: Vec<Vec<u64>> = vec![];
    let mut ops: Vec<char> = vec![];
    //let mut ops: Vec<char>=vec![];
    for line in lines {
        if line.contains("*") {
            ops = line
                .split_whitespace()
                .map(|op| op.parse().expect(&format!("Could not parse {op}")))
                .collect();
        } else {
            //let _ = line.split(" ").inspect(|x| println!("{x}")).collect();
            let row_number: Vec<u64> = line
                .split_whitespace()
                .map(|number| {
                    number
                        .replace(" ", "")
                        .parse()
                        .expect(&format!("Could not parse {number}"))
                })
                //.inspect(|x| println!("{x}"))
                .collect();

            v.push(row_number);
        }
    }
    println!("{:?}", v);
    println!("{:?}", ops);
    // can we do it with fold?
    let mut total = 0;
    for (i, op) in ops.iter().enumerate() {
        match op {
            '*' => total += v.iter().fold(1, |acc, x| acc * x[i]),
            '+' => total += v.iter().fold(0, |acc, x| acc + x[i]),
            _ => panic!("not recognized operation"),
        }
    }

    total
}

fn solve_part_2(lines: Lines) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2025_06_example.txt").lines();
        let solution = solve_part_1(lines);
        assert_eq!(4277556, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2025_06_example.txt").lines();
        let solution = solve_part_2(lines);
        assert_eq!(4, solution);
    }
}
