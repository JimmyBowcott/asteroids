use sdl2::rect::Point;
use rand::Rng;
use std::f64::consts::PI;
use crate::{core::{colour::RGB, renderer::Renderer}, utils::{self, triangle_polygon_collision}};

pub struct AsteroidConstructor {
    x: Option<f64>,
    y: Option<f64>,
    vertices: Vec<Point>,
    scale: Option<f64>,
    velocity_x: Option<f64>,
    velocity_y: Option<f64>,
    parent: Option<bool>,
    screen_width: u32,
    screen_height: u32,
}

impl AsteroidConstructor {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            x: None,
            y: None,
            vertices: Vec::new(),
            scale: None,
            velocity_x: None,
            velocity_y: None,
            parent: None,
            screen_width,
            screen_height,
        }
    }

    pub fn x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }

    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn velocity_x(mut self, velocity_x: f64) -> Self {
        self.velocity_x = Some(velocity_x);
        self
    }

    pub fn velocity_y(mut self, velocity_y: f64) -> Self {
        self.velocity_y = Some(velocity_y);
        self
    }

    pub fn parent(mut self, parent: bool) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn build(self) -> Asteroid {
        let mut rng = rand::thread_rng();

        let scale = self.scale.unwrap_or_else(|| rng.gen_range(30.0..50.0));
        let (x, y) = self.x.zip(self.y).unwrap_or_else(|| {
            utils::generate_spawn_points(self.screen_width, self.screen_height, scale)
        });
        let (velocity_x, velocity_y) = self.velocity_x.zip(self.velocity_y).unwrap_or_else(|| {
            utils::generate_velocity(0.01, 0.02)
        });
        let parent = self.parent.unwrap_or(false);

        Asteroid {
            x,
            y,
            vertices: self.vertices,
            scale,
            velocity_x,
            velocity_y,
            parent,
        }
    }
}

pub struct Asteroid {
    pub parent: bool,
    x: f64,
    y: f64,
    vertices: Vec<Point>,
    scale: f64,
    velocity_x: f64,
    velocity_y: f64,
}

impl Asteroid {
    pub fn update(&mut self, screen_width: u32, screen_height: u32) {
        self.move_asteroid();
        self.ensure_asteroid_is_on_screen(screen_width, screen_height);
        self.recalculate_vertices();
    }

    pub fn draw(&self, renderer: &mut impl Renderer, colour: RGB) -> Result<(), String> {
        renderer.draw_lines(&self.vertices[..], colour)?;
        Ok(())
    }

    pub fn is_hit(&self, laser_x: f64, laser_y: f64) -> bool {
        let laser_point = Point::new(laser_x as i32, laser_y as i32);
        utils::is_point_in_polygon(laser_point, &self.vertices)
    }

    pub fn is_colliding(&self, triangle: &Vec<Point>) -> bool {
        triangle_polygon_collision(&triangle, &self.vertices)
    }

    pub fn generate_child(&self, screen_width: u32, screen_height: u32) -> Asteroid {
        let (x, y) = self.generate_child_position();
        let (velocity_x, velocity_y) = self.generate_child_velocity();
        let scale = self.generate_child_scale();

        AsteroidConstructor::new(screen_width, screen_height)
        .x(x)
        .y(y)
        .velocity_x(velocity_x)
        .velocity_y(velocity_y)
        .scale(scale)
        .build()
    }

    fn generate_child_position(&self) -> (f64, f64) {
        let mut rng = rand::thread_rng();
        let variance = 20.0;
        let delta_x = rng.gen_range(-variance..variance);
        let delta_y = rng.gen_range(-variance..variance);
        (self.x + delta_x, self.y + delta_y)
    }

    fn generate_child_velocity(&self) -> (f64, f64) {
        let mut rng = rand::thread_rng();
        let x_variance = 0.25*self.velocity_x.abs();
        let y_variance = 0.25*self.velocity_y.abs();
        let delta_x = rng.gen_range(-x_variance..x_variance);
        let delta_y = rng.gen_range(-y_variance..y_variance);
        (self.velocity_x + delta_x, self.velocity_y + delta_y)
    }

    fn generate_child_scale(&self) -> f64 {
        let mut rng = rand::thread_rng();
        0.4*self.scale + rng.gen_range(-0.2*self.scale..0.2*self.scale)
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
