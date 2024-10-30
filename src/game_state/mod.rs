#[derive(PartialEq)]
#[allow(dead_code)]
pub enum DisplayServer {
    Xorg = 0,
    Wayland,
    Windows,
    Unknown
}

pub struct GlobalConfig {
    pub width: u32,
    pub height: u32,
    pub linux_window_server: DisplayServer
}

pub struct GameState {
    pub global_config: GlobalConfig,
    pub should_continue: bool
}

/**
 * initialize the game state
 */
pub fn init_game_state(platform : &str) -> GameState {

    let gc = GlobalConfig{
        width : 800,
        height : 600,
        linux_window_server: match platform {
           "Windows" => DisplayServer::Windows,
           "Linux" => DisplayServer::Wayland,
           _ => DisplayServer::Unknown
        }
    };

    let gs = GameState{
        global_config : gc,
        should_continue: true
    };

    return gs;
}