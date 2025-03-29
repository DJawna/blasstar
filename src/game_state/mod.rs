use std::{error::Error, fmt::Display};

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum LinuxWindowServer {
    Xorg = 0,
    Wayland
}

pub struct GlobalConfig {
    pub width: u32,
    pub height: u32,
    pub linux_window_server: LinuxWindowServer
}

pub struct GameState {
    pub global_config: GlobalConfig,
    pub should_continue: bool
}

#[derive(Debug)]
pub struct BlastarError {
    pub message: String
}

impl Display for BlastarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Blastar error occured! Message: {}", self.message)
    }
}

impl Error for BlastarError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/**
 * initialize the game state
 */
pub fn init_game_state() -> GameState {
    let gc = GlobalConfig{
        width : 800,
        height : 600,
        linux_window_server: LinuxWindowServer::Wayland
    };

    let gs = GameState{
        global_config : gc,
        should_continue: true
    };

    return gs;
}