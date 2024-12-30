use sdl2::{keyboard::KeyboardState, pixels::Color, render::Canvas, video::Window };
use std::f64::consts::PI;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    velocity_x: f64,
    velocity_y: f64,
    rotation_speed: f64,
    acceleration: f64,
    max_velocity: f64,
    deceleration: f64,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            angle: -PI/2.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            rotation_speed: 0.001,
            acceleration: 0.0001,
            max_velocity: 0.05,
            deceleration: 0.000005,
        }
    }

    pub fn update(&mut self, keyboard_state: &KeyboardState) {
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

        self.x += self.velocity_x - self.deceleration;
        self.y += self.velocity_y - self.deceleration;



    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, color: Color) -> Result<(), String> {
        let scale = 20.0;
    
        let tip_x = self.x + scale * (self.angle).cos();
        let tip_y = self.y + scale * (self.angle).sin();
    
        let left_x = self.x + 0.5 * scale * ((self.angle + 2.0 * PI / 3.0).cos());
        let left_y = self.y + 0.5 * scale * ((self.angle + 2.0 * PI / 3.0).sin());
    
        let right_x = self.x + 0.5 * scale * ((self.angle - 2.0 * PI / 3.0).cos());
        let right_y = self.y + 0.5 * scale * ((self.angle - 2.0 * PI / 3.0).sin());
    
        let mut vertices = [
            (tip_x as i32, tip_y as i32),
            (left_x as i32, left_y as i32),
            (right_x as i32, right_y as i32),
        ];
        
        vertices.sort_by_key(|&(_, y)| y);
    
        let (x1, y1) = vertices[0];
        let (x2, y2) = vertices[1];
        let (x3, y3) = vertices[2];
    
        canvas.set_draw_color(color);
    
        for y in y1..=y3 {
            let x_start = if y < y2 {
                interpolate(y, y1, y2, x1, x2)
            } else {
                interpolate(y, y2, y3, x2, x3)
            };
    
            let x_end = interpolate(y, y1, y3, x1, x3);
    
            for x in x_start.min(x_end)..=x_start.max(x_end) {
                canvas.draw_point((x, y)).unwrap();
            }
        }
    
        Ok(())

    }
}

fn interpolate(y: i32, y1: i32, y2: i32, x1: i32, x2: i32) -> i32 {
    if y1 == y2 {
        x1
    } else {
        x1 + (x2 - x1) * (y - y1) / (y2 - y1)
    }
}