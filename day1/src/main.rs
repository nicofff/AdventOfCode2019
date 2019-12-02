use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // Puzzle 1
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let fuel: isize = reader.lines()
    .map(|line|{
        line.unwrap().trim().parse::<isize>().unwrap()
    })
    .map(|mass| mass / 3 - 2)
    .sum();

    println!("puzzle one fuel: {}",fuel);

    // Puzzle 2
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let fuel: isize = reader.lines()
    .map(|line|{
        line.unwrap().trim().parse::<isize>().unwrap()
    })
    .map(|mass| mass / 3 - 2)
    .map(|mass_fuel| {
        let mut total_fuel = mass_fuel;
        let mut new_mass = mass_fuel;
        loop {
            let extra_fuel: isize = new_mass / 3 - 2;
            if extra_fuel < 0 {
                break;
            }
            total_fuel += extra_fuel;
            new_mass = extra_fuel;
        }
        total_fuel
    })
    .sum();

    println!("puzzle two fuel: {}",fuel);

    Ok(())
}