use crate::utils;

use super::colour::RGB;
use sdl2::{
    rect::Rect,
    render::Canvas,
    video::Window,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<Point> for sdl2::rect::Point {
    fn from(p: Point) -> Self {
        sdl2::rect::Point::new(p.x, p.y)
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point{x, y}
    }
}

pub trait Renderer {
    fn set_colour(&mut self, colour: RGB);
    fn clear(&mut self);
    fn draw_rect(&mut self, x: i32, y: i32, w: u32, h: u32, colour: RGB);
    // fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32);
    fn draw_lines(&mut self, vertices: &[Point], colour: RGB) -> Result<(), String>;
    fn draw_text(&mut self, text: &str, colour: RGB, position: (i32, i32)) -> Result<(), String>;
    fn draw_vertices(&mut self, vertices: &Vec<Point>, colour: RGB) -> Result<(), String>;
    fn draw_game_over_screen(&mut self, score: u32) -> Result<(), String>;
    fn present(&mut self);
}

pub struct SdlRenderer<'a> {
    canvas: &'a mut Canvas<Window>,
    font: &'a sdl2::ttf::Font<'a, 'a>,
    screen_width: u32,
    screen_height: u32,
}

impl<'a> SdlRenderer<'a> {
    pub fn new(
        canvas: &'a mut Canvas<Window>,
        font: &'a sdl2::ttf::Font<'a, 'a>,
        screen_width: u32,
        screen_height: u32,
    ) -> Self {
        SdlRenderer {
            canvas,
            font,
            screen_width,
            screen_height,
        }
    }
}

impl<'a> Renderer for SdlRenderer<'a> {
    fn set_colour(&mut self, colour: RGB) {
        self.canvas.set_draw_color(colour);
    }

    fn clear(&mut self) {
        self.canvas.clear();
    }

    fn draw_rect(&mut self, x: i32, y: i32, w: u32, h: u32, colour: RGB) {
        self.canvas.set_draw_color(colour);
        let _ = self.canvas.fill_rect(Rect::new(x, y, w, h));
    }

    // fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
    //     let _ = self
    //         .canvas
    //         .draw_line(Point::new(x1, y1), Point::new(x2, y2));
    // }

    fn draw_lines(&mut self, vertices: &[Point], colour: RGB) -> Result<(), String> {
        let sdl_points: Vec<sdl2::rect::Point> = vertices.iter().copied().map(Into::into).collect();
        self.canvas.set_draw_color(colour);
        self.canvas.draw_lines(&sdl_points[..])?;
        Ok(())
    }

    fn draw_text(&mut self, text: &str, colour: RGB, position: (i32, i32)) -> Result<(), String> {
        let surface = self
            .font
            .render(text)
            .solid(colour)
            .map_err(|e| e.to_string())?;

        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let text_width = surface.width();
        let text_height = surface.height();
        let text_rect = Rect::new(position.0, position.1, text_width, text_height);

        self.canvas.copy(&texture, None, Some(text_rect))?;
        Ok(())
    }

    fn draw_game_over_screen(&mut self, score: u32) -> Result<(), String> {
        let colour = RGB::WHITE;

        let mut text = "GAME OVER";
        let mut position = (
            (self.screen_width / 2 - 110) as i32,
            (self.screen_height / 2 - 100) as i32,
        );
        self.draw_text(text, colour, position)?;

        let score_text = format!("SCORE: {}", score);
        position = (
            (self.screen_width / 2 - 100) as i32,
            (self.screen_height / 2 - 50) as i32,
        );
        self.draw_text(&score_text, colour, position)?;

        text = "Press Enter to play again";
        position = (
            (self.screen_width / 2 - 250) as i32,
            (self.screen_height / 2) as i32,
        );
        self.draw_text(text, colour, position)?;

        Ok(())
    }

    fn draw_vertices(&mut self, vertices: &Vec<Point>, colour: RGB) -> Result<(), String> {
        let mut sorted_vertices = vertices.clone();
        sorted_vertices.sort_by_key(|point| point.y);

        let (x1, y1) = (sorted_vertices[0].x, sorted_vertices[0].y);
        let (x2, y2) = (sorted_vertices[1].x, sorted_vertices[1].y);
        let (x3, y3) = (sorted_vertices[2].x, sorted_vertices[2].y);

        self.canvas.set_draw_color(colour);

        for y in y1..=y3 {
            let x_start = if y < y2 {
                utils::interpolate(y, y1, y2, x1, x2)
            } else {
                utils::interpolate(y, y2, y3, x2, x3)
            };

            let x_end = utils::interpolate(y, y1, y3, x1, x3);

            for x in x_start.min(x_end)..=x_start.max(x_end) {
                self.canvas.draw_point((x, y))?;
            }
        }
        Ok(())
    }

    fn present(&mut self) {
        self.canvas.present();
    }
}
