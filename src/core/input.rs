#[derive(PartialEq, Eq)]
pub enum Command {
    RotateLeft,
    RotateRight,
    Accelerate,
    Fire,
}

pub trait InputController {
    fn poll(&self) -> Vec<Command>;
}

pub struct SdlController<'a> {
    pump: &'a sdl2::EventPump,
}

impl<'a> SdlController<'a> {
    pub fn new(controller: &'a sdl2::EventPump) -> Self {
        return SdlController { pump: controller };
    }
}

impl<'a> InputController for SdlController<'a> {
    fn poll(&self) -> Vec<Command> {
        let mut res: Vec<Command> = Vec::new();
        let keyboard = self.pump.keyboard_state();

        if keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::Left)
            || keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::A)
        {
            res.push(Command::RotateLeft);
        }
        if keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::Right)
            || keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::D)
        {
            res.push(Command::RotateRight);
        }
        if keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::Up)
            || keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::W)
        {
            res.push(Command::Accelerate);
        }
        if keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::Space) {
            res.push(Command::Fire)
        }

        res
    }
}
