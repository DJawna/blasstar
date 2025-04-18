use sdl3::log::{Category, log_critical, log_info};

use crate::game_func::game_func;
mod draw_system;
mod game_func;
mod game_state;

mod asset_manager;

fn main() {
    log_info(Category::Application, "Start Blastar ...");

    let result = game_func();

    if let Err(err) = result {
        log_critical(
            Category::Application,
            format!("game terminated with: {err}").as_str(),
        )
    };

    log_info(Category::Application, "End Blastar.");
}
