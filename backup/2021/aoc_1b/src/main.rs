
fn main() {
    let s = include_str!("input.txt").lines();
    let mut measurements: Vec<u32> = Vec::new();

    // reading as vector
    for measurement in s {
        measurements.push(measurement.parse::<u32>().expect("Could not parse int"));
    }

    let mut previous_sum_measure: u32 = u32::MAX;
    let mut count: u32 = 0;
    for (index,measure) in measurements[1..measurements.len()-1].iter().enumerate() {
        let current_sum_measure: u32 = measurements[index..index+3].iter().sum();
        if current_sum_measure > previous_sum_measure {
            count+=1;
        } 
        previous_sum_measure = current_sum_measure;
    }
    println!("{}", count);
}
