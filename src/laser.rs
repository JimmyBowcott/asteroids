use crate::core::{colour::RGB, renderer::Renderer};

#[derive(Debug)]
pub struct Laser {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    speed: f64,
}

impl Laser {
    pub fn new(x: f64, y: f64, angle: f64) -> Self {
        Laser { x, y, angle, speed: 0.075 }
    }

    pub fn update(&mut self) {
        self.x += self.speed * self.angle.cos();
        self.y += self.speed * self.angle.sin();
    }

    pub fn draw(&self, renderer: &mut impl Renderer, colour: RGB) -> Result<(), String> {
        let dot_size = 3;

        renderer.draw_rect(
            (self.x - dot_size as f64 / 2.0) as i32,
            (self.y - dot_size as f64 / 2.0) as i32,
            dot_size as u32,
            dot_size as u32,
            colour,
        );

        Ok(())
    }
}
