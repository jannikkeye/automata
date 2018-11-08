extern crate rand;

mod automaton;
mod game_of_life;
mod graphics;
mod predator_and_prey;
mod util;

use std::fmt;
use automaton::*;
use game_of_life::GameOfLife;
use predator_and_prey::PredatorAndPrey;
use std::thread::sleep;
use std::time::Duration;

fn clear_terminal() {
    print!("\x1b[2J");
}

fn main() {
    clear_terminal();
    println!("
        Hi, there!\n
        Choose a simulation:\n
        a: Predator and Prey
        b: Conways Game Of Life
    ");
    print!("Pick an option: ");
    let mut option = String::new();


    match std::io::stdin().read_line(&mut option) {
        Ok(_line) => option = option.replace("\n", ""),
        Err(e) => println!("{}", e)
    }

    enum Automaton {
        PredatorAndPrey(PredatorAndPrey),
        GameOfLife(GameOfLife),
    }

    impl Automaton {
        pub fn to_string(&self) -> String {
            match self {
                Automaton::PredatorAndPrey(pap) => pap.to_string(),
                Automaton::GameOfLife(gol) => gol.to_string()
            }
        }
    }

    impl Tick for Automaton {
        fn tick(&mut self) {
            match self {
                Automaton::PredatorAndPrey(pap) => pap.tick(),
                Automaton::GameOfLife(gol) => gol.tick(), 
            }
        }
    }

    let dimensions: [u32; 2] = [50, 50];
    let width = dimensions[0].clone();
    let height = dimensions[1].clone();
    let mut steps: usize = 0;
    let mut simulation = match option.as_ref() {
        "a" => Automaton::PredatorAndPrey(PredatorAndPrey::new(width, height)),
        "b" => Automaton::GameOfLife(GameOfLife::new(width, height)),
        _ => Automaton::PredatorAndPrey(PredatorAndPrey::new(width, height))
    };

    loop {
        clear_terminal();
        simulation.tick();
        println!("Steps: {:?}", steps);
        println!();
        println!("{}", simulation.to_string());
        println!();
        println!();
        steps = steps + 1;
        sleep(Duration::from_millis(1000 / 30));
    }
}
