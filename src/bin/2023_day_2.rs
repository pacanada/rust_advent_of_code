use std::{
    str::{Chars, FromStr, Lines},
    string::ParseError,
    sync::Arc,
    vec,
};

#[derive(PartialEq, Debug)]
enum Color {
    Green,
    Red,
    Blue,
}

#[derive(Debug)]
struct Cubes {
    color: Color,
    qty: u8,
}
impl Cubes {
    fn is_contained_in(&self, other: &Cubes) -> bool {
        (self.color == other.color) & (self.qty <= other.qty)
    }
}
impl FromStr for Cubes {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let qty = split.next().unwrap().parse::<u8>().unwrap();
        let color_str = split.next().unwrap();
        let color = match color_str {
            "blue" => Color::Blue,
            "red" => Color::Red,
            "green" => Color::Green,
            _ => panic!(),
        };
        let cubes = Cubes { qty, color };
        Ok(cubes)
    }
}
#[derive(Debug)]
struct CubesBatch {
    cubes: Vec<Cubes>,
}
impl CubesBatch {
    fn is_contained_in(self, other: &CubesBatch) -> bool {
        // for a batch contained in another batch, all of the batches from self must be a subset of
        // the other CubesBatch
        let is_contained: bool = self
            .cubes
            .iter()
            .map(|cube| {
                other
                    .cubes
                    .iter()
                    .map(|other_cube| cube.is_contained_in(other_cube))
                    .any(|x| x)
            })
            .all(|x| x);
        is_contained
    }
    fn get_power(self)->u32 {
       self.cubes.iter().map(|cubes| cubes.qty as u32).product()
    }
}
impl FromStr for CubesBatch {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s
            .split(",")
            .map(|batch| batch.parse::<Cubes>().unwrap())
            .collect();
        let cubes_batch = CubesBatch { cubes };
        Ok(cubes_batch)
    }
}

fn main() {
    let lines = include_str!("../../inputs/2023_2.txt").lines();
    let solution_1: u32 = solve_a(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_b(lines);
    println!("Part 2: {solution_2}");
}

fn solve_a(lines: Lines) -> u32 {
    let reference = " 12 red, 13 green, 14 blue".parse::<CubesBatch>().unwrap();
    let mut count:u32 = 0;
    //dbg!(&reference);

    for (i, line) in lines.enumerate() {
        let mut split = line.split(":");
        let _ = split.next().unwrap();
        let batches = split.next().unwrap();
        //dbg!(&batches);
        let is_contained = batches
            .split(";")
            .map(|batch| batch.parse::<CubesBatch>().unwrap().is_contained_in(&reference)).all(|x| x);

        if is_contained {
            count+=i as u32+1;

        }

    }
    count
}
#[derive(Debug)]
struct Game {
    batches: Vec<CubesBatch>
}
impl FromStr for Game {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batches = s
        .split(";")
        .map(|batch| batch.parse::<CubesBatch>().unwrap()).collect();
    Ok(Game { batches })
        
    }
}
impl Game {
    fn get_maximum(self) -> CubesBatch {
        let mut max_qty_green = 0;
        let mut max_qty_red = 0;
        let mut max_qty_blue = 0;
        for cubes_batch in self.batches {
            for cube in cubes_batch.cubes {
                match cube.color {
                    Color::Blue => if cube.qty > max_qty_blue {max_qty_blue=cube.qty},
                    Color::Green => if cube.qty > max_qty_green {max_qty_green=cube.qty},
                    Color::Red => if cube.qty > max_qty_red {max_qty_red=cube.qty},
                }
            }

            }
        CubesBatch { cubes: vec![
            Cubes {qty: max_qty_blue, color: Color::Blue}, 
            Cubes {qty: max_qty_red, color: Color::Red},
            Cubes {qty: max_qty_green, color: Color::Green}] }
        }
    
    }


fn solve_b(lines: Lines) -> u32 {
    let mut sum = 0;
    for line in lines {
        let mut split = line.split(":");
        let _ = split.next().unwrap();
        let batches = split.next().unwrap();
        let game = batches.parse::<Game>().unwrap();
        let power = game.get_maximum().get_power();
        sum+=power;
        
        
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2023_2_a_example.txt").lines();
        let solution = solve_a(lines);
        assert_eq!(8, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2023_2_b_example.txt").lines();
        let solution = solve_b(lines);
        assert_eq!(2286, solution);
    }
}
