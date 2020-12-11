use lock_keys::*;
use systray::Application;

use std::error::Error;

pub type ErrBox = Box<dyn Error>;

pub static WHITE: &'static str = "/home/drozdziak1/Samogon/WIP/white.png";
pub static GRAY: &'static str = "/home/drozdziak1/Samogon/WIP/gray.png";

fn state2file(s: LockKeyState) -> &'static str {
    match s {
        LockKeyState::Enabled => WHITE,
        LockKeyState::Disabled => GRAY,
    }
}

fn main() -> Result<(), ErrBox> {
    let mut lk = LockKey::new();

    let mut capslock_app = Application::new()?;
    let mut numlock_app = Application::new()?;

    let mut capslock_state = lk.state(LockKeys::CapitalLock)?;
    let mut numlock_state = lk.state(LockKeys::NumberLock)?;

    numlock_app.set_icon_from_file(state2file(numlock_state))?;

    loop {
        let cur_capslock_state = lk.state(LockKeys::CapitalLock)?;
        let cur_numlock_state = lk.state(LockKeys::NumberLock)?;

        if cur_capslock_state != capslock_state {
            capslock_state = cur_capslock_state;
            capslock_app.set_icon_from_file(state2file(capslock_state))?;
        }

        if cur_numlock_state != numlock_state {
            numlock_state = cur_numlock_state;

            numlock_app.set_icon_from_file(state2file(numlock_state))?;
        }
    }

    println!("State: {:?}", numlock_state);

    Ok(())
}
