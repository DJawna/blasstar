use sdl2::{
    log::{
        log_critical, log_debug, log_info, Category
    }, pixels::Color, rect::Rect, render::Canvas, video::Window, VideoSubsystem, event::Event, keyboard::Keycode
};


fn main() {
    log_info("Start Blastar...", Category::Application);
    
    let result = game_func();

    match result {
        Err(err) => log_critical(format!("game terminated with: {err}").as_str(), Category::Application),
        Ok(_) => ()
    };

    log_info("End Blastar.", Category::Application);
}

/**
 * initialize the game state
 */
fn init_game_state() -> GameState {
    let gc = GlobalConfig{
        width : 800,
        height : 600,
    };

    let gs = GameState{
        global_config : gc,
        should_continue: true
    };

    return gs;
}

struct GlobalConfig {
    width: u32,
    height: u32,
}

struct GameState {
    global_config: GlobalConfig,
    should_continue: bool
}

fn game_func() -> Result<(),String> {
    let mut game_state = init_game_state();

    while(game_state.should_continue) {
        game_loop(&mut game_state)?;
    }
    
    Ok(())
}


fn game_loop(global_state: &mut GameState) -> Result<(),String> {
    log_debug("Starting the Game loop...", Category::Application);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = start_window(video_subsystem, global_state.global_config.width, global_state.global_config.height)?;
    
    let mut canvas = window
                        .into_canvas()
                        .build()
                        .map_err(|error| format!("could not create canvas: {}", error))?;
    
    let mut event_pump = sdl_context.event_pump()?;
    while(global_state.should_continue) {
        draw_function(&mut canvas)?;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), .. } => {
                    global_state.should_continue = false;
                    break;
                },
                _ => {}
            }
        }
    }

    log_debug("Stopping the Game loop...", Category::Application);
    Ok(())
}

fn draw_function(canvas: &mut Canvas<Window>) -> Result<(),String> {
    canvas.set_draw_color(Color::RGB(0,255,255));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255,0,0));
    let my_rect: Rect = Rect::new(5,5,200,200);
    canvas.draw_rect(my_rect)?;
    canvas.present();
    Ok(())
}

fn start_window(video_system: VideoSubsystem,width: u32, height: u32) -> Result<Window,String> {
    return video_system.window("blastar", width, height)
        .position_centered()
        .build()
        .map_err(|error| format!("could not create the sdl window: {}", error));
}