
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