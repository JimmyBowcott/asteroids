mod player;
mod laser;
mod asteroid;
mod game_state;
mod utils;

use sdl2::{event::Event, keyboard::Keycode};
use game_state::{GameState, State};

fn main() -> Result<(), String> {   
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path = "./src/assets/font/HomeVideoBold-R90Dv.ttf";
    let font: sdl2::ttf::Font<'_, '_> = ttf_context.load_font(font_path, 32)?;

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

    let mut event_queue = sdl_context.event_pump().unwrap();
    let mut game_state = GameState::new(screen_width, screen_height);


    while game_state.running {

        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    game_state.running = false;
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    game_state.toggle_paused()
                }
                _ => {}
            }
        }

        let keyboard_state = event_queue.keyboard_state();

        canvas.clear();

        match game_state.state {
            State::Playing => {
                game_state.update(&keyboard_state);
                game_state.draw(&mut canvas, &font)?;
            }
            State::Paused => {
                game_state.draw(&mut canvas, &font)?;
            }
            State::GameOver => {
                utils::draw_game_over_text(&mut canvas, &font)?;
            }
        }

        canvas.present();
        
    }

    Ok(())
}
