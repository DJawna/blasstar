use std::{fmt::Display, time::Duration};

use sdl3::{
    event::{Event, EventPollIterator},
    keyboard::Keycode,
};

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum LinuxWindowServer {
    Xorg = 0,
    Wayland,
}

pub struct GlobalConfig {
    pub width: u32,
    pub height: u32,
    pub linux_window_server: LinuxWindowServer,
    pub desired_frame_time: Duration,
}

pub struct GameState {
    pub global_config: GlobalConfig,
    pub should_continue: bool,
    pub debug_mode: bool,
    pub current_ui: Ui,
    pub game_input: GameInput,
}

pub struct GameInput {
    pub _d_pad_input: DPadDirection,
}

#[derive(PartialEq)]
pub enum DPadDirection {
    None,
    Up,
    Down,
    _Left,
    _Right,
}

pub enum Ui {
    Start(StartMenuOptions),
    _Game,
    _Settings,
}

impl GameState {
    /// this method will process the input of the `GameState` it will update all the systems and uis
    pub fn update_game_state(&mut self, event_iterator: EventPollIterator) {
        for event in event_iterator {
            match event {
                Event::Quit { .. } => {
                    self.should_continue = false;
                    break;
                }
                Event::KeyDown {
                    keycode: Some(key_code),
                    ..
                } => match key_code {
                    Keycode::F3 => {
                        self.debug_mode = !self.debug_mode;
                    }
                    Keycode::Escape => {
                        self.should_continue = false;
                        break;
                    }
                    Keycode::Up => {
                        self.game_input = GameInput {
                            _d_pad_input: DPadDirection::Up,
                        }
                    }
                    Keycode::Down => {
                        self.game_input = GameInput {
                            _d_pad_input: DPadDirection::Down,
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Todo: should be a generic method: There were any inputs yes or no
        if self.game_input._d_pad_input == DPadDirection::None {
            return;
        }
        self.current_ui = self.current_ui.process_input(&self.game_input);
        self.game_input._d_pad_input = DPadDirection::None;
    }
}

impl Ui {
    /// this method will process the input
    pub fn process_input(&self, _input: &GameInput) -> Ui {
        match self {
            // Todo: the direction is currently not doing anything since there are only two options in the startmenu
            Ui::Start(start_menu_option) => Ui::Start(start_menu_option.next_option(true)),
            _ => todo!("porcess input is not yet implemented for the other menus"),
        }
    }
}

impl Display for Ui {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ui_string = match self {
            Ui::Start(_) => "Start",
            Ui::_Game => "Game",
            Ui::_Settings => "Settings",
        };
        f.write_fmt(format_args!("Ui: {}", ui_string))
    }
}

pub enum StartMenuOptions {
    StartNewGame,
    ExitGame,
}

impl StartMenuOptions {
    /// gets the next option dependending on the current option and the direction of the next step
    ///
    /// `direction_up` if true goes up in the option otherwise goes down in the options
    pub fn next_option(&self, _direction_up: bool) -> StartMenuOptions {
        match self {
            StartMenuOptions::StartNewGame => StartMenuOptions::ExitGame,
            StartMenuOptions::ExitGame => StartMenuOptions::StartNewGame,
        }
    }
}

/**
 * initialize the game state
 */
pub fn init_game_state() -> GameState {
    let gc = GlobalConfig {
        width: 800,
        height: 600,
        linux_window_server: LinuxWindowServer::Wayland,
        desired_frame_time: Duration::from_micros(1_000_000 / 120),
    };

    GameState {
        global_config: gc,
        should_continue: true,
        debug_mode: false,
        current_ui: Ui::Start(StartMenuOptions::StartNewGame),
        game_input: GameInput {
            _d_pad_input: DPadDirection::None,
        },
    }
}
