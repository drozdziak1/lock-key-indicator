use failure::format_err;
use lock_keys::*;
use systray::Application;
use tempfile::{Builder, TempDir};

use std::error::Error;
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub type ErrBox = Box<dyn Error>;

pub static N_ON_C_ON_FNAME: &'static str = "n_on_c_on.png";
pub static N_ON_C_OFF_FNAME: &'static str = "n_on_c_off.png";
pub static N_OFF_C_ON_FNAME: &'static str = "n_off_c_on.png";
pub static N_OFF_C_OFF_FNAME: &'static str = "n_off_c_off.png";

pub static FNAMES: [&'static str; 4] = [
    N_ON_C_ON_FNAME,
    N_ON_C_OFF_FNAME,
    N_OFF_C_ON_FNAME,
    N_OFF_C_OFF_FNAME,
];

pub static N_ON_C_ON_BYTES: &'static [u8] = include_bytes!("../n_on_c_on.png");
pub static N_ON_C_OFF_BYTES: &'static [u8] = include_bytes!("../n_on_c_off.png");
pub static N_OFF_C_ON_BYTES: &'static [u8] = include_bytes!("../n_off_c_on.png");
pub static N_OFF_C_OFF_BYTES: &'static [u8] = include_bytes!("../n_off_c_off.png");

pub static BYTES: [&'static [u8]; 4] = [
    N_ON_C_ON_BYTES,
    N_ON_C_OFF_BYTES,
    N_OFF_C_ON_BYTES,
    N_OFF_C_OFF_BYTES,
];

fn state2filename(numlock_state: LockKeyState, capslock_state: LockKeyState) -> &'static str {
    use LockKeyState::*;
    match (numlock_state, capslock_state) {
        (Enabled, Enabled) => N_ON_C_ON_FNAME,
        (Enabled, Disabled) => N_ON_C_OFF_FNAME,
        (Disabled, Enabled) => N_OFF_C_ON_FNAME,
        (Disabled, Disabled) => N_OFF_C_OFF_FNAME,
    }
}

fn prepare_asset_files(dir: &TempDir) -> Result<(), ErrBox> {
    for (idx, bytes) in BYTES.iter().enumerate() {
        let mut pb = PathBuf::from(dir.path());
        pb.push(FNAMES[idx]);
        let mut f = File::create(pb)?;
        f.write_all(bytes)?;
    }
    Ok(())
}

fn main() -> Result<(), ErrBox> {
    let lk = LockKey::new();

    let base_dir = Builder::new().prefix("lock-key-indicator").tempdir()?;

    prepare_asset_files(&base_dir)?;

    let mut app = Application::new()?;

    let mut numlock_state = lk.state(LockKeys::NumberLock)?;
    let mut capslock_state = lk.state(LockKeys::CapitalLock)?;

    let mut pb = PathBuf::from(base_dir.path());
    pb.push(state2filename(numlock_state, capslock_state));
    app.set_icon_from_file(
        pb.to_str()
            .ok_or_else(|| format_err!("Could not convert path to string"))?,
    )?;

    // Message polling for systray is a bit wonky and we need to know when we're done
    let done = Arc::new(Mutex::new(false));

    let done_for_menu = done.clone();
    app.add_menu_item("Quit", move |_a| -> Result<(), std::io::Error> {
        println!("Bye!");
        let mut done = done_for_menu.lock().unwrap();
        *done = true;
        Ok(())
    })?;

    while !*done.lock().unwrap() {
        match app.try_wait(Default::default()) {
            Err(systray::Error::TimeoutError) | Ok(_) => {}
            other => {
                other?;
            }
        }
        let cur_numlock_state = lk.state(LockKeys::NumberLock)?;
        let cur_capslock_state = lk.state(LockKeys::CapitalLock)?;

        if cur_capslock_state != capslock_state || cur_numlock_state != numlock_state {
            numlock_state = cur_numlock_state;
            capslock_state = cur_capslock_state;
            let mut pb = PathBuf::from(base_dir.path());
            pb.push(state2filename(numlock_state, capslock_state));
            app.set_icon_from_file(
                pb.to_str()
                    .ok_or_else(|| format_err!("Could not convert path to string"))?,
            )?;
        }
    }
    Ok(())
}
