use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: usize
}

#[derive(Debug)]
enum Direction{
    Left,
    Right,
    Up,
    Down
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       let direction = match s.chars().nth(0).unwrap(){
           'R' => Direction::Right,
           'L' => Direction::Left,
           'U' => Direction::Up,
           'D' => Direction::Down,
           _ => unreachable!()
       };
       let length = s[1..].parse::<usize>()?;
       Ok(Instruction{
           direction,
           length
       })
    }
}

fn path_from_instructions(path: &[Instruction]) -> Vec<(isize,isize)>{
    let mut cablePath : Vec<(isize,isize)> = Vec::new();
    let mut current_position = (0,0);
    //cablePath.push(current_position);
    for instruction in path{
        match instruction {
            Instruction {direction: Direction::Up, length: l} => {
                let positions: Vec<(isize,isize)> = (1..=*l).map(|offset| (current_position.0,current_position.1 + offset as isize)).collect();
                cablePath.extend(positions);
                current_position = (current_position.0, current_position.1 + *l as isize);
            },
            Instruction {direction: Direction::Down, length: l} => {
                let positions: Vec<(isize,isize)> = (1..=*l).map(|offset| (current_position.0,current_position.1 - offset as isize)).collect();
                cablePath.extend(positions);
                current_position = (current_position.0, current_position.1 - *l as isize);
            },
            Instruction {direction: Direction::Left, length: l} => {
                let positions: Vec<(isize,isize)> = (1..=*l).map(|offset| (current_position.0 - offset as isize ,current_position.1)).collect();
                cablePath.extend(positions);
                current_position = (current_position.0 - *l as isize, current_position.1 );
            },
            Instruction {direction: Direction::Right, length: l} => {
                let positions: Vec<(isize,isize)> = (1..=*l).map(|offset| (current_position.0 + offset as isize ,current_position.1)).collect();
                cablePath.extend(positions);
                current_position = (current_position.0 + *l as isize, current_position.1 );
            },
        }
    }
    cablePath
}

fn main() -> io::Result<()> {
    // Puzzle 1
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let cables: Vec<Vec<Instruction>> = reader.lines()
    .map(|line|{
        line.unwrap().trim().split(",").map(|s| s.parse::<Instruction>().unwrap()).collect()
    })
    .collect();
    let cable1_path = path_from_instructions(&cables[0]);
    let cable2_path = path_from_instructions(&cables[1]);

    // puzzle 1
    // let closest_intersection = cable1_path.iter()
    // .filter(|pos| cable2_path.contains(pos))
    // .min_by_key(|pos| pos.0.abs() + pos.1.abs())
    // .unwrap();
    // println!("{:?}", closest_intersection.0.abs() + closest_intersection.1.abs());

    // puzzle 2
    let closest_intersection = cable1_path.iter()
    .min_by_key(|pos|{
        if let Some(distance_for_cable_one) = cable1_path.iter().position(|&p| p == **pos){
            if let Some(distance_for_cable_two) = cable2_path.iter().position(|&p| p == **pos){
                return distance_for_cable_one + distance_for_cable_two;
            }
        }
        std::usize::MAX
    })
    .unwrap();
    let cable1_length = cable1_path.iter().position(|p| p == closest_intersection).unwrap();
    let cable2_length = cable2_path.iter().position(|p| p == closest_intersection).unwrap();
    println!("{:?}", cable1_length + cable2_length + 2); // +2 because we start counting from 0
    Ok(())
}