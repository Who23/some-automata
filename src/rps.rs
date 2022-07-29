use rand::prelude::*;
use std::time::Instant;

use crate::HEIGHT;
use crate::WIDTH;

#[derive(Copy, Clone)]
enum State {
    Rock,
    Paper,
    Scissors,
}

pub struct RPS {
    buffer: [State; (WIDTH * HEIGHT) as usize],
    pub last_update: Instant,
}

impl RPS {
    pub fn new() -> Self {
        let mut buffer = [State::Rock; (WIDTH * HEIGHT) as usize];
        let mut rng = thread_rng();
        for pix in &mut buffer {
            *pix = match rng.gen_range(0..=2) {
                0 => State::Rock,
                1 => State::Paper,
                2 => State::Scissors,
                _ => unreachable!(),
            }
        }

        RPS {
            buffer,
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let mut new_buffer = [State::Paper; (WIDTH * HEIGHT) as usize];
        for idx in 0..((WIDTH * HEIGHT) as isize) {
            let neighbors = [
                idx - 1,                   // left
                idx + 1,                   // right
                idx - HEIGHT as isize,     // up
                idx + HEIGHT as isize,     // down
                idx - HEIGHT as isize + 1, // top right
                idx - HEIGHT as isize - 1, // top left
                idx + HEIGHT as isize + 1, // bottom right
                idx + HEIGHT as isize - 1, // bottom left
            ];

            let mut counts = [0, 0, 0];
            for n_idx in neighbors {
                // this is definitely bugged
                if n_idx >= 0 && n_idx < (WIDTH * HEIGHT) as isize {
                    counts[self.buffer[n_idx as usize] as usize] += 1;
                }
            }

            match self.buffer[idx as usize] {
                State::Rock => {
                    if counts[State::Paper as usize] >= 3 {
                        new_buffer[idx as usize] = State::Paper
                    } else {
                        new_buffer[idx as usize] = State::Rock
                    }
                }
                State::Paper => {
                    if counts[State::Scissors as usize] >= 3 {
                        new_buffer[idx as usize] = State::Scissors
                    } else {
                        new_buffer[idx as usize] = State::Paper
                    }
                }
                State::Scissors => {
                    if counts[State::Rock as usize] >= 3 {
                        new_buffer[idx as usize] = State::Rock
                    } else {
                        new_buffer[idx as usize] = State::Scissors
                    }
                }
            }
        }
        self.buffer = new_buffer;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (pixel, state) in frame.chunks_exact_mut(4).zip(self.buffer) {
            let rgba = match state {
                State::Rock => [0x0b, 0x09, 0x0c, 0xff],
                State::Paper => [0x8b, 0x55, 0x76, 0xff],
                State::Scissors => [0xf5, 0xf5, 0xf5, 0xff],
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
