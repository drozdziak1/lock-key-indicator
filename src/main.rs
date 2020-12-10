use lock_key::{LockKey, LockKeys, LockKeyState};

use std::error::Error;

pub type ErrBox = Box<dyn Error>;

fn main() -> Result<(), ErrBox> {
    let mut lk = LockKey::new();

    let state = lk.state(LockKeys::NumberLock)?;

    println!("State: {:?}", state);

    Ok(())
}
