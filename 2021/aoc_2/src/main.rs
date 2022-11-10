fn main() {
    let s = include_str!("input.txt").lines();
    //dbg!(&s);
    for line in s {
        let mut a = line.split("\n");
        println!("{:?}", a.collect());  
    }
        
}
