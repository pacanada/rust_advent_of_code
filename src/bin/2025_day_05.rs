use std::{collections::HashSet, str::Lines};

fn main() {
    let lines = include_str!("../../inputs/2025_05.txt").lines();
    let solution_1: u32 = solve_part_1(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u64 = solve_part_2(lines);
    println!("Part 2: {solution_2}");
}


fn solve_part_1(lines: Lines) -> u32 {
    // parse
    let mut ranges: Vec<(u64, u64)>=vec![];
    let mut ids: Vec<u64>=vec![];
    for line in lines {
        if line.contains("-") {
            let tmp: Vec<&str> = line.split("-").collect();
            let lower_limit: u64 = tmp[0].parse().unwrap();
            let upper_limit: u64 = tmp[1].parse().unwrap();
            ranges.push((lower_limit, upper_limit));

        }
        else {
            if let Ok(id) = line.parse::<u64>() {
                ids.push(id);

            }
        }
    }
    // fresh ids
    let mut count = 0;
    for id in ids {
        for range in &ranges{
            if (id>=range.0) & (id<=range.1) {
                count+=1;
                break

            }
        }
    }
    println!("{:?}", ranges);
    //println!("{:?}",&ids);

    count
}
// struct Range {
//     lower_limit: u64,
//     upper_limit: u64,
// }
// struct Ranges {
//     ranges: Vec<Range>
// }
// impl Ranges {
//     fn union(self, other: Range)-> Vec<Ranges> {
//         vec![]  }
// }
// fn union(v: Vec<(u64, u64)>)->Vec<(u64, u64)>{
//     return v
// }
fn solve_part_2(lines: Lines) -> u64 {
        // parse
    let mut ranges: Vec<(u64, u64)>=vec![];

    for line in lines {
        if line.contains("-") {
            let tmp: Vec<&str> = line.split("-").collect();
            let lower_limit: u64 = tmp[0].parse().unwrap();
            let upper_limit: u64 = tmp[1].parse().unwrap();
            if lower_limit>upper_limit {
                panic!("not expected")
            }
            ranges.push((lower_limit, upper_limit));
            //lower_limits.push(lower_limit);
            //upper_limits.push(upper_limit);

        }
    }
    println!("{:?}", ranges);
    
    //ranges.sort_by(|first,second| first.0.cmp(&second.0));
    let mut ranges_sort_by_lower = ranges.clone();
    ranges_sort_by_lower.sort_by(|first,second| first.0.cmp(&second.0));

    // let mut ranges_sort_by_upper = ranges.clone();
    // ranges_sort_by_upper.sort_by(|first,second| first.1.cmp(&second.1));
    println!("Ordered: {:?}", ranges_sort_by_lower);
    //println!("{:?}", ranges_sort_by_upper);

    loop {
        let mut new: Vec<(u64, u64)> = vec![];
        let mut last_should_be_included = true;
        let mut skip_next: bool =false;
        for a in ranges_sort_by_lower.windows(2) {
            if skip_next {
                skip_next=false;
                // in case is the last one
                last_should_be_included=true;
                continue;
            }
            if a[0].1>= a[1].0 {
                // max is to account for the possibility that first is inside second
                new.push((a[0].0, a[1].1.max(a[0].1)));
                last_should_be_included=false;
                skip_next=true;
                
                
            } else {
                new.push((a[0].0, a[0].1));
                last_should_be_included=true;
                skip_next=false;
            }
        }
        // add the last once we are done
        if last_should_be_included {
            new.push((ranges_sort_by_lower.last().unwrap().0, ranges_sort_by_lower.last().unwrap().1));
        }

        
        println!("New: {:?}", new);
        println!("{}", new.len());
        if ranges_sort_by_lower.len()==new.len() {break}
        ranges_sort_by_lower = new.clone();
        //ranges_sort_by_lower.sort_by(|first,second| first.0.cmp(&second.0));


    }
    println!("{:?}", ranges_sort_by_lower);
    let mut count=0;
    // check
    for range in ranges_sort_by_lower.windows(2) {
        if range[0].1>=range[1].0 {
            println!("Here: {:?}  {:?}", range[0], range[1]);
        }
        

    }
    for range in ranges_sort_by_lower {
        count+=range.1-range.0+1;

    }
    return count
    
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2025_05_example.txt").lines();
        let solution = solve_part_1(lines);
        assert_eq!(3, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2025_05_example.txt").lines();
        let solution = solve_part_2(lines);
        assert_eq!(14, solution);
    }
}
