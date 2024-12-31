use sdl2::{keyboard::KeyboardState, pixels::Color, render::Canvas, video::Window};
use std::time::{Instant, Duration};

use crate::player::Player;
use crate::laser::Laser;
use crate::asteroid::Asteroid;

pub struct GameState {
    pub running: bool,
    pub player: Player,
    pub asteroids: Vec<Asteroid>,
    screen_width: u32,
    screen_height: u32,
    lasers: Vec<Laser>,
    n_asteroids: usize,
    max_lasers: usize,
    last_fired_time: Instant,
    firing_interval: Duration,
}

impl GameState {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        GameState {
            screen_width,
            screen_height,
            running: true,
            asteroids: Vec::new(),
            player: Player::new(screen_width as f64 / 2.0, screen_height as f64 / 2.0),
            lasers: Vec::new(),
            n_asteroids: 7,
            max_lasers: 64,
            last_fired_time: Instant::now(),
            firing_interval: Duration::from_millis(350),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let black = Color::RGB(0,0,0);
        let white = Color::RGB(255, 255, 255);

        canvas.set_draw_color(black);
        canvas.clear();

        self.player.draw(canvas, white)?;
        
        for asteroid in &self.asteroids {
            asteroid.draw(canvas, white)?;
        }

        for laser in &self.lasers {
            laser.draw(canvas, white)?;
        }

        canvas.present();
        Ok(())
    }

    pub fn handle_firing(&mut self, keyboard_state: &KeyboardState) {
        self.lasers.retain(|laser| laser.x >= 0.0 && laser.x <= 800.0 && laser.y >= 0.0 && laser.y <= 600.0);

        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Space) {
            if self.last_fired_time.elapsed() >= self.firing_interval {
                if self.lasers.len() < self.max_lasers {
                    self.lasers.push(Laser::new(self.player.x, self.player.y, self.player.angle));
                    self.last_fired_time = Instant::now();
                }
            }
        }

        for laser in self.lasers.iter_mut() {
            laser.update();
        }
    }

    pub fn add_asteroids(&mut self) {
        while self.asteroids.len() < self.n_asteroids {
            let asteroid = Asteroid::new(self.screen_width, self.screen_height);
            self.asteroids.push(asteroid)
        }
    }

    pub fn handle_asteroid_hits(&mut self) {
        let mut asteroids_to_destroy = Vec::new();
        for laser in &self.lasers {
            for (index, asteroid) in self.asteroids.iter().enumerate() {
                if asteroid.is_hit(laser.x, laser.y) {
                    asteroids_to_destroy.push(index);
                }
            }
        }

        for index in asteroids_to_destroy.into_iter().rev() {
            self.destroy_asteroid(index);
        }

    }

    fn destroy_asteroid(&mut self, index: usize) {
        self.asteroids.remove(index);
    }

}
