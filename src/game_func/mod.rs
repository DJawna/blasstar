use std::error::Error;

use crate::{
    asset_manager::AssetManager,
    draw_system::{DebugUiTextures, DrawSystem, Layer, Scene, SolidRect, UiTexture, draw_text},
    game_state::{GameState, LinuxWindowServer, init_game_state},
};

use sdl3::{
    VideoSubsystem,
    event::Event,
    hint::set,
    keyboard::Keycode,
    log::{Category, log_debug},
    pixels::Color,
    render::FRect,
    video::Window,
};

pub fn game_func() -> Result<(), Box<dyn Error>> {
    let mut game_state = init_game_state();

    while game_state.should_continue {
        game_loop(&mut game_state)?;
    }

    Ok(())
}

fn game_loop(global_state: &mut GameState) -> Result<(), Box<dyn Error>> {
    log_debug(Category::Application, "Starting the Game loop...");

    if global_state.global_config.linux_window_server == LinuxWindowServer::Wayland {
        set("SDL_VIDEO_DRIVER", "Wayland");
    }
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = start_window(
        video_subsystem,
        global_state.global_config.width,
        global_state.global_config.height,
    )?;

    let canvas = window.into_canvas();

    let mut event_pump = sdl_context.event_pump()?;

    let scene = Scene {
        layers: vec![Layer {
            solid_rects: vec![
                SolidRect {
                    fill_color: Color::RED,
                    rect: FRect {
                        x: 5f32,
                        y: 5f32,
                        h: 20f32,
                        w: 20f32,
                    },
                },
                SolidRect {
                    fill_color: Color::GREEN,
                    rect: FRect {
                        x: 32f32,
                        y: 32f32,
                        h: 20f32,
                        w: 20f32,
                    },
                },
            ],
            texture_units: vec![],
        }],
    };

    let texture_creator = canvas.texture_creator();
    let asset_manager = AssetManager::init()?;
    let default_font = asset_manager.load_default_font(20f32)?;
    let debug_label = draw_text(&texture_creator, &default_font, "this is debug overlay")?;

    let mut draw_system = DrawSystem::init(
        UiTexture {
            debug: DebugUiTextures {
                debug_info_labels: debug_label,
            },
        },
        canvas,
    )?;

    while global_state.should_continue {
        draw_system.draw_function(&scene, global_state.debug_mode)?;

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
                Event::KeyDown { 
                    keycode: Some(Keycode::F3),
                    ..
                } => {
                    global_state.debug_mode = !global_state.debug_mode;
                }
                _ => {}
            }
        }
    }
    log_debug(Category::Application, "Stopping the Game loop...");
    Ok(())
}

fn start_window(
    video_system: VideoSubsystem,
    width: u32,
    height: u32,
) -> Result<Window, Box<dyn Error>> {
    Ok(video_system
        .window("blastar", width, height)
        .position_centered()
        .build()
        .map_err(|op| Box::new(op))?)
}
