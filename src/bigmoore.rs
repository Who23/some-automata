use rand::prelude::*;
use rand::Fill;
use std::time::Instant;

use crate::HEIGHT;
use crate::WIDTH;

pub struct BigMoore {
    buffer: [[bool; WIDTH as usize]; HEIGHT as usize],
    pub last_update: Instant,
}

impl BigMoore {
    pub fn new() -> Self {
        let mut buffer = [[false; WIDTH as usize]; HEIGHT as usize];
        let mut rng = thread_rng();
        for row in &mut buffer {
            row.try_fill(&mut rng).unwrap();
        }

        BigMoore {
            buffer,
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let mut new_buffer = [[false; WIDTH as usize]; HEIGHT as usize];
        for row in 0..(HEIGHT as isize) {
            for col in 0..(WIDTH as isize) {
                let mut neighbor_count = 0;
                for row_offset in [-2isize, -1, 0, 1, 2] {
                    for col_offset in [-2isize, -1, 0, 1, 2] {
                        if row_offset.abs() == 2 && col_offset.abs() == 2 {
                            continue;
                        }

                        let mut neighbor_row = row + row_offset;
                        let mut neighbor_col = col + col_offset;

                        if neighbor_row < 0 {
                            neighbor_row = HEIGHT as isize + neighbor_row;
                        } else if neighbor_row >= HEIGHT as isize {
                            neighbor_row = neighbor_row - HEIGHT as isize;
                        }

                        if neighbor_col < 0 {
                            neighbor_col = WIDTH as isize + neighbor_col;
                        } else if neighbor_col >= WIDTH as isize {
                            neighbor_col = neighbor_col - WIDTH as isize;
                        }

                        neighbor_count +=
                            if self.buffer[neighbor_row as usize][neighbor_col as usize] {
                                1
                            } else {
                                0
                            }
                    }
                }

                new_buffer[row as usize][col as usize] = (neighbor_count > 5
                    && neighbor_count < 13)
                    || (neighbor_count > 17 && neighbor_count < 23);
            }
        }

        self.buffer = new_buffer;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = if self.buffer[index / WIDTH as usize][index % WIDTH as usize] {
                [0x9a, 0x8c, 0x98, 0xff]
            } else {
                [0x4a, 0x4e, 0x69, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
