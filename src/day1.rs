// Day 1: https://adventofcode.com/2022/day/1

use std::env;
use std::fs;

struct Elf {
    food: Vec<u32>,
}

impl Elf {
    fn get_total_calories(&self) -> u32 {
        let mut total = 0;

        for x in self.food.iter() {
            total += x;
        }

        total
    }
}

fn max_total_calories(elfs: &mut Vec<Elf>) -> u32 {
    elfs.sort_by(|a, b| b.get_total_calories().cmp(&a.get_total_calories()));

    return elfs[0].get_total_calories();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = format!("./{}", &args[1]);
    let contents = fs::read_to_string(format!("./{filename}"))
        .expect("Error when reading a file. Input data file should be included in this project!!!");

    let mut elfs = Vec::new();

    for x in contents.split("\n\n") {
        let mut elf = Elf { food: Vec::new() };

        for y in x.split("\n") {
            match y.parse() {
                Ok(n) => elf.food.push(n),
                Err(_n) => (),
            }
        }

        elfs.push(elf)
    }

    println!("Max total calories: {}", max_total_calories(&mut elfs));
}
