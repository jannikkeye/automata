mod creature;

use automaton::{Tick};
use util::*;

use self::creature::Creature;
use std::fmt;

pub struct PredatorAndPrey {
    pub creatures: Vec<Creature>,
    pub width: u32,
    pub height: u32,
}

impl PredatorAndPrey {
    pub fn new(width: u32, height: u32) -> Self {
        let cells = width * height;
        let mut creatures = Vec::new();

        for i in 0..cells {
            let x = i % width;
            let y = i / width;

            creatures.push(Creature::new(x, y));
        }

        PredatorAndPrey {
            creatures,
            width,
            height,
        }
    }
}

impl Tick for PredatorAndPrey {
    fn tick(&mut self) {
        for i in 0..self.creatures.len() {
            let (first, second) = mut_two::<Creature>(
                i,
                get_other_index(i as u32, self.width, self.height) as usize,
                &mut self.creatures,
            );

            first.tick(second);
        }
    }
}

impl fmt::Display for PredatorAndPrey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        string.push_str(format!(
            "Predators: {}\n",
            self
                .creatures
                .iter()
                .filter(|x| x.is_predator())
                .count()
        ).as_ref());
        string.push_str(format!(
            "Prey: {}\n",
            self.creatures.iter().filter(|x| x.is_prey()).count()
        ).as_ref());

        for creature in self.creatures.iter() {
            if creature.x == 0 {
                string.push('\n');
            }

            string.push_str(&creature.to_string());
            string.push(' ');
        }

        write!(f, "{}", string)
    }
}
