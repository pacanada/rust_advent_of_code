use std::str::{FromStr, Lines};
#[derive(Debug)]
struct Rule{
    left: u32,
    right: u32
}
// overkill but cool
#[derive(Debug)]
struct ParseRuleError;
impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once("|")
            .ok_or(ParseRuleError)?;

        let left_fromstr = x.parse::<u32>().map_err(|_| ParseRuleError)?;
        let right_fromstr = y.parse::<u32>().map_err(|_| ParseRuleError)?;

        Ok(Rule { left: left_fromstr, right: right_fromstr })
    }
}

fn main() {
    let input = include_str!("../../inputs/2024_05.txt");
    let solution_1: u32 = solve_part_1(input);
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(input);
    println!("Part 2: {solution_2}");
}
fn check_rule(update: &Vec<u32>, rules: &Vec<Rule>)-> bool {
    for rule in rules {
        if update.contains(&rule.left) & update.contains(&rule.right) {
            let idx_left_element = update.iter().position(|&n| n == rule.left).unwrap();
            let idx_right_element = update.iter().position(|&n| n == rule.right).unwrap();
            if idx_left_element<idx_right_element {
                continue;
            } else {
                return false;
            }
       
        } else {
            continue;
        }
    }
    true
}
fn fix_update(update: &Vec<u32>, rules: &Vec<Rule>)-> Vec<u32> {
    let mut fixed_updated: Vec<u32> = update.clone();
    while !check_rule(&fixed_updated, rules) {
    for rule in rules {
        if fixed_updated.contains(&rule.left) & fixed_updated.contains(&rule.right) {
            let idx_left_element = fixed_updated.iter().position(|&n| n == rule.left).unwrap();
            let idx_right_element = fixed_updated.iter().position(|&n| n == rule.right).unwrap();
            if idx_left_element<idx_right_element {
                continue;
            } else {
                fixed_updated[idx_right_element]=rule.left;
                fixed_updated[idx_left_element]=rule.right;
            }
       
        } else {
            continue;
        }
    }
}
    fixed_updated
}

fn solve_part_1(input: &str) -> u32 {
    let mut iter = input.split("\n\n");
    let rules: Vec<Rule> = iter.next().unwrap().lines().map(|rule|rule.parse().unwrap()).collect();
    let updates: Vec<Vec<u32>> = iter
        .next()
        .unwrap()
        .split("\n")
        .map(|update| {
            update
                .split(",")
                .map(|page| page.parse().unwrap())
                .collect()
        })
        .collect();
    let mut sol = 0;
    for update in updates {
        if check_rule(&update, &rules) {
            sol+=update[update.len()/2];
        }

    }
    sol
}

fn solve_part_2(input: &str) -> u32 {
    let mut iter = input.split("\n\n");
    let rules: Vec<Rule> = iter.next().unwrap().lines().map(|rule|rule.parse().unwrap()).collect();
    let updates: Vec<Vec<u32>> = iter
        .next()
        .unwrap()
        .split("\n")
        .map(|update| {
            update
                .split(",")
                .map(|page| page.parse().unwrap())
                .collect()
        })
        .collect();
    let mut sol = 0;
    for update in updates {
        if !check_rule(&update, &rules) {
            let fixed_update = fix_update(&update, &rules);
            //println!("{:?}", fixed_update);
            sol+=fixed_update[fixed_update.len()/2];
        }

    }
    sol
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2024_05_example.txt");
        let solution = solve_part_1(lines);
        assert_eq!(143, solution);
    }
    #[test]
    fn question_b() {
        let input = include_str!("../../inputs/2024_05_example.txt");
        let solution = solve_part_2(input);
        assert_eq!(123, solution);
    }
}
