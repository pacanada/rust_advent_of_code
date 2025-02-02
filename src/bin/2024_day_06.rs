use std::str::Lines;
#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}
#[derive(Debug)]
struct Player {
    x: usize,
    y: usize,
    direction: Direction,
}
impl Player {
    fn step(&mut self) {
        match self.direction {
            Direction::UP => self.x -= 1,
            Direction::DOWN => self.x += 1,
            Direction::RIGHT => self.y += 1,
            Direction::LEFT => self.y -= 1,
            _ => panic!(),
        }
    }
    fn turn_right(&mut self) {
        match self.direction {
            Direction::UP => self.direction = Direction::RIGHT,
            Direction::DOWN => self.direction = Direction::LEFT,
            Direction::RIGHT => self.direction = Direction::DOWN,
            Direction::LEFT => self.direction = Direction::UP,
            _ => panic!(),
        }
    }
    fn look_ahead_coords(&self) -> (usize, usize) {
        match self.direction {
            Direction::UP => return (self.x - 1, self.y),
            Direction::DOWN => return (self.x + 1, self.y),
            Direction::RIGHT => return (self.x, self.y + 1),
            Direction::LEFT => return (self.x, self.y - 1),
        }
    }
}

fn main() {
    let lines = include_str!("../../inputs/2024_06_example.txt").lines();
    let solution_1: u32 = solve_part_1(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(lines);
    println!("Part 2: {solution_2}");
}

fn solve_part_1(lines: Lines) -> u32 {
    let map: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut p = None;
    let len_x = map.len();
    let len_y = map[0].len();
    for i in 0..len_x {
        for j in 0..len_y {
            match map[i][j] {
                '>' => {
                    p = Some(Player {
                        x: i,
                        y: j,
                        direction: Direction::RIGHT,
                    });
                }
                '<' => {
                    p = Some(Player {
                        x: i,
                        y: j,
                        direction: Direction::LEFT,
                    });
                }
                '^' => {
                    p = Some(Player {
                        x: i,
                        y: j,
                        direction: Direction::UP,
                    });
                }
                'v' => {
                    p = Some(Player {
                        x: i,
                        y: j,
                        direction: Direction::DOWN,
                    });
                }
                _ => (),
            }
        }
    }
    // // p = p.unwrap();
    // if let Some(p_1) = p  {
    //     println!("");
    // }
    let mut p1 = p.unwrap();
    let mut positions_visited = vec![(p1.x, p1.y)];
    println!("{:?}", p1);

    loop {
        let (x_next, y_next) = p1.look_ahead_coords();
        // This works because it exits on positive coordinates, otherwise will panic. We
        // should use i32 for the coordinates
        if (x_next < 0) | (x_next >= len_x) | (y_next < 0) | (y_next >= len_y) {
            break;
        }
        if map[x_next][y_next] == '#' {
            p1.turn_right();
        } else {
            p1.step();
            if !positions_visited.contains(&(p1.x, p1.y)) {
                positions_visited.push((p1.x, p1.y));
            }
        }

       // println!("{:?}{:?}", p1, positions_visited.len());
    }

    positions_visited.len() as u32
}

fn solve_part_2(lines: Lines) -> u32 {
    let map: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut p = None;
    let len_x = map.len();
    let len_y = map[0].len();
    for i in 0..len_x {
        for j in 0..len_y {
            match map[i][j] {
                '>' => {
                    p = Some(Player {
                        x: i,
                        y: j,
                        direction: Direction::RIGHT,
                    });
                }
                '<' => {
                    p = Some(Player {
                        x: i,
                        y: j,
                        direction: Direction::LEFT,
                    });
                }
                '^' => {
                    p = Some(Player {
                        x: i,
                        y: j,
                        direction: Direction::UP,
                    });
                }
                'v' => {
                    p = Some(Player {
                        x: i,
                        y: j,
                        direction: Direction::DOWN,
                    });
                }
                _ => (),
            }
        }
    }
    // // p = p.unwrap();
    // if let Some(p_1) = p  {
    //     println!("");
    // }
    let mut p1 = p.unwrap();
    let mut positions_visited_plus = vec![(p1.x, p1.y, p1.direction)];
    let mut position_with_obstacle_simulated: Vec<(usize, usize)> = vec![];
    println!("{:?}", p1);

    loop {
        let (x_next, y_next) = p1.look_ahead_coords();
        // This works because it exits on positive coordinates, otherwise will panic. We
        // should use i32 for the coordinates
        if (x_next < 0) | (x_next >= len_x) | (y_next < 0) | (y_next >= len_y) {
            break;
        }
        if map[x_next][y_next] == '#' {
            p1.turn_right();
        } else {
            p1.step();
            if !positions_visited.contains(&(p1.x, p1.y)) {
                positions_visited.push((p1.x, p1.y));
            }
        }

        println!("{:?}{:?}", p1, positions_visited.len());
    }

    positions_visited.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2024_06_example.txt").lines();
        let solution = solve_part_1(lines);
        assert_eq!(41, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2024_06_example.txt").lines();
        let solution = solve_part_2(lines);
        assert_eq!(4, solution);
    }
}
