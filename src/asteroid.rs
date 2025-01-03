use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};
use rand::Rng;
use std::f64::consts::PI;
use crate::utils::{self, triangle_polygon_collision};

pub struct Asteroid {
    x: f64,
    y: f64,
    vertices: Vec<Point>,
    scale: f64,
    velocity_x: f64,
    velocity_y: f64,
}

impl Asteroid {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let mut rng = rand::thread_rng();
        let scale = rng.gen_range(20.0..50.0);
        let (x,y) = utils::generate_spawn_points(screen_width, screen_height, scale);
        let (velocity_x, velocity_y) = utils::generate_velocity(0.01, 0.02);

        Self {
            x,
            y,
            vertices: Vec::new(),
            scale,
            velocity_x,
            velocity_y,
        }
    }

    pub fn update(&mut self, screen_width: u32, screen_height: u32) {
        self.move_asteroid();
        self.ensure_asteroid_is_on_screen(screen_width, screen_height);
        self.recalculate_vertices();
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, color: Color) -> Result<(), String> {
        canvas.set_draw_color(color);
        canvas.draw_lines(&self.vertices[..])?;
        Ok(())
    }

    pub fn is_hit(&self, laser_x: f64, laser_y: f64) -> bool {
        let laser_point = Point::new(laser_x as i32, laser_y as i32);
        utils::is_point_in_polygon(laser_point, &self.vertices)
    }

    pub fn is_colliding(&self, triangle: &Vec<Point>) -> bool {
        triangle_polygon_collision(&triangle, &self.vertices)
    }

    fn move_asteroid(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }

    fn recalculate_vertices(&mut self) {
        self.vertices.clear();
        let num_points = 7;

        for i in 0..num_points {
            let angle = 2.0 * PI * i as f64 / num_points as f64;
            let x_offset = self.scale * angle.cos();
            let y_offset = self.scale * angle.sin();
            self.vertices.push(Point::new((self.x + x_offset) as i32, (self.y + y_offset) as i32));
        }

        self.vertices.push(self.vertices[0]);
    }

    fn ensure_asteroid_is_on_screen(&mut self, screen_width: u32, screen_height: u32) {
        if self.is_off_screen_x(screen_width) {
            if self.velocity_x > 0.0 {
                self.x = -self.scale;
            } else {
                self.x = self.scale + screen_width as f64;
            }
        }
        if self.is_off_screen_y(screen_height) {
            if self.velocity_y > 0.0 {
                self.y = -self.scale;
            } else {
                self.y = self.scale + screen_height as f64;
            }
        }
    }

    fn is_off_screen_x(&self, screen_width: u32) -> bool {
        self.x <= -self.scale || self.x >= self.scale + screen_width as f64
    }

    fn is_off_screen_y(&self, screen_height: u32) -> bool {
        self.y <= -self.scale || self.y >= self.scale + screen_height as f64
    }

}
