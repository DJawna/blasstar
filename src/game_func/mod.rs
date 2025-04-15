use std::{error::Error, thread, time::Instant};

use crate::{
    asset_manager::AssetManager,
    draw_system::{
        DebugUiTextures, DrawSystem, Layer, Scene, SolidRect, StartUiTextures, UiTexture, draw_text,
    },
    game_state::{GameState, LinuxWindowServer, init_game_state},
};

use sdl3::{
    VideoSubsystem,
    hint::set,
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
    let sdl_init_time = Instant::now();
    global_state.sdl_init_time = Some(sdl_init_time);
    let event_subsystem = sdl_context.event()?;
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
    let debug_info_label = draw_text(&texture_creator, &default_font, "this is debug overlay")?;
    let start_new_game_label = draw_text(&texture_creator, &default_font, "start new game")?;
    let exit_game_label = draw_text(&texture_creator, &default_font, "exit game")?;

    let mut draw_system = DrawSystem::init(
        UiTexture {
            debug: DebugUiTextures { debug_info_label },
            start: StartUiTextures {
                start_new_game_label,
                exit_game_label,
            },
        },
        canvas,
    )?;

    global_state.event_sender = Some(event_subsystem.event_sender());

    while global_state.should_continue {
        let start_of_frame = Instant::now();

        global_state.update_game_state(event_pump.poll_iter())?;

        draw_system.draw_function(&scene, global_state.debug_mode, &global_state.current_ui)?;

        let measured_frame_time = start_of_frame.elapsed();
        if measured_frame_time < global_state.global_config.desired_frame_time {
            let sleep_delay =
                measured_frame_time.abs_diff(global_state.global_config.desired_frame_time);
            thread::sleep(sleep_delay);
        }
    }
    log_debug(Category::Application, "Stopping the Game loop...");
    Ok(())
}

fn start_window(
    video_system: VideoSubsystem,
    width: u32,
    height: u32,
) -> Result<Window, anyhow::Error> {
    video_system
        .window("blastar", width, height)
        .position_centered()
        .build()
        .map_err(|op| anyhow::Error::msg(op.to_string()))
}
