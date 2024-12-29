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

    let screen_area = Rect::new(0, 0, screen_width, screen_height);
    let black = Color::RGB(0,0,0);
    canvas.set_draw_color(black);

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

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
        canvas.fill_rect(screen_area);
        canvas.present();
    }

    println!("Running!");

    Ok(())
}
