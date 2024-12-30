use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window};
use std::{f64::consts::PI, time::Instant};

#[derive(Debug)]
struct Laser {
    x: f64,
    y: f64,
    angle: f64,
    speed: f64,
}

impl Laser {
    fn new(x: f64, y: f64, angle: f64, speed: f64) -> Self {
        Laser { x, y, angle, speed }
    }

    fn update(&mut self) {
        self.x += self.speed * self.angle.cos();
        self.y += self.speed * self.angle.sin();
    }

    fn draw(&self, canvas: &mut Canvas<Window>, color: Color) -> Result<(), String> {
        let laser_length = 20.0;

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

fn draw_triangle(
    canvas: &mut Canvas<Window>,
    center_x: f64,
    center_y: f64,
    size: f64,
    angle: f64,
) -> Result<(), String> {
    let tip_x = center_x + size * (angle).cos();
    let tip_y = center_y + size * (angle).sin();

    let left_x = center_x + size * 0.5 * ((angle + 2.0 * PI / 3.0).cos());
    let left_y = center_y + size * 0.5 * ((angle + 2.0 * PI / 3.0).sin());

    let right_x = center_x + size * 0.5 * ((angle - 2.0 * PI / 3.0).cos());
    let right_y = center_y + size * 0.5 * ((angle - 2.0 * PI / 3.0).sin());

    let mut vertices = [
        (tip_x as i32, tip_y as i32),
        (left_x as i32, left_y as i32),
        (right_x as i32, right_y as i32),
    ];
    vertices.sort_by_key(|&(_, y)| y);

    let (x1, y1) = vertices[0];
    let (x2, y2) = vertices[1];
    let (x3, y3) = vertices[2];

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

fn interpolate(y: i32, y1: i32, y2: i32, x1: i32, x2: i32) -> i32 {
    if y1 == y2 {
        x1
    } else {
        x1 + (x2 - x1) * (y - y1) / (y2 - y1)
    }
}

fn main() -> Result<(), String> {   
    let screen_width = 800;
    let screen_height = 600;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Asteroids", screen_width, screen_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    let black = Color::RGB(0,0,0);
    let white = Color::RGB(255, 255, 255);
    canvas.set_draw_color(black);

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    let mut angle: f64 = -PI/2.0;
    let rotation_speed: f64 = 0.001;
    let mut pos_x: f64 = screen_width as f64 / 2.0;
    let mut pos_y: f64 = screen_height as f64 / 2.0;
    let mut velocity_x: f64 = 0.0;
    let mut velocity_y: f64 = 0.0;
    let max_velocity: f64 = 0.05;
    let acceleration: f64 = 0.0001;
    let decceleration: f64 = 0.000005;

    let mut lasers: Vec<Laser> = Vec::new();
    let laser_speed: f64 = 0.075;
    let max_lasers: usize = 10;
    let firing_interval = std::time::Duration::from_millis(350);
    let mut last_fired_time = Instant::now();
    let mut firing = false;

    while running {

        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    // Pause loop and open pause menu
                },
                _ => {}
            }
        }

        let keyboard_state = event_queue.keyboard_state();

        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left) ||
        keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            angle -= rotation_speed;
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right) ||
        keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            angle += rotation_speed;
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Up) ||
        keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            velocity_x += acceleration * angle.cos();
            velocity_y += acceleration * angle.sin();    
        } else {
            velocity_x -= decceleration * velocity_x.signum();
            velocity_y -= decceleration * velocity_y.signum();
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Space) {
            firing = true;
            if last_fired_time.elapsed() >= firing_interval {
                if lasers.len() < max_lasers {
                    lasers.push(Laser::new(pos_x, pos_y, angle, laser_speed));
                    last_fired_time = Instant::now();
                }
            }
        } else {
            firing = false;
        }

        let current_velocity: f64 = (velocity_x.powi(2) + velocity_y.powi(2)).sqrt();
        if current_velocity > max_velocity {
            let scale: f64 = max_velocity / current_velocity;
            velocity_x *= scale;
            velocity_y *= scale;
        }

        pos_x += velocity_x - decceleration;
        pos_y += velocity_y - decceleration;

        if pos_x < 0.0 {
            pos_x = screen_width as f64;
        } else if pos_x > screen_width as f64 {
            pos_x = 0.0;
        }

        if pos_y < 0.0 {
            pos_y = screen_height as f64;
        } else if pos_y > screen_height as f64 {
            pos_y = 0.0;
        }

        for laser in lasers.iter_mut() {
            laser.update();
        }

        lasers.retain(|laser| laser.x >= 0.0 && laser.x <= screen_width as f64 && laser.y >= 0.0 && laser.y <= screen_height as f64);
        canvas.set_draw_color(black);
        canvas.clear();

        canvas.set_draw_color(white);
        draw_triangle(&mut canvas, pos_x, pos_y, 30.0, angle)?;

        for laser in lasers.iter() {
            laser.draw(&mut canvas, white)?;
        }

        canvas.present();
    }

    println!("Running!");

    Ok(())
}
