use automaton::{Tick};
use graphics::Color;
use rand::prelude::*;
use std::fmt;
use util::get_surrounding_indices;

pub struct GameOfLife {
    cells: Vec<Status>,
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Status {
    Dead = 0,
    Alive = 1,
}

impl GameOfLife {
    pub fn new(width: u32, height: u32) -> Self {
        let max_cells = width * height;
        let mut cells = Vec::new();
        let mut rng = thread_rng();

        for i in 0..max_cells {
            let status = rng.gen_range(0, 100);
            cells.push(match status {
                0 ... 10 => Status::Alive,
                _ => Status::Dead,
            });
        }

        GameOfLife {
            width,
            height,
            cells,
        }
    }
}

impl Tick for GameOfLife {
    fn tick(&mut self) {
        for i in 0..self.cells.len() {
            let mut surrounding_indices =
                get_surrounding_indices(i as u32, self.width, self.height);
            let mut neighbours = Vec::new();
            let is_border_top = i as u32 / self.width == 0;
            let is_border_right = (i as u32 + 1) as f64 / self.width as f64 % 1.0 == 0.0;
            let is_border_bottom = {
                let val = i as f64 / self.width as f64;

                val >= self.height as f64 - 1.0 && val <= self.height as f64
            };
            let is_border_left = (i as f64 / self.width as f64) % 1.0 == 0.0;
            {
                for (i, index) in surrounding_indices.clone().iter().enumerate() {
                    if is_border_top {
                        if i < 3 {
                            neighbours.push(Status::Dead);
                        } else {
                            neighbours.push(self.cells[*index as usize]);
                        }
                        continue;
                    }

                    if is_border_right {
                        if i >= 2 && i <= 4 {
                            neighbours.push(Status::Dead);
                        } else {
                            neighbours.push(self.cells[*index as usize]);
                        }
                        continue;
                    }

                    if is_border_bottom {
                        if i >= 4 && i <= 6 {
                            neighbours.push(Status::Dead);
                        } else {
                            neighbours.push(self.cells[*index as usize]);
                        }
                        continue;
                    }

                    if is_border_left {
                        if i == 0 || i == 6 || i == 7 {
                            neighbours.push(Status::Dead);
                        } else {
                            neighbours.push(self.cells[*index as usize]);
                        }
                        continue;
                    }

                    neighbours.push(self.cells[*index as usize]);
                }
            }

            {
                let alive_count = {
                    let mut count = 0;

                    for status in neighbours.iter() {
                        match status {
                            Status::Alive => count += 1,
                            _ => (),
                        }
                    }

                    count
                };

                match self.cells[i] {
                    Status::Alive => {
                        if alive_count < 2 {
                            self.cells[i] = Status::Dead;
                        }

                        if alive_count > 3 {
                            self.cells[i] = Status::Dead;
                        }
                    }
                    Status::Dead => {
                        if alive_count == 3 {
                            self.cells[i] = Status::Alive;
                        }
                    }
                }
            }
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        let green = Color::new(0, 200, 0);
        let blue = Color::new(0, 0, 200);

        match self {
            Status::Alive => string.push('X'),
            Status::Dead => string.push(' '),
            _ => string.push(' '), // Status::Alive => string.push_str(&green.to_ansi_string("A")),
                                     // Status::Dead => string.push_str(&blue.to_ansi_string("A"))
        }

        write!(f, "{}", string)
    }
}

impl fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();

        for (i, cell) in self.cells.iter().enumerate() {
            if i as u32 % self.width == 0 {
                string.push('\n');
            }

            string.push_str(&cell.to_string());
            string.push(' ');
        }

        write!(f, "{}", string)
    }
}
