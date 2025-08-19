#[derive(PartialEq, Eq)]
pub enum MoveCommand {
    RotateLeft,
    RotateRight,
    Accelerate,
}

pub trait InputController {
    fn poll(&self) -> Vec<MoveCommand>;
}

pub struct SdlController<'a> {
    pump: &'a sdl2::EventPump,
}

impl<'a> InputController for SdlController<'a> {
    fn poll(&self) -> Vec<MoveCommand> {
        let mut res: Vec<MoveCommand> = Vec::new();
        let keyboard = self.pump.keyboard_state();

        if keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::Left)
            || keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::A)
        {
            res.push(MoveCommand::RotateLeft);
        }
        if keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::Right)
            || keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::D)
        {
            res.push(MoveCommand::RotateRight);
        }
        if keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::Up)
            || keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::W)
        {
            res.push(MoveCommand::Accelerate);
        }

        res
    }
}
