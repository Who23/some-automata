// https://cargocollective.com/sagejenson/physarum
use rand::prelude::*;
use std::time::Instant;

use crate::HEIGHT;
use crate::WIDTH;

const BOUND: usize = (WIDTH * HEIGHT) as usize;

const NEIGHBOR_TRANSFORM: [isize; 8] = [
    -1,                     // left
    -(HEIGHT as isize) - 1, // top left
    -(HEIGHT as isize),     // up
    -(HEIGHT as isize) + 1, // top right
    1,                      // right
    HEIGHT as isize + 1,    // bottom right
    HEIGHT as isize,        // down
    HEIGHT as isize - 1,    // bottom left
];

struct Agent {
    heading: usize,
    location: usize,
}

pub struct Slime {
    trail: [f64; BOUND],
    agents: Vec<Agent>,
    rng: ThreadRng,
    pub last_update: Instant,
}

impl Slime {
    pub fn new() -> Self {
        let trail = [0.0; BOUND];
        let mut rng = thread_rng();
        let mut agents = Vec::new();
        for _ in 0..100 {
            agents.push(Agent {
                heading: rng.gen_range(0..=7),
                location: rng.gen_range(0..=BOUND),
            })
        }

        Slime {
            trail,
            agents,
            rng,
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        for agent in self.agents.iter_mut() {
            let front = agent.location as isize + NEIGHBOR_TRANSFORM[agent.heading];
            let right = agent.location as isize + NEIGHBOR_TRANSFORM[(agent.heading + 1) % 8];
            let farright =
                agent.location as isize + 2 * NEIGHBOR_TRANSFORM[(agent.heading + 1) % 8];
            let extremeright =
                agent.location as isize + NEIGHBOR_TRANSFORM[(agent.heading + 2) % 8];
            let left = agent.location as isize + NEIGHBOR_TRANSFORM[(agent.heading + 7) % 8];
            let farleft = agent.location as isize + 2 * NEIGHBOR_TRANSFORM[(agent.heading + 7) % 8];
            let extremeleft = agent.location as isize + NEIGHBOR_TRANSFORM[(agent.heading + 6) % 8];

            if front >= 0
                && front < BOUND as isize
                && right >= 0
                && right < BOUND as isize
                // && farfront >= 0
                // && farfront < BOUND as isize
                && left >= 0
                && left < BOUND as isize
                && farleft >= 0
                && farleft < BOUND as isize
                && farright >= 0
                && farright < BOUND as isize
                && extremeleft >= 0
                && extremeleft < BOUND as isize
                && extremeright >= 0
                && extremeright < BOUND as isize
            {
                let front = self.trail[front as usize];
                // 2.0 + self.trail[farfront as usize] / 2.0;
                let right = (self.trail[right as usize]
                    + self.trail[farright as usize]
                    + self.trail[extremeright as usize])
                    / 3.0;
                let left = (self.trail[left as usize]
                    + self.trail[farleft as usize]
                    + self.trail[extremeleft as usize])
                    / 3.0;

                if front.abs_sub(left) < 1.0 && front.abs_sub(right) < 1.0 {
                    if self.rng.gen_bool(0.5) {
                        agent.heading += 1;
                    } else {
                        // wrapping subtraction of -1
                        agent.heading += 7;
                    }
                }

                if right > front && front >= left {
                    agent.heading += 1;
                }

                if left > front && front >= right {
                    agent.heading += 7;
                }

                agent.heading = agent.heading % 8;
            }

            self.trail[agent.location] = 100.0;
            agent.location =
                (agent.location as isize + 3 * NEIGHBOR_TRANSFORM[agent.heading]) as usize;

            if agent.location > BOUND {
                agent.location = BOUND / 2 + 30;
            }
        }

        let mut new_trail = [0.0; BOUND];
        for (idx, p) in self.trail.iter().enumerate() {
            let indicies = NEIGHBOR_TRANSFORM.map(|v| idx as isize + v);
            let mut total = p.clone();
            let mut amt = 1;
            for neighbor in indicies {
                if neighbor >= 0 && neighbor < BOUND as isize {
                    total += self.trail[neighbor as usize];
                    amt += 1;
                }
            }

            new_trail[idx] = (total / (amt as f64)) * 0.98;
        }

        self.trail = new_trail;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (pixel, trail_level) in frame.chunks_exact_mut(4).zip(self.trail) {
            let grayscale = ((trail_level / 10.0) * 256.0) as u8;
            pixel.copy_from_slice(&[grayscale, grayscale, grayscale, 0xff]);
        }
    }
}
