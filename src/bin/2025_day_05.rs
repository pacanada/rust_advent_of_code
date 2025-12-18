use std::{collections::HashSet, str::Lines};

fn main() {
    let lines = include_str!("../../inputs/2025_05.txt").lines();
    let solution_1: u32 = solve_part_1(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(lines);
    println!("Part 2: {solution_2}");
}


fn solve_part_1(lines: Lines) -> u32 {
    // parse
    let mut ranges: Vec<(u64, u64)>=vec![];
    let mut ids: Vec<u64>=vec![];
    for line in lines {
        if line.contains("-") {
            let tmp: Vec<&str> = line.split("-").collect();
            let lower_limit: u64 = tmp[0].parse().unwrap();
            let upper_limit: u64 = tmp[1].parse().unwrap();
            ranges.push((lower_limit, upper_limit));

        }
        else {
            if let Ok(id) = line.parse::<u64>() {
                ids.push(id);

            }
        }
    }
    // fresh ids
    let mut count = 0;
    for id in ids {
        for range in &ranges{
            if (id>=range.0) & (id<=range.1) {
                count+=1;
                break

            }
        }
    }
    println!("{:?}", ranges);
    //println!("{:?}",&ids);

    count
}

fn solve_part_2(lines: Lines) -> u32 {
        // parse
    let mut ranges: Vec<(u64, u64)>=vec![];
    for line in lines {
        if line.contains("-") {
            let tmp: Vec<&str> = line.split("-").collect();
            let lower_limit: u64 = tmp[0].parse().unwrap();
            let upper_limit: u64 = tmp[1].parse().unwrap();
            if lower_limit>upper_limit {
                panic!("not expected")
            }
            ranges.push((lower_limit, upper_limit));

        }
    }
    //ofc brute force doesnt work
    let mut simplified_ranges: Vec<(u64, u64)> = vec![];
    for (index, range) in ranges.iter().enumerate() {
        // if range is contained fully in any of the simplified, drop that
        for simplified_range in &mut simplified_ranges {
            if (range.0>=simplified_range.0) & (range.1<=simplified_range.1) {
                continue;
            }
            if (range.0>=simplified_range.0) & (range.1>=simplified_range.1) {
                simplified_range.1=range.1;
            }
            if (range.0<=simplified_range.0) & (range.1<=simplified_range.1) {
                simplified_range.0=range.0;
            }
            if (range.1<simplified_range.0) | (range.0>simplified_range.1) {
                simplified_ranges.push(range.clone());

            }
        }
    }

    //let mut set_of_ids: HashSet<u64>= HashSet::new();
    // for range in ranges {
    //     for i in range.0..=range.1 {
    //         set_of_ids.insert(i);
    //     }
    // }
    // set_of_ids.len() as u32
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2025_05_example.txt").lines();
        let solution = solve_part_1(lines);
        assert_eq!(3, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2025_05_example.txt").lines();
        let solution = solve_part_2(lines);
        assert_eq!(14, solution);
    }
}
