use std::str::Lines;

fn main() {
    let lines = include_str!("../../inputs/2025_01_a.txt").lines();
    let solution_1: u32 = solve_part_1(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(lines);
    println!("Part 2: {solution_2}");
}


fn solve_part_1(lines: Lines) -> u32 {
    let mut start: i32 = 50;
    let mut count = 0;
    for line in lines {
        let direction = line.chars().nth(0).unwrap();
        let number: i32 = line[1..].parse().unwrap();
        #[cfg(feature="dbg")]
        println!("{direction} {number}");
        match direction {
            'L' => {
                start-=number;
                if start < 0 {
                    start+=100*(1+(-start)/100);
                }
            },
            'R' => {
                start+=number;
                if start >= 100 {

                    start-=100*(start/100);
                }
            }
            _ => panic!()
            
        }
        //println!("{start}");
        if start == 0 {
            count+=1;
        }
        // could happen in L branch
        if start == 100 {
            start=0;
            count+=1;
        }
    }
    count
}
fn solve_part_2(lines: Lines) -> u32 {
    let mut start: i32 = 50;
    let mut count: i32 = 0;
    for line in lines {
        let direction = line.chars().nth(0).unwrap();
        let number: i32 = line[1..].parse().unwrap();
        #[cfg(feature="dbg")]
        println!("{direction} {number}");
        match direction {
            'L' => {
                
                for i in 1..=number {
                    start-=1;
                    println!("{start} {i}");
                    if start==0 {
                        count+=1;
                    }
                    if start==-1 {
                        start=99;
                    }
                }
            },
            'R' => {
                for i in 1..=number {
                    start+=1;
                    if start==100 {
                        count+=1;
                        start=0;
                    }
                }
            }
            _ => panic!()
            
        }

        if start == 0 {
            //count+=1;
        }
        // could happen in L branch
        if start == -100 {
            start=0;
            println!("now")
            //count+=1;
        }
        #[cfg(feature="dbg")]
        println!("{start} {count}");
    }

    count as u32
}

// fn solve_part_2(lines: Lines) -> u32 {
//     let mut start: i32 = 50;
//     let mut count: i32 = 0;
//     for line in lines {
//         let direction = line.chars().nth(0).unwrap();
//         let number: i32 = line[1..].parse().unwrap();
//         #[cfg(feature="dbg")]
//         println!("{direction} {number}");
//         match direction {
//             'L' => {
//                 start-=number;
//                 if start < 0 {
//                     // how many times has passed 0?
//                     count+=1+(-start)/100;
//                     if start==-100 {
//                         //println!("now")
//                         start=0;
//                         //count-=1;

//                     } else {
//                     start+=100*(1+(-start)/100);
//                     }

//                 }
//             },
//             'R' => {
//                 start+=number;
//                 if start > 100 {
//                     // how many times has passed 0?
//                     count+=(start)/100;

//                     start-=100*(start/100);
//                 }
//             }
//             _ => panic!()
            
//         }

//         if start == 0 {
//             //count+=1;
//         }
//         // could happen in L branch
//         if start == -100 {
//             start=0;
//             println!("now")
//             //count+=1;
//         }
//         #[cfg(feature="dbg")]
//         println!("{start} {count}");
//     }

//     count as u32
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2025_01_a_example.txt").lines();
        let solution = solve_part_1(lines);
        assert_eq!(3, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2025_01_a_example.txt").lines();
        let solution = solve_part_2(lines);
        assert_eq!(6, solution);
    }
}
