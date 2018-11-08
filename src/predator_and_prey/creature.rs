use graphics::Color;
use rand::prelude::*;
use std::fmt::{Display, Formatter, Result};

const REPRODUCTION_THRESHOLD: u8 = 5;

#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    Nothing = 0,
    Predator = 1,
    Prey = 2,
}

#[derive(Debug)]
pub struct Creature {
    pub kind: Kind,
    pub health: u8,
    reproduction_threshold: u8,
    pub color: Color,
    pub x: u32,
    pub y: u32,
    sprite: char,
}

impl Creature {
    pub fn new(x: u32, y: u32) -> Self {
        let mut rng = thread_rng();
        let kind = rng.gen_range(0, 3);

        Creature {
            kind: match kind {
                0 => Kind::Nothing,
                1 => Kind::Predator,
                2 => Kind::Prey,
                _ => Kind::Nothing,
            },
            color: match kind {
                0 => Color::new(0, 0, 0),
                1 => Color::new(255, 0, 0),
                2 => Color::new(0, 255, 0),
                _ => Color::new(0, 0, 0),
            },
            x,
            y,
            reproduction_threshold: REPRODUCTION_THRESHOLD,
            health: match kind {
                1 => 5,
                2 => 1,
                _ => 0,
            },
            sprite: match kind {
                0 => ' ',
                1 => '▀',
                2 => '▀',
                _ => ' ',
            },
        }
    }

    pub fn is_prey(&self) -> bool {
        self.kind == Kind::Prey
    }

    pub fn is_predator(&self) -> bool {
        self.kind == Kind::Predator
    }

    pub fn is_nothing(&self) -> bool {
        self.kind == Kind::Nothing
    }

    fn to_nothing(&mut self) {
        self.kind = Kind::Nothing;
        self.sprite = '_';
        self.color = Color::new(0, 0, 0);
    }

    fn to_prey(&mut self) {
        self.kind = Kind::Prey;
        self.sprite = '▀';
        self.color = Color::new(0, 200, 0);
    }

    fn to_predator(&mut self) {
        self.kind = Kind::Predator;
        self.sprite = '▀';
        self.color = Color::new(200, 0, 0);
    }

    fn heal(&mut self) {
        self.health += 1;
    }

    fn bleed(&mut self) {
        self.health -= 1;
    }

    fn try_move(&mut self, other: &mut Creature) {
        if self.is_predator() {
            self.bleed();

            if self.health <= 0 {
                self.to_nothing();

                return;
            }

            if other.is_prey() {
                self.health += other.health;
                if self.health > self.reproduction_threshold {
                    self.health = 5;
                }
                other.to_predator();
                other.health = 5;

                return;
            }
        }

        if self.is_prey() {
            self.heal();

            if self.health >= self.reproduction_threshold && other.is_nothing() {
                self.health = 1;
                other.to_prey();
                other.health = 1;

                return;
            }
        }

        if other.is_nothing() {
            other.kind = self.kind.clone();
            other.color = self.color.clone();
            other.health = self.health;
            other.sprite = self.sprite;

            self.to_nothing();
        }
    }

    pub fn tick(&mut self, other: &mut Creature) {
        if self.is_nothing() {
            return;
        }

        self.try_move(other);
    }
}

impl Display for Creature {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.color.to_ansi_string(&self.sprite.to_string()))
    }
}
