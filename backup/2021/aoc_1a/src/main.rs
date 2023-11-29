fn main() {
    let s = include_str!("input.txt").lines();
    let mut previous_measure: u32 = u32::MAX;
    let mut count: u32 = 0;
    for measure in s {
        let current_measure: u32 = measure.parse::<u32>().expect("Could not parse int");
        if current_measure > previous_measure {
            count+=1;
        } 
        previous_measure = current_measure;
    }
    println!("{}", count);
}
