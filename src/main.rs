#![allow(dead_code)]
use raylib::prelude::*;

pub mod error;
pub use error::Error;

pub mod game;
pub use game::*;

pub mod secrets;
pub mod twitch;
pub use secrets::{ 
    CLIENT_SECRET, 
    CLIENT_ID, 
    ACCESS_TOKEN, 
    USER_ID
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (mut rl, t) = raylib::init()
        .size(640, 480)
        .title("")
        .build();
    rl.set_target_fps(120);
    let mut users = twitch::update_users().await?;
    let mut last_update = std::time::Instant::now();
    while !rl.window_should_close() {
        if last_update + std::time::Duration::from_secs(3) < std::time::Instant::now() {
            last_update = std::time::Instant::now();
            users = twitch::update_users().await?;
            dbg!(&users);
        }
        let mut d = rl.begin_drawing(&t);
        d.draw_rectangle(0, 0, 100, 100, Color { r: 0xff, g: 0x18, b: 0x18, a: 0xff });
    }
    Ok(())
}
