use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};


pub fn draw_function(canvas: &mut Canvas<Window>) -> Result<(),String> {
    // reset the background
    canvas.set_draw_color(Color::RGB(0,255,255));
    canvas.clear();

    // the drawing calls

    canvas.set_draw_color(Color::RGB(255,0,0));
    let my_rect: Rect = Rect::new(5,5,200,200);
    canvas.draw_rect(my_rect)?;

    // present the scene
    canvas.present();
    Ok(())
}