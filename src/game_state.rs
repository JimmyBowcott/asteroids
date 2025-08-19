use sdl2::{pixels::Color, render::Canvas, video::Window};
use std::time::{Instant, Duration};

use crate::core::input::{Command, InputController};
use crate::player::Player;
use crate::laser::Laser;
use crate::asteroid::{Asteroid, AsteroidConstructor};
use crate::utils;

#[derive(PartialEq)]
pub enum State {
    Playing,
    Paused,
    GameOver,
}

pub struct GameState {
    pub running: bool,
    pub state: State,
    pub player: Player,
    pub asteroids: Vec<Asteroid>,
    screen_width: u32,
    screen_height: u32,
    lasers: Vec<Laser>,
    n_parent_asteroids: usize,
    max_parent_asteroids: usize,
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
            state: State::Playing,
            asteroids: Vec::new(),
            player: Player::new(screen_width as f64 / 2.0, screen_height as f64 / 2.0),
            lasers: Vec::new(),
            n_parent_asteroids: 0,
            max_parent_asteroids: 7,
            max_lasers: 64,
            last_fired_time: Instant::now(),
            firing_interval: Duration::from_millis(350),
        }
    }

    pub fn update(&mut self, controller: &impl InputController) {
        self.add_asteroids();
        for asteroid in self.asteroids.iter_mut() {
            asteroid.update(self.screen_width, self.screen_height)
        }
        self.player.update(controller, self.screen_width, self.screen_height);
        self.handle_firing(controller);
        self.handle_asteroid_hits();
        self.handle_player_collision();
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, font: &sdl2::ttf::Font<'_, '_>) -> Result<(), String> {
        let white = Color::RGB(255, 255, 255);

        self.player.draw(canvas, white)?;
        self.player.draw_score(canvas, white, font)?;
        self.player.draw_lives(canvas, self.screen_width, white)?;

        for asteroid in &self.asteroids {
            asteroid.draw(canvas, white)?;
        }

        for laser in &self.lasers {
            laser.draw(canvas, white)?;
        }

        if self.state == State::Paused {
            self.draw_paused_screen(canvas, white, font)?;
        }

        canvas.present();
        Ok(())
    }

    pub fn handle_firing(&mut self, controller: &impl InputController) {
        self.lasers.retain(|laser| laser.x >= 0.0 && laser.x <= 800.0 && laser.y >= 0.0 && laser.y <= 600.0);

        for cmd in controller.poll() {
            match cmd {
                Command::Fire => self.fire_laser(),
                _ => {},
            }
        }

        for laser in self.lasers.iter_mut() {
            laser.update();
        }
    }

    pub fn add_asteroids(&mut self) {
        while self.n_parent_asteroids < self.max_parent_asteroids {
            let asteroid = AsteroidConstructor::new(self.screen_width, self.screen_height).parent(true).build();
            self.asteroids.push(asteroid);
            self.n_parent_asteroids += 1;
        }
    }

    pub fn handle_asteroid_hits(&mut self) {
        let mut asteroids_to_destroy = Vec::new();
        let mut asteroids_to_spawn = Vec::new();
        let mut lasers_to_remove = Vec::new();

        for (laser_index, laser) in self.lasers.iter().enumerate() {
            for (index, asteroid) in self.asteroids.iter().enumerate() {
                if asteroid.is_hit(laser.x, laser.y) {
                    asteroids_to_destroy.push(index);
                    lasers_to_remove.push(laser_index);
                    self.player.increment_score();
                    if asteroid.parent {
                        self.n_parent_asteroids -= 1;
                        asteroids_to_spawn.push(index);
                    }
                }
            }
        }

        for index in asteroids_to_spawn.into_iter() {
            if let Some(asteroid) = self.asteroids.get(index) {
                let child_1 = asteroid.generate_child(self.screen_width, self.screen_height);
                let child_2 = asteroid.generate_child(self.screen_width, self.screen_height);
                self.asteroids.push(child_1);
                self.asteroids.push(child_2);
            }
        }

        for index in asteroids_to_destroy.into_iter().rev() {
            self.destroy_asteroid(index);
        }

        for index in lasers_to_remove.into_iter().rev() {
            self.remove_laser(index);
        }
    }

    pub fn handle_player_collision(&mut self) {
        for asteroid in self.asteroids.iter() {
            if asteroid.is_colliding(&self.player.vertices) {
                self.player.hit(self.screen_width, self.screen_height);
                if self.player.is_dead() {
                    self.state = State::GameOver
                }
            }
        }  
    }

    pub fn toggle_paused(&mut self) {
        if self.state == State::Playing {
            self.state = State::Paused
        } else if self.state == State::Paused {
            self.state = State::Playing
        }
    }

    pub fn draw_paused_screen(&self,  canvas: &mut Canvas<Window>, color: Color, font: &sdl2::ttf::Font<'_, '_>) -> Result<(), String> {
        let text = "PAUSED";
        let x_offset = -50;
        let y_offset = -20;
        let position: (i32, i32) = ((0.5*self.screen_width as f32) as i32 + x_offset, (0.5*self.screen_height as f32) as i32 + y_offset);
        utils::draw_text(canvas, &text, color, font, position)
    }

    pub fn reset(&mut self) {
        self.asteroids.clear();
        self.lasers.clear();
        self.player.reset(self.screen_width, self.screen_height);
    }

    fn destroy_asteroid(&mut self, index: usize) {
        self.asteroids.remove(index);
    }

    fn remove_laser(&mut self, index: usize) {
        self.lasers.remove(index);
    }

    fn fire_laser(&mut self) {
        if self.last_fired_time.elapsed() >= self.firing_interval {
            if self.lasers.len() < self.max_lasers {
                self.lasers.push(Laser::new(self.player.x, self.player.y, self.player.angle));
                self.last_fired_time = Instant::now();
            }
        }
    }

}
