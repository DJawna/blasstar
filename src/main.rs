use sdl3::log::{ log_critical, log_info, Category };

use crate::game_func::game_func;
mod game_func;
mod game_state;
mod draw_system;

fn main() {
    log_info(Category::Application, "Start Blastar ...");
    
    let result = game_func();

    match result {
        Err(err) => log_critical(Category::Application, format!("game terminated with: {err}").as_str()),
        Ok(_) => ()
    };

    log_info(Category::Application, "End Blastar.");
}
