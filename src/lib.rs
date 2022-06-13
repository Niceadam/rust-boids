use macroquad::prelude::*;

const MAX_SPEED: f32 = 3.0;
const MAX_FORCE: f32 = 0.07;
const SEPARATION_RADIUS: f32 = 30.0;
const ALIGN_RADIUS: f32 = 50.0;
const COHESION_RADIUS: f32 = 50.0;
const BOUNDS: f32 = 180.0;

pub struct Boid {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
}

impl Boid {
    fn new(x: f32, y: f32) -> Self {
        Self {
            pos: vec2(x, y),
            vel: vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)),
            acc: vec2(0.0, 0.0),
        }
    }

    fn seek(&mut self, desire: Vec2) {
        let desired = desire - self.pos;
        self.acc += (desired - self.vel).clamp_length_max(MAX_FORCE * 3.0);
    }

    fn bounds(&mut self) {
        let mut desired = Vec2::ZERO;
        if self.pos.x > screen_width() - BOUNDS {
            desired += vec2(-1.0 * MAX_SPEED, self.vel.y);
        }
        if self.pos.x < BOUNDS {
            desired += vec2(MAX_SPEED, self.vel.y);
        }
        if self.pos.y > screen_height() - BOUNDS {
            desired += vec2(self.vel.x, -1.0 * MAX_SPEED);
        }
        if self.pos.y < BOUNDS {
            desired += vec2(self.vel.x, MAX_SPEED);
        }

        if desired.length_squared() > 0.0 {
            self.acc += (desired - self.vel).clamp_length_max(MAX_FORCE);
        }
    }

    fn loop_bounds(&mut self) {
        if self.pos.x < 0.0 {
            self.pos.x = screen_width() - self.pos.x;
        } else if self.pos.x > screen_width() {
            self.pos.x = self.pos.x % screen_width();
        }
        if self.pos.y < 0.0 {
            self.pos.y = screen_height() - self.pos.y;
        } else if self.pos.y > screen_height() {
            self.pos.y = self.pos.y % screen_height();
        }
    }

    fn draw(&self) {
        let back = self.pos - self.vel.normalize() * 15.0;
        let sides = self.vel.perp().normalize() * 5.0;
        let v1 = back + sides;
        let v2 = back - sides;
        draw_triangle_lines(self.pos, v1, v2, 0.6, BLACK);
        draw_triangle(self.pos, v1, v2, GRAY);
    }

    fn update(&mut self) {
        self.vel = (self.vel + self.acc).clamp_length_max(MAX_SPEED);
        self.pos += self.vel;
        self.acc = Vec2::ZERO;
    }
}

pub struct Flock {
    num_boids: usize,
    boids: Vec<Boid>,
    dist: Vec<Vec<Vec2>>,
}

impl Flock {
    pub fn new(num_boids: usize) -> Self {
        let boids = (0..num_boids)
            .map(|_i| Boid::new(screen_width() / 2.0, screen_height() / 2.0))
            .collect();
        let dist = vec![vec![Vec2::ZERO; num_boids]; num_boids];
        Self {
            num_boids,
            boids,
            dist,
        }
    }

    pub fn draw(&self) {
        for boid in &self.boids {
            boid.draw();
        }
    }

    pub fn add_boid(&mut self, x: f32, y: f32) {
        self.boids.push(Boid::new(x, y));
        self.num_boids += 1;
    }

    fn flock(&mut self) {
        for i in 0..self.num_boids {
            let mut steer_sep = Vec2::ZERO;
            let mut steer_align = Vec2::ZERO;
            let mut steer_cohen = Vec2::ZERO;
            for j in 0..self.num_boids {
                let dist = self.dist[i][j].length();
                if dist > 0.0 {
                    if dist < ALIGN_RADIUS {
                        steer_align += self.boids[j].vel;
                    }
                    if dist < COHESION_RADIUS {
                        steer_cohen += self.boids[j].pos;
                    }
                    if dist < SEPARATION_RADIUS {
                        steer_sep -= self.dist[i][j] / dist;
                    }
                }
            }

            let mut steer = Vec2::ZERO;
            steer += steer_sep.normalize_or_zero() * MAX_SPEED;
            steer += steer_align.normalize_or_zero() * MAX_SPEED;
            steer += steer_cohen.normalize_or_zero() * MAX_SPEED;

            if steer.max_element() > 0.0 {
                let boid = &mut self.boids[i];
                boid.acc += (steer - boid.vel).clamp_length_max(MAX_FORCE);
            }
        }
    }

    pub fn update(&mut self) {
        // Pairwise distances
        self.dist = vec![vec![Vec2::ZERO; self.num_boids]; self.num_boids];
        for i in 0..self.num_boids {
            for j in i..self.num_boids {
                self.dist[i][j] = self.boids[j].pos - self.boids[i].pos;
                self.dist[j][i] = -1.0 * self.dist[i][j];
            }
        }

        // Flocking
        self.flock();

        // Individual & Update
        for i in 0..self.num_boids {
            let boid = &mut self.boids[i];
            //boid.seek(mouse_position().into());
            boid.loop_bounds();
            boid.update();
        }
    }
}
