use std::{str::{Lines}, collections::HashSet};


fn solve_a(mut lines: Lines)-> u32 {
    let line = lines.next().expect("wrong");
    let mut vec = Vec::new();
    for a in line.chars(){
        vec.push(a);

    }
    let mut count = 0;

    loop {
        //let subset = vec[count..count+4];
        let mut set = HashSet::new();
        for i in 0..4 {
            set.insert(vec[count+i]);
        }
        // println!("{:?}", set);
        // println!("{:?}", count);
        if set.len() == 4{
            break;
        }

        count+=1;

    } 
    let out: u32 = count.try_into().expect("wrogn");
    out + 4
}

fn solve_b(mut lines: Lines)->u32 {
    let line = lines.next().expect("wrong");
    let mut vec = Vec::new();
    for a in line.chars(){
        vec.push(a);

    }
    let mut count = 0;

    loop {
        //let subset = vec[count..count+4];
        let mut set = HashSet::new();
        for i in 0..14 {
            set.insert(vec[count+i]);
        }
        // println!("{:?}", set);
        // println!("{:?}", count);
        if set.len() == 14 {
            break;
        }

        count+=1;

    } 
    let out: u32 = count.try_into().expect("wrogn");
    out + 14
}

fn main() {
    let lines_a = include_str!("input.txt").lines();
    let lines_b = include_str!("input.txt").lines();
    let solution_a = solve_a(lines_a);
    let solution_b = solve_b(lines_b);
    println!("Solution a: {solution_a}");
    println!("Solution b: {solution_b}");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve_a(lines);
        assert_eq!(7, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(19, solution);
    
}
}
