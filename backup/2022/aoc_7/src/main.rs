use std::{
    collections::{HashMap, HashSet},
    str::Lines,
};

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

struct Tree {
    path: String,
    trees: Vec<Tree>,
    files: Vec<File>,
}
impl Tree {}

fn solve_a(mut lines: Lines) -> u32 {
    let _ = lines.next();
    let lines_array: Vec<&str> = lines.collect();
    let n_commands = lines_array.len();
    let mut current_dir: String = String::from("/"); //&str = "/";//
    // dict with paths and
    let mut tree: HashMap<String, File> = HashMap::new();
    let mut command_count = 0;
    loop {
        // main loop
        if command_count >= n_commands-1 {
            break;
        }
        if lines_array[command_count].contains("cd") {
            let mut unparsed = lines_array[command_count].split(" ");
            unparsed.next().expect("bla");
            unparsed.next().expect("bli");
            let dir = unparsed.next().expect("ble");
            println!("Here {:?}", dir);
            command_count+=1;
            let current_dir = current_dir;
            
            current_dir += dir;
            //current_dir +=dir;

        }else if lines_array[command_count] == "$ ls" {
            println!("ls command, must check for files");
            command_count += 1;
            if lines_array[command_count].starts_with("$") {
                println!("Got this line {:?}", lines_array[command_count]);
                command_count+=1;
                break;
            } else {
                println!("Here we are suppose to start readong files and size");
                println!("{:?}", lines_array[command_count]);
                loop {
                    if (lines_array[command_count].starts_with("$")) | (command_count==n_commands-1){
                        command_count += 1;
                        break;
                    }
                    else {
                        println!("Reading {:?}", lines_array[command_count]);
                        let mut a = lines_array[command_count].split(" ");
                        let size = a.next().expect("wrong");
                        let name = a.next().expect("wrong");
                        if size == "dir" {
                            command_count+=1;
                            break;
                        } else {
                            let name = String::from(name);
                            let size = size.parse::<u32>().unwrap();
                            println!("Current dir {:?}", current_dir);
                            tree.insert(current_dir,  File {name, size});
                            command_count += 1;

                        }

                    }

                }
                //(size, name) = scan_fmt_some!();
                //tree[current_dir] = File {};
                
                //break;
            }
            //println!("{:?}", lines_array[command_count]);

            
        }
        command_count += 1;
        //println!("{:?}", lines_array[command_count]);

    }
    println!("Tree: {:?}", tree);

    // for (i, line) in lines_array.iter().enumerate() {
    //     if *line == "$ ls" {
    //         println!("ls command, must check fro files");
    //         let count = i;
    //         for line_inner in lines_array.iter().next() {
    //             if line_inner.starts_with("$") {
    //                 println!("Got this line {:?}", line_inner);
    //                 break;
    //             } else {
    //                 println!("Here we are suppose to read files and sized");
    //                 println!("{:?}",line_inner);
    //                 //(size, name) = scan_fmt_some!();
    //                 //tree[current_dir] = File {};

    //             }
    //         }

    // let line_inner = lines_array;
    // loop {
    //     if *line[count].starts_with("$") {
    //         println!("Got this line {:?}", line_inner);
    //         break;
    //     } else {
    //         println!("Here we are suppose to read files and sized");
    //         println!("{:?}",line_inner);
    //         //(size, name) = scan_fmt_some!();
    //         //tree[current_dir] = File {};

    // }
    // for line_inner in lines_array {
    //     if line_inner.starts_with("$") {
    //         println!("Got this line {:?}", line_inner);
    //         break;
    //     } else {
    //         println!("Here we are suppose to read files and sized");
    //         println!("{:?}",line_inner);
    //         //(size, name) = scan_fmt_some!();
    //         //tree[current_dir] = File {};

    //     }

    //println!("{:?}", line);

    1
}

fn solve_b(lines: Lines) -> u32 {
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
        assert_eq!(7, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("input_test.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(19, solution);
    }
}
