use std::{str::{Lines, Chars}};

// fn get_common_char<'a>(f: &'a str, l:&'a str)->&'a str {
//     for bla in f.chars() {
//        println!("{:?}", bla)
//     }
//     //f.chars().any(|c| l.contains(c))
// }

fn get_common_char<'a>(f: &'a str, l: &'a str) -> char  {
    let mut out: char = 'a';
    for c in f.chars() {
        if l.contains(c) {
            out = c;
            break;
        }
    }
    out
}

fn get_common_chars<'a>(f: &'a str, l: &'a str) ->String{
    let mut vec = Vec::new();
    for c in f.chars() {
        if l.contains(c) {
            vec.push(c); 
        }
    }
    let out: String = vec.into_iter().collect();
    out
}

fn solve_a(lines: Lines)-> u32 {
    let mut total = 0;
    for line in lines {
        let half_index = line.len()/2;
        let first = &line[..half_index];
        let last = &line[half_index..];
        //println!("{:?}", first);
        //println!("{:?}", last);
        let c = get_common_char(first , last);
        //dbg!(c);
        let eval = c.to_digit(36).expect("Something went worng");
        if c.is_lowercase() {
            // a is 10 and want to be 1
            total += eval-9;
        } else {
            total += eval-9+26;
        }
    }
    total
}

fn solve_b(lines: Lines)->u32 {
    let mut total = 0;
    // clone it to be able to calculate the length and also iterate with next
    let mut lines_ = lines.clone();
    let a: Vec<&str> = lines.collect();
    let length = a.len();
    for i in 0..length/3 {
        let first = lines_.next().expect("Something wrong");
        let second = lines_.next().expect("Something wrong");
        let third = lines_.next().expect("Something wrong");
        let common_chars_first_two = get_common_chars(&first, &second);
        let common_per_group = get_common_chars(&common_chars_first_two, &third);
        // the char could be duplicated
        let c = common_per_group.chars().next().unwrap();
        let eval = c.to_digit(36).expect("Something went worng");
        if c.is_lowercase() {
            // a is 10 and want to be 1
            total += eval-9;
        } else {
            total += eval-9+26;
        }

    }
    total
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
        assert_eq!(157, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(70, solution);
    
}
}

