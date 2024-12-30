use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};
use rand::Rng;
use std::f64::consts::PI;

pub struct Asteroid {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
}

impl Asteroid {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(20.0..50.0);
        let x = rng.gen_range(0.0..screen_width as f64);
        let y = rng.gen_range(0.0..screen_height as f64);
        let velocity_x = rng.gen_range(-0.02..0.02);
        let velocity_y = rng.gen_range(-0.02..0.02);

        Self {
            x,
            y,
            size,
            velocity_x,
            velocity_y,
        }
    }

    pub fn update(&mut self, screen_width: u32, screen_height: u32) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;

        if self.x < -self.size { self.x = screen_width as f64 + self.size }
        if self.x > screen_width as f64 + self.size { self.x = -self.size }
        if self.y < -self.size { self.y = screen_height as f64 + self.size }
        if self.y > screen_height as f64 + self.size { self.y = -self.size }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, color: Color) -> Result<(), String> {
        canvas.set_draw_color(color);

        let num_points = 20;
        let mut points = Vec::new();

        for i in 0..num_points {
            let angle = 2.0 * PI * i as f64 / num_points as f64;
            let x_offset = self.size * angle.cos();
            let y_offset = self.size * angle.sin();
            points.push(Point::new((self.x + x_offset) as i32, (self.y + y_offset) as i32));
        }

        points.push(points[0]);

        canvas.draw_lines(&points[..])?;
        Ok(())
    }
}
