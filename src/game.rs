use raylib::prelude::*;
use rand::prelude::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn random_color() -> Color {
    Color {
        r: rand::rng().random::<u8>(),
        g: rand::rng().random::<u8>(),
        b: rand::rng().random::<u8>(),
        a: 0xff,
    }
}

fn random_vec2(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Vector2 {
    Vector2 {
        x: rand::rng().random_range(min_x..=max_x),
        y: rand::rng().random_range(min_y..=max_y),
    }
}

pub struct Game {
    
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_name: String,
    pub mod_flag: bool,
    pub debug_flag: bool,
    pub user_color: Option<Color>,
    pub pos: Option<Vector2>,
    pub vel: Option<Vector2>,
    pub acc: Option<Vector2>,
}

impl User {
    pub fn new_debug() -> Self {
        Self {
            user_name: "".to_owned(),
            mod_flag: false,
            debug_flag: true,
            user_color: Some(random_color()),
            pos: Some(random_vec2(0f32, 0f32, WIDTH as f32, HEIGHT as f32)),
            vel: Some(random_vec2(-1f32, -1f32, 1f32, 1f32)),
            acc: Some(Vector2::zero()),
        }
    }
    /*
    pub fn new(user_id: String, user_name: String, mod_flag: bool, user_color: Color) -> Self {
        
    }
    */
}
