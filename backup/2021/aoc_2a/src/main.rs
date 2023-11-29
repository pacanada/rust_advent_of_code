use std::str::Lines;

fn solve(lines: Lines) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for line in lines {
        let vec: Vec<&str> = line.split(" ").collect();
        let direction = vec[0];
        let value: i32 = vec[1].parse().unwrap();

        match direction {
            "up" => y-=value,
            "down" => y+=value,
            "forward" => x+=value,
            _ => println!("Unrecognized")
        }

        //let direction = line.split(" ").next().as_str();
        //let (a,b) = line.split(" ").unwrap();
    
    }
    println!("{:?}", (x*y));     
    println!("{:?}", (x,y));
    x*y  


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
        assert_eq!(150, solution);
    }
}
