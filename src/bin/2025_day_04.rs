
use std::{collections::HashMap, str::Lines};

fn main() {
    let lines = include_str!("../../inputs/2025_04.txt").lines();
    let solution_1: u32 = solve_part_1(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(lines);
    println!("Part 2: {solution_2}");
}


fn solve_part_1(lines: Lines) -> u32 {
// parsing
let m: Vec<Vec<char>> = lines.map(|row| row.chars().map(|e| e).collect()).collect();
let mut count=0;
for (i, row) in m.iter().enumerate() {
    for (j, c) in row.iter().enumerate() {
        let mut count_adj = 0;
        if *c=='.' {continue;}
        for offset in vec![(-1,-1), (-1,0), (-1, 1), (1,1), (1,0), (1,-1), (0,-1), (0,1)] {
            let x = i as i32+offset.0;
            let y = j as i32+offset.1;
            if (x<0) | (y<0) | (y >=(row.len()) as i32) | (x>=(m.len()) as i32) {
                continue

            }
            match m[x as usize][y as usize] {
                '@' => {count_adj+=1},
                '.' => {},
                _ => {panic!("unrecognized char")}

            }
        }
        if count_adj<4 {
            count+=1;
            //println!("{i}{j}")

        }
    }
}
//println!("{:?}", m);
//println!("{count}");
count
}



fn solve_part_2(lines: Lines) -> u32 {
let mut m: Vec<Vec<char>> = lines.map(|row| row.chars().map(|e| e).collect()).collect();
let mut total_count=0;

loop {
    let mut positions_to_remove: Vec<(usize, usize)> = vec![];
    let mut count=0;
    for (i, row) in m.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let mut count_adj = 0;
            if *c=='.' {continue;}
            for offset in vec![(-1,-1), (-1,0), (-1, 1), (1,1), (1,0), (1,-1), (0,-1), (0,1)] {
                let x = i as i32+offset.0;
                let y = j as i32+offset.1;
                if (x<0) | (y<0) | (y >=(row.len()) as i32) | (x>=(m.len()) as i32) {
                    continue

                }
                match m[x as usize][y as usize] {
                    '@' => {count_adj+=1},
                    '.' => {},
                    _ => {panic!("unrecognized char")}

                }
            }
            if count_adj<4 {
                count+=1;
                positions_to_remove.push((i,j));
                //println!("{i}{j}")

            }
        }
    }
    if positions_to_remove.is_empty() {
        break
    }
    for pos in positions_to_remove {
        m[pos.0][pos.1] = '.';
    }
    total_count+=count;
}
//println!("{:?}", m);
//println!("{count}");
total_count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2025_04_example.txt").lines();
        let solution = solve_part_1(lines);
        assert_eq!(13, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2025_04_example.txt").lines();
        let solution = solve_part_2(lines);
        assert_eq!(43, solution);
    }
}
