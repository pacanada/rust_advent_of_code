use core::num;
use std::{str::{Lines}, collections::{HashMap, HashSet}};

fn is_away(x_0: i32, y_0: i32, x_1:i32, y_1:i32) -> bool {
    let d_x = x_0-x_1;
    let d_y = y_0-y_1;
    d_x*d_x+d_y*d_y > 2

    
}

fn solve_a(lines: Lines)-> u32 {
//let mut positions : HashMap<(i32,i32), i32> = HashMap::new(); 
    let mut positions : HashSet<(i32,i32)> = HashSet::new(); 
    let mut t_x_0 = 0;
    let mut t_y_0 = 0;
    let mut h_x_0 = 0;
    let mut h_y_0 = 0;
    let mut last_h_x_0 = 0;
    let mut last_h_y_0 = 0;
    for line in lines {
        let mut split = line.split(" ");
        let direction = split.next().expect("something went wrong");
        let number = split.next().expect("something went wrong").parse().expect("could not parse");

        //println!("{} {}", direction, number);
        
        for i in 0..number {
            last_h_x_0 = h_x_0;
            last_h_y_0 = h_y_0;

            if direction=="D" {
                h_y_0-=1;

            } else if direction=="U" {
                h_y_0+=1;
                
            } else if direction =="L" {
                h_x_0-=1;
                
            } else if direction == "R" {
                h_x_0+=1;
            }
            //println!("H pos {} {}", h_x_0, h_y_0);
            // if distance is more than sqrt(2), update T to most recent position known from H
            if is_away(h_x_0, h_y_0, t_x_0, t_y_0) {
                t_x_0 = last_h_x_0;
                t_y_0 = last_h_y_0;
                //println!("Updating");
                //println!("T pos {} {}", t_x_0, t_y_0);
                positions.insert((t_x_0, t_y_0));
                // if (t_x_0, t_y_0) in positions.keys() {
                //     println!("Already visited");
                // } else {
                //     positions[(t_x_0, t_y_0)] = 1;
                // }

            }

        }
        
    }
    // inital positions
    positions.len() as u32
   
}

fn solve_b(lines: Lines)->u32 {
   1
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
        assert_eq!(13, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(8, solution);
    
}
}
