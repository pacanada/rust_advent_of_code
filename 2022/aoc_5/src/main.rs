use std::{str::{Lines}};

fn get_indexes_based_on_width(width: u32) -> Vec<u32> {
    // Getting the indexes we want to check for boxes
    let mut indexes = vec![1];
    for i in 0..width/3-1 {
        indexes.push((i+1)*4+1);
    }
    indexes

}

fn parse_instructions(lines: Lines) ->Vec<Vec<u32>>{
    let mut instructions = Vec::new();
    for line in lines {
        if line.contains("move") {
            // keep the ones that are numbers
            let numbers: String = line.chars().filter(|c| c.is_digit(10)).collect();
            let instruction: Vec<u32> = numbers.chars().map(|c| c.to_digit(10).unwrap()).collect();
            instructions.push(instruction);
            //dbg!(numbers);
        }
        
    }
    //dbg!(instructions);
    instructions
}
fn parse_stacks(lines: Lines) -> Vec<Vec<char>>{
    let mut lines_ = lines.clone();
    let width =  lines_.next().expect("Something wrong").len() as u32;
    let mut array = Vec::new();
    for line in lines {
        if line.contains("1") {
            // break when we are done parsing the stacks
            break;
        }
        // should not be calculated everytime but it complains about ownership
        let indexes = get_indexes_based_on_width(width);
        let mut row = Vec::new();
        for (i,c) in line.chars().enumerate() {
            let i: u32 = i.try_into().unwrap();
        if (indexes.contains(&i)) {//& (c!=' ') {
                row.push(c)
        }
        }
        array.push(row);

        //dbg!(line);
    }
    array.reverse();

    // transpose it
    
    let mut transposed_array = Vec::new();
    let max_height=8;// = array[0].len();
    let n_piles=9; // = array.len();
    for i in 0..n_piles {
        let mut pile = Vec::new();
        for j in 0..max_height {
            if array[j][i] != ' ' {
                pile.push(array[j][i])
            }
            
        }
        transposed_array.push(pile);
        // dbg!(i);

    }
    transposed_array

}

fn solve_a(lines: Lines)-> String {
    //let mut stacks = Vec::new();
    // what is the total number of piles
    let lines_ = lines.clone();
    let mut array = parse_stacks(lines);
    // first index is pile, second is height
    // dbg!(array);
    // dbg!(array[0][0]);
    // dbg!(array[0][1]);
    // dbg!(array[2][0]);

    let instructions = parse_instructions(lines_);
    for instruction in instructions {
        let quantity = instruction[0];
        let initial_pile = instruction[1] - 1;
        let target_pile = instruction[2] - 1;
        let mut buffer = Vec::new();
        dbg!(quantity, initial_pile, target_pile);
        

        for i in 0..quantity {
            buffer.push(array[initial_pile as usize].pop().expect("something worng")); 

        }
        //dbg!(buffer);
        for i in buffer {
            array[target_pile as usize].push(i);
        }
    }

    // crate on top
    let mut top = String::new();
    for mut pile in array {
        top.push(pile.pop().expect("something wrong"));
    }
    
    top
}

fn solve_b(lines: Lines)->&str {
    ""
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
        assert_eq!("CMZ", solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve_b(lines);
        assert_eq!("", solution);
    
}
}
