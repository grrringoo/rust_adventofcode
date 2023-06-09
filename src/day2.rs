use std::env;
use std::fs;

enum HandShapes {
    Rock,
    Paper,
    Scissors,
}

impl HandShapes {
    fn fight(&self, another: &HandShapes) -> RoundResult {
        match (self, another) {
            (HandShapes::Rock, HandShapes::Paper) => RoundResult::Lost,
            (HandShapes::Rock, HandShapes::Scissors) => RoundResult::Win,
            (HandShapes::Scissors, HandShapes::Rock) => RoundResult::Lost,
            (HandShapes::Scissors, HandShapes::Paper) => RoundResult::Win,
            (HandShapes::Paper, HandShapes::Scissors) => RoundResult::Lost,
            (HandShapes::Paper, HandShapes::Rock) => RoundResult::Win,
            _ => RoundResult::Draw,
        }
    }
}

enum RoundResult {
    Lost,
    Draw,
    Win,
}

trait Player {
    fn get_handshape(&self, x: &str) -> Option<HandShapes>;
}

struct MyPlayer {}

impl Player for MyPlayer {
    fn get_handshape(&self, x: &str) -> Option<HandShapes> {
        match x {
            "X" => Some(HandShapes::Rock),
            "Y" => Some(HandShapes::Paper),
            "Z" => Some(HandShapes::Scissors),
            _ => None,
        }
    }
}

struct Oponent {}

impl Player for Oponent {
    fn get_handshape(&self, x: &str) -> Option<HandShapes> {
        match x {
            "A" => Some(HandShapes::Rock),
            "B" => Some(HandShapes::Paper),
            "C" => Some(HandShapes::Scissors),
            _ => None,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = format!("./{}", &args[1]);
    let contents = fs::read_to_string(filename)
        .expect("Error when reading a file. Input data file should be included in this project!!!");
    let mut score = 0;

    for x in contents.split("\n") {
        let round: Vec<&str> = x.split_whitespace().collect();

        // TODO: for some reason in the end of the file I'm receiving empty line. Below is kostil'.
        if round.len() == 0 {
            continue;
        }

        let oponent = Oponent {}
            .get_handshape(&round[0])
            .expect("Invalid handshape code for a My Player. Check your input file.");
        let my_player = MyPlayer {}
            .get_handshape(&round[1])
            .expect("Invalid handshape code for a Oponent. Check your input file.");

        let bonus = match my_player {
            HandShapes::Rock => 1,
            HandShapes::Paper => 2,
            HandShapes::Scissors => 3,
        };

        let round_result = match my_player.fight(&oponent) {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Lost => 0,
        };

        score += round_result + bonus;
    }

    println!("Score: {score}");
}
