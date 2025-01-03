use sdl2::{keyboard::KeyboardState, pixels::Color, rect::Point, render::Canvas, video::Window };
use std::f64::consts::PI;

use crate::utils;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub lives: u8,
    pub vertices: Vec<Point>,
    velocity_x: f64,
    velocity_y: f64,
    rotation_speed: f64,
    acceleration: f64,
    max_velocity: f64,
    deceleration: f64,
    score: u32,
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

        self.move_player();
        self.get_vertices();
        self.ensure_player_is_on_screen(screen_width, screen_height);

    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, color: Color) -> Result<(), String> {
        let mut sorted_vertices = self.vertices.clone();
        sorted_vertices.sort_by_key(|point| point.y);
        
        let (x1, y1) = (sorted_vertices[0].x, sorted_vertices[0].y);
        let (x2, y2) = (sorted_vertices[1].x, sorted_vertices[1].y);
        let (x3, y3) = (sorted_vertices[2].x, sorted_vertices[2].y);
    
        canvas.set_draw_color(color);
    
        for y in y1..=y3 {
            let x_start = if y < y2 {
                utils::interpolate(y, y1, y2, x1, x2)
            } else {
                utils::interpolate(y, y2, y3, x2, x3)
            };
    
            let x_end = utils::interpolate(y, y1, y3, x1, x3);
    
            for x in x_start.min(x_end)..=x_start.max(x_end) {
                canvas.draw_point((x, y)).unwrap();
            }
        }
    
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

    pub fn is_dead(&self) -> bool {
        self.lives == 0
    }

    fn move_player(&mut self) {
        self.x += self.velocity_x - self.deceleration;
        self.y += self.velocity_y - self.deceleration;
    }

    fn get_vertices(&mut self) {
        self.vertices.clear();
        let scale = 20.0;
    
        let tip_x = self.x + scale * (self.angle).cos();
        let tip_y = self.y + scale * (self.angle).sin();
    
        let left_x = self.x + 0.5 * scale * ((self.angle + 2.0 * PI / 3.0).cos());
        let left_y = self.y + 0.5 * scale * ((self.angle + 2.0 * PI / 3.0).sin());
    
        let right_x = self.x + 0.5 * scale * ((self.angle - 2.0 * PI / 3.0).cos());
        let right_y = self.y + 0.5 * scale * ((self.angle - 2.0 * PI / 3.0).sin());
    
        self.vertices.push(Point::new(tip_x as i32, tip_y as i32));
        self.vertices.push(Point::new(left_x as i32, left_y as i32));
        self.vertices.push(Point::new(right_x as i32, right_y as i32));
    }

    fn ensure_player_is_on_screen(&mut self, screen_width: u32, screen_height: u32) {
        if self.x < 0.0 { self.x = screen_width as f64 }
        else if self.x > screen_width as f64 { self.x = 0.0 }
        if self.y < 0.0 { self.y = screen_height as f64 }
        else if self.y > screen_height as f64 { self.y = 0.0 }
    }

}