use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use sdl3::{
    event::{Event, EventPollIterator, EventSender},
    keyboard::Keycode, log::log_debug,
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
    pub event_sender: Option<EventSender>,
    pub sdl_init_time: Option<Instant>,
}

pub struct GameInput {
    // the direction
    pub _d_pad_input: DPadDirection,

    // the confirmation gesture has been triggered by the user
    pub confirm_gesture: bool,
}

#[derive(PartialEq, Copy, Clone)]
pub enum DPadDirection {
    None,
    Up,
    Down,
    _Left,
    _Right,
}

pub enum Ui {
    Start(StartMenuState),
    _Game,
    _Settings,
}

pub struct StartMenuState {
    pub selected_option: StartMenuOptions,
}

impl GameInput {
    fn empty() -> GameInput {
        GameInput {
            _d_pad_input: DPadDirection::None,
            confirm_gesture: false,
        }
    }
}

const EVENTSENDERNOTINITIAL: &str = "The event sender must be initialized and assigned to global state manager before the first call to the update method of global state";

impl GameState {
    /// this method will process the input of the `GameState` it will update all the systems and uis
    pub fn update_game_state(
        &mut self,
        event_iterator: EventPollIterator,
    ) -> Result<(), anyhow::Error> {
        let event_sender = self.event_sender.as_ref().expect(EVENTSENDERNOTINITIAL);
        let mut game_input = GameInput::empty();

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
                    Keycode::Escape => event_sender.push_event(Event::Quit {
                        timestamp: self.get_ns_since_sdlinit(),
                    })?,
                    Keycode::Up => {
                        log_debug(sdl3::log::Category::Input, "up button pressed");
                        game_input._d_pad_input = DPadDirection::Up;
                    }
                    Keycode::Down => {

                        log_debug(sdl3::log::Category::Input, "down button pressed");
                        game_input._d_pad_input = DPadDirection::Down;
                    },
                    Keycode::Return => {
                        log_debug(sdl3::log::Category::Input, "return button pressed");
                        game_input.confirm_gesture = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Todo: should be a generic method: There were any inputs yes or no
        if game_input._d_pad_input == DPadDirection::None {
            return Ok(());
        }
        self.current_ui
            .update_ui_state(&game_input, event_sender, self.get_ns_since_sdlinit())?;
        Ok(())
    }

    fn get_ns_since_sdlinit(&self) -> u64 {
        let init_time = self.sdl_init_time.expect("Programming error: sdl_init_time property of global state was not initialized before the first call to `get_ns_since_sdlinit`");
        init_time.elapsed().as_nanos() as u64
    }
}

impl Ui {
    /// this method will process the input
    pub fn update_ui_state(
        &mut self,
        input: &GameInput,
        event_sender: &EventSender,
        timestamp: u64,
    ) -> Result<(), anyhow::Error> {
        match self {
            // Todo: the direction is currently not doing anything since there are only two options in the startmenu
            Ui::Start(start_menu_option) => {
                start_menu_option.update_state(input, event_sender, timestamp)
            }
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

#[derive(Clone, Copy)]
pub enum StartMenuOptions {
    StartNewGame,
    _ExitGame,
}

impl StartMenuState {
    fn update_state(
        &mut self,
        game_input: &GameInput,
        event_sender: &EventSender,
        timestamp: u64,
    ) -> Result<(), anyhow::Error> {
        match (
            self.selected_option,
            game_input._d_pad_input,
            game_input.confirm_gesture,
        ) {
            (StartMenuOptions::_ExitGame, DPadDirection::Down | DPadDirection::Up, _) => {
                self.selected_option = StartMenuOptions::StartNewGame;
                Ok(())
            }
            (StartMenuOptions::StartNewGame, DPadDirection::Down | DPadDirection::Up, _) => {
                self.selected_option = StartMenuOptions::_ExitGame;
                Ok(())
            }
            (StartMenuOptions::_ExitGame, _, true) => {
                Ok(event_sender.push_event(Event::Quit { timestamp })?)
            }
            (StartMenuOptions::StartNewGame, _, true) => {
                Err(anyhow::Error::msg("Start new game not yet implemented"))
            }
            (_, _, _) => Ok(()),
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
        current_ui: Ui::Start(StartMenuState {
            selected_option: StartMenuOptions::StartNewGame,
        }),
        event_sender: None,
        sdl_init_time: None,
    }
}
