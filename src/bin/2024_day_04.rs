use std::str::Lines;

fn main() {
    let lines = include_str!("../../inputs/2024_04.txt").lines();
    let solution_1: u32 = solve_part_1(lines.clone());
    println!("Part 1: {solution_1}");
    let solution_2: u32 = solve_part_2(lines);
    println!("Part 2: {solution_2}");
}

// The approach is to go from left to right, comparing against the two possibilities xmas and samx,
// doing only one pass, to avoid repeating the count
fn solve_part_1(lines: Lines) -> u32 {
    let m: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    //dbg!(&m);
    let x = m.len();
    let y = m[0].len();
    // offsets
    let right = [[0, 0], [0, 1], [0, 2], [0, 3]];
    let down = [[0, 0], [1, 0], [2, 0], [3, 0]];
    let diag_right_down = [[0, 0], [1, 1], [2, 2], [3, 3]];
    let diag_right_up = [[0, 0], [-1, 1], [-2, 2], [-3, 3]];
    let xmas = vec!['X', 'M', 'A', 'S'];
    let xmas_rev = vec!['S', 'A', 'M', 'X'];
    let mut count = 0;
    for i in 0..x {
        for j in 0..y {
            for offsets in [right, down, diag_right_down, diag_right_up] {
                let mut seen = Vec::<char>::new();
                // right
                for offset in offsets {
                    if i as i32 + offset[0] < 0 {
                        // we cannot perform the diag_right_up
                        break;
                    }
                    // converting here to avoid issue with usize and i32 from the neg diag offset
                    let c_x = (i as i32 + offset[0]) as usize;
                    let c_y = j + offset[1] as usize;
                    if (c_x > x - 1) | (c_y > y - 1) {
                        break;
                    }
                    seen.push(m[c_x][c_y]);
                }
                if seen.eq(&xmas) | seen.eq(&xmas_rev) {
                    count += 1;
                }

                //println!("{i}, {j}: {}", m[i][j]);
            }
        }
    }

    count
}

fn solve_part_2(lines: Lines) -> u32 {
    let m: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    //dbg!(&m);
    let mut count = 0;
    let x = m.len();
    let y = m[0].len();
    for i in 1..x-1 {
        for j in 1..y-1 {
            if m[i][j]!='A'{
                continue;
            }
            let is_mas_diag_up: bool = (m[i+1][j+1]=='M') & (m[i-1][j-1]=='S');
            let is_sam_diag_up: bool = (m[i+1][j+1]=='S') & (m[i-1][j-1]=='M');
            let is_mas_diag_down: bool = (m[i+1][j-1]=='M') & (m[i-1][j+1]=='S');
            let is_sam_diag_down: bool = (m[i+1][j-1]=='S') & (m[i-1][j+1]=='M');

            if  (is_mas_diag_up|is_sam_diag_up) & (is_mas_diag_down|is_sam_diag_down){

                count+=1;
            }

        }

    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn question_a() {
        let lines = include_str!("../../inputs/2024_04_example.txt").lines();
        let solution = solve_part_1(lines);
        assert_eq!(18, solution);
    }
    #[test]
    fn question_b() {
        let lines = include_str!("../../inputs/2024_04_example.txt").lines();
        let solution = solve_part_2(lines);
        assert_eq!(0, solution);
    }
}
