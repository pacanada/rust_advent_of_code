use std::{collections::HashMap, str::Lines};

fn main() {
    let lines = include_str!("../../inputs/2024_01.txt").lines();
    let solution_1: i32 = solve_part_1(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(lines);
    println!("Part 2: {solution_2}");
}

fn solve_part_1(lines: Lines) -> i32 {
    let mut vec_1 = Vec::<u32>::new();
    let mut vec_2 = Vec::<u32>::new();
    for line in lines {
        let mut split = line.split_whitespace();
        let element_1 = split.next().unwrap();
        let element_2 = split.next().unwrap();
        vec_1.push(element_1.parse().unwrap());
        vec_2.push(element_2.parse().unwrap());
    }
    vec_1.sort();
    vec_2.sort();

    vec_1
        .iter()
        .zip(vec_2.iter())
        .map(|(a, b)| (*a as i32 - *b as i32).abs())
        .sum()
}

fn solve_part_2(lines: Lines) -> u32 {
    let mut vec_1 = Vec::<u32>::new();
    let mut map_count = HashMap::<u32, u32>::new();
    for line in lines {
        let mut split = line.split_whitespace();
        let element_1: u32 = split.next().unwrap().parse().unwrap();
        let element_2: u32 = split.next().unwrap().parse().unwrap();
        vec_1.push(element_1);
        // count of occurrences
        match map_count.get(&element_2) {
            Some(v) => {
                map_count.insert(element_2, v + 1);
            }
            None => {
                map_count.insert(element_2, 1);
            }
        }
    }

    vec_1
        .iter()
        .map(|x| match map_count.get(x) {
            Some(v) => x * v,
            None => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2024_01_example.txt").lines();
        let solution = solve_part_1(lines);
        assert_eq!(11, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2024_01_example.txt").lines();
        let solution = solve_part_2(lines);
        assert_eq!(31, solution);
    }
}
