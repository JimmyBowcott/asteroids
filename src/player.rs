use sdl2::{keyboard::KeyboardState, pixels::Color, rect::Point, render::Canvas, video::Window };
use std::{f64::consts::PI, time::{Duration, Instant}};

use crate::utils;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub vertices: Vec<Point>,
    pub score: u32,
    velocity_x: f64,
    velocity_y: f64,
    rotation_speed: f64,
    acceleration: f64,
    max_velocity: f64,
    deceleration: f64,
    lives: u8,
    timer: Instant,
    invulnrable: bool,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            vertices: Vec::new(),
            angle: -PI/2.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            rotation_speed: 0.001,
            acceleration: 0.000025,
            max_velocity: 0.065,
            deceleration: 0.000005,
            score: 0,
            lives: 3,
            timer: Instant::now(),
            invulnrable: false,
        }
    }

    pub fn update(&mut self, keyboard_state: &KeyboardState, screen_width: u32, screen_height: u32) {
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left) ||
        keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            self.angle -= self.rotation_speed;
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right) ||
        keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            self.angle += self.rotation_speed;
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Up) ||
        keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            self.velocity_x += self.acceleration * self.angle.cos();
            self.velocity_y += self.acceleration * self.angle.sin();    
        } else {
            self.velocity_x -= self.deceleration * self.velocity_x.signum();
            self.velocity_y -= self.deceleration * self.velocity_y.signum();
        }

        let current_velocity: f64 = (self.velocity_x.powi(2) + self.velocity_y.powi(2)).sqrt();
        if current_velocity > self.max_velocity {
            let scale: f64 = self.max_velocity / current_velocity;
            self.velocity_x *= scale;
            self.velocity_y *= scale;
        }

        if self.timer.elapsed() >= Duration::from_secs(3) {
            self.invulnrable = false;
        }

        self.move_player();
        self.vertices = utils::get_vertices((self.x, self.y), self.angle, 20.0);
        self.ensure_player_is_on_screen(screen_width, screen_height);

    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, color: Color) -> Result<(), String> {
            if self.invulnrable && self.timer.elapsed().as_millis() / 150 % 2 == 0 {
                return Ok(());
            }
            utils::draw_vertices(canvas, &self.vertices, color)?;
        Ok(())
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }

    pub fn draw_score(&self, canvas: &mut Canvas<Window>, color: Color, font: &sdl2::ttf::Font<'_, '_>) -> Result<(), String> {
        let text = format!("SCORE: {}", self.score);
        let position: (i32, i32) = (25, 25);
        utils::draw_text(canvas, &text, color, font, position)
    }

    pub fn draw_lives(&self, canvas: &mut Canvas<Window>, screen_width: u32, color: Color) -> Result<(), String> {
        // Hack to avoid crashes
        if self.lives == 0 {
            return Ok(());
        }
        let scale = 15.0;
        let offset = 35.0;
        let spacing = 30.0;
        let start_x: f64 = screen_width as f64 - offset - spacing * (self.lives - 1) as f64;
        for i in 0..self.lives {
            let x: f64 = start_x + i as f64 * spacing;
            let vertices = utils::get_vertices((x, offset), -PI/2.0, scale);
            utils::draw_vertices(canvas, &vertices, color)?;
        }
        Ok(())
    }

    pub fn hit(&mut self, screen_width: u32, screen_height: u32) {
        if !self.invulnrable {
            self.x = (screen_width/2) as f64;
            self.y = (screen_height/2) as f64;
            self.lives -= 1;
            self.velocity_x = 0.0;
            self.velocity_y = 0.0;
            self.invulnrable = true;
            self.timer = Instant::now();
        }
    }

    pub fn is_dead(&self) -> bool {
        self.lives == 0
    }

    pub fn reset(&mut self, screen_width: u32, screen_height: u32) {
        self.score = 0;
        self.lives = 3;
        self.x = (screen_width/2) as f64;
        self.y = (screen_height/2) as f64;
        self.velocity_x = 0.0;
        self.velocity_y = 0.0;
        self.invulnrable = false;
    }

    fn move_player(&mut self) {
        self.x += self.velocity_x - self.deceleration;
        self.y += self.velocity_y - self.deceleration;
    }

    fn ensure_player_is_on_screen(&mut self, screen_width: u32, screen_height: u32) {
        if self.x < 0.0 { self.x = screen_width as f64 }
        else if self.x > screen_width as f64 { self.x = 0.0 }
        if self.y < 0.0 { self.y = screen_height as f64 }
        else if self.y > screen_height as f64 { self.y = 0.0 }
    }

}