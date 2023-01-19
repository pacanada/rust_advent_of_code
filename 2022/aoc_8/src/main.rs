use std::{str::{Lines}};

fn is_visible(array: &Vec<Vec<u32>>, size_x:usize, size_y:usize, a:usize, b:usize) -> bool{
    let mut is_visible_right = true;
    let mut is_visible_left = true;
    let mut is_visible_top = true;
    let mut is_visible_bottom = true;
    let tree_height = array[a][b];
    //println!("{:?}", array);
    // right
    for i in a+1..size_x {
        //println!("{}", array[i][b]);
        if tree_height<=array[i][b] {
            is_visible_right=false;
        }
    }
    //left
    for i in 0..a {
        //println!("{}", array[i][b]);
        if tree_height<=array[i][b] {
            is_visible_left=false;
        }
    }

    // bottom
    for i in b+1..size_y {
        //println!("{}", array[i][b]);
        if tree_height<=array[a][i] {
            is_visible_bottom=false;
        }
    }
    //top
    for i in 0..b {
        //println!("{}", array[i][b]);
        if tree_height<=array[a][i] {
            is_visible_top=false;
        }
    }

    is_visible_right||is_visible_left||is_visible_top||is_visible_bottom
}

fn calculate_score(array: &Vec<Vec<u32>>, size_x:usize, size_y:usize, a:usize, b:usize) -> u32{
    let mut n_visible_right = 0;
    let mut n_visible_left = 0;
    let mut n_visible_top:u32 = 0;
    let mut n_visible_bottom = 0;
    let tree_height = array[a][b];
    // right
    for i in a+1..size_x {
        n_visible_right+=1;
        if tree_height<=array[i][b] {
            break;
            
        }
    }
    //left (important the direction)
    for i in 0..a {
        n_visible_left+=1;
        if tree_height<=array[a-i-1][b] {
            break;
        }
    }

    // bottom
    for i in b+1..size_y {
        //println!("{}", array[i][b]);
        n_visible_bottom+=1;
        if tree_height<=array[a][i] {
            break;
        }
    }
    //top
    for i in 0..b {
        //println!("{}", array[i][b]);
        n_visible_top+=1;
        if tree_height<=array[a][b-i-1] {
           break;
        }
    }

    n_visible_right*n_visible_left*n_visible_bottom*n_visible_top
}


fn solve_a(lines: Lines)-> u32 {
    let mut array = Vec::new();
    for line in lines {
        let row:Vec<u32> = line.chars().map(|d| d.to_digit(10).expect("Could not parse")).collect();
        array.push(row);
    }
    let size_x = array.len();
    let size_y = array[0].len();
    //println!("{size_x}, {size_y}");
    //println!("{:?}", array);
    let n_outside = (size_x *2 + size_y*2 - 4) as u32;
    println!("{n_outside}");
    let mut n_inside:u32 = 0;
    for a in 1..size_x-1 {
        for b in 1..size_y-1 {
            //println!("{:?}",array[a][b]);
            if is_visible(&array, size_x, size_y, a, b) {
                n_inside+=1;
            }

        }

    }
   n_inside+n_outside
}

fn solve_b(lines: Lines)->u32 {
    let mut array = Vec::new();
    for line in lines {
        let row:Vec<u32> = line.chars().map(|d| d.to_digit(10).expect("Could not parse")).collect();
        array.push(row);
    }
    let size_x = array.len();
    let size_y = array[0].len();
    let mut  scores = Vec::new();
    for a in 1..size_x-1 {
        for b in 1..size_y-1 {
            //println!("{:?}",array[a][b]);
            scores.push(calculate_score(&array, size_x, size_y, a, b));

        }

    }
   *scores.iter().max().expect("Something wrong")
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
        assert_eq!(21, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(8, solution);
    
}
}
