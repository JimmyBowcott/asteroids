mod player;
mod laser;
mod asteroid;
mod game_state;
mod utils;

use sdl2::event::Event;
use game_state::GameState;

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

    let mut event_queue = sdl_context.event_pump().unwrap();
    let mut game_state = GameState::new(screen_width, screen_height);

    while game_state.running {

        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    game_state.running = false;
                }
                _ => {}
            }
        }

        let keyboard_state = event_queue.keyboard_state();

        game_state.add_asteroids();
        for asteroid in game_state.asteroids.iter_mut() {
            asteroid.update(screen_width, screen_height)
        }
        game_state.player.update(&keyboard_state, screen_width, screen_height);
        game_state.handle_firing(&keyboard_state);
        game_state.handle_asteroid_hits();
        game_state.draw(&mut canvas)?;
        canvas.present();
        
    }

    Ok(())
}
