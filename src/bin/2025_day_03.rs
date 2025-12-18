use std::{collections::HashMap, str::Lines};

fn main() {
    let lines = include_str!("../../inputs/2025_03_example.txt").lines();
    let solution_1: u32 = solve_part_1(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u64 = solve_part_2(lines);
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
//
// From vector of elements to a hasmap containing the elements and their position 0010 -> {0: [0,1,3], 1: [2] }
//
fn parse_to_array_of_vec_struct(v: Vec<u32>)-> HashMap<u32, Vec<usize>> {
    let mut h: HashMap<u32, Vec<usize>> = HashMap::new();
    for (pos, element) in v.iter().enumerate() {

        if let Some(vec) = h.get_mut(element) {
            vec.push(pos)

        } else {
            // initialize
            h.insert(*element, vec![pos]); 

        }

        
    }
    h
}


fn is_possible(h: &HashMap<u32, Vec<usize>>, number: u64)->bool {
    let vec_of_digits = parse_vec_from_line(&number.to_string());
    let mut last_index: i32 = -1;
    for digit in vec_of_digits {
        //println!("Considering {digit}. Last index {last_index}");
        if let Some(vec_pos) = h.get(&digit) {
            let mut next_position = None;
            for i in vec_pos {
                if *i as i32> last_index {
                    next_position = Some(i);
                    //println!("{:?}", next_position);
                    break
                }
            }
            if next_position.is_some() {
                last_index = *next_position.unwrap() as i32;
            } else {
                return false
            }
        } else {
            return false
        }

    }
    true

}
fn max_number_of_n_digits(h: &HashMap<u32, Vec<usize>>, n: u32)->u64{
    //let mut h2 = h.clone();
    println!("{}-{}", 10_u64.pow(n+1)-1,10_u64.pow(n) );
    println!("{:?}", h);
    println!("{}", is_possible(h, 987654321111));
    for i in (10_u64.pow(n-1)..10_u64.pow(n)-1).rev() {
        //println!("Testing {i}");
        if is_possible(h, i) {
            return i
        }
    //for i in 100..1 {
        //println!("{i}");
        //println!("{}", is_possible(h, i));
    

    }
0
}

fn find_max_of_vector(v: &[u32]) -> Option<(u32, usize)> {
    let max = v.iter().max();
    if !max.is_some() {
        return None
    }
    let max = max.unwrap();
    let pos = v.iter().position(|x| x==max).unwrap();
    Some((*max, pos))

}

fn solve_part_2(lines: Lines) -> u64 {
    let mut max_accum = 0;
    for line in lines {

        let v = parse_vec_from_line(line);
        // backtracking?
        println!("{:?}", find_max_of_vector(&v[..]));
        println!("{:?}", find_max_of_vector(&v[1..]));
        let h = parse_to_array_of_vec_struct(v);
        println!("{}", line);
        // other approach
        // find maximum of the remaining of vector, and repeat until we have n digits, if not, reduce by one the last searched 
        // max_accum+=max_number_of_n_digits(&h, 12);
        //println!("{:?}", h);
        // what is the max number in order of 12 elements
    }
    max_accum
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
