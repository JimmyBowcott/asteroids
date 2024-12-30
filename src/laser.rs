use sdl2::{pixels::Color, render::Canvas, video::Window};

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

    pub fn draw(&self, canvas: &mut Canvas<Window>, color: Color) -> Result<(), String> {
        let laser_length = 15.0;

        let end_x = self.x + laser_length * self.angle.cos();
        let end_y = self.y + laser_length * self.angle.sin();

        canvas.set_draw_color(color);
        canvas.draw_line(
            (self.x as i32, self.y as i32),
            (end_x as i32, end_y as i32),
        )?;

        Ok(())
    }
}