use sdl2::log::{log_critical, log_info, Category};

use crate::game_func::game_func;
mod game_func;
mod game_state;

fn main() {
    log_info("Start Blastar...", Category::Application);

    let result = game_func();

    match result {
        Err(err) => log_critical(
            format!("game terminated with: {err}").as_str(),
            Category::Application,
        ),
        Ok(_) => (),
    };

    log_info("End Blastar.", Category::Application);
}
