use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

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

    let character_width = 20;
    let character_height = 20;
    let character_x = (screen_width / 2 - character_width / 2) as i32;
    let character_y = (screen_height / 2 - character_height / 2) as i32;

    let character_rect = Rect::new(character_x, character_y, character_width, character_height);

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

        canvas.set_draw_color(black);
        canvas.clear();

        canvas.set_draw_color(white);
        canvas.fill_rect(character_rect)?;

        canvas.present();
    }

    println!("Running!");

    Ok(())
}
