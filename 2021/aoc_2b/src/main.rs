use std::str::Lines;

fn solve(lines: Lines) -> i32 {
    let mut position: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    for line in lines {
        let vec: Vec<&str> = line.split(" ").collect();
        let direction = vec[0];
        let value: i32 = vec[1].parse().unwrap();

        match direction {
            "up" => aim-=value,
            "down" => aim+=value,
            "forward" => {position+=value; depth+=aim*value},
            _ => println!("Unrecognized")
        }
    
    }
    println!("{:?}", (position*depth));     
    println!("{:?}", (position,depth));
    position*depth  


}
fn main() {
    let lines = include_str!("input.txt").lines();
    let solution: i32 = solve(lines);
    println!("{solution}")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve(lines);
        assert_eq!(900, solution);
    }
}
