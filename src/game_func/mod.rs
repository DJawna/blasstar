use crate::game_state::{init_game_state, DisplayServer, GameState};
use crate::graphics_func::{draw_function, Scene};
use sdl2::{
    event::Event,
    hint::set,
    keyboard::Keycode,
    log::{log_debug, Category},
    pixels::Color,
    rect::Rect,
    video::Window,
    VideoSubsystem,
};

pub fn game_func() -> Result<(), String> {
    let platform = sdl2::get_platform();
    let mut game_state = init_game_state(platform);

    while game_state.should_continue {
        game_loop(&mut game_state)?;
    }

    Ok(())
}

fn game_loop(global_state: &mut GameState) -> Result<(), String> {
    log_debug("Starting the Game loop...", Category::Application);

    if global_state.global_config.linux_window_server == DisplayServer::Wayland {
        set("SDL_VIDEODRIVER", "wayland");
    }
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = start_window(
        video_subsystem,
        global_state.global_config.width,
        global_state.global_config.height,
    )?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|error| format!("could not create canvas: {}", error))?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut scene = Scene::empty_scene(4);

    setup_debug_view(&mut scene, 3)?;

    while global_state.should_continue {

        draw_function(&mut canvas, &scene)?;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    global_state.should_continue = false;
                    break;
                }
                _ => {}
            }
        }
    }

    log_debug("Stopping the Game loop...", Category::Application);
    Ok(())
}

fn setup_debug_view(scene: & mut Scene, debug_layer: usize) -> Result<(), String> {
    let text_height = 5;
    let heading_widgth = 200;
    let font_color = Color::RGBA(0, 255, 0, 255);
    _ = scene.add_textfield(debug_layer, 
                        Rect::new(0,0,heading_widgth, text_height), 
                        "Debug view", 
                        font_color, 
                        font_color)?;
    Ok(())
}

fn start_window(video_system: VideoSubsystem, width: u32, height: u32) -> Result<Window, String> {
    return video_system
        .window("blastar", width, height)
        .position_centered()
        .build()
        .map_err(|error| format!("could not create the sdl window: {}", error));
}
