use std::error::Error;

use sdl3::{pixels::Color, render::FRect, render::Canvas, video::Window};
pub struct SolidRect {
    pub rect: FRect,
    pub fill_color: Color
}
pub struct Layer {
    pub solid_rects: Vec<SolidRect>,

}
pub struct Scene {
    // the layers of the scene starting with the lowest layer
    pub layers: Vec<Layer>
}


pub fn draw_function(canvas: &mut Canvas<Window>, scene: &Scene) -> Result<(),Box<dyn Error>> {
    // reset the background
    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();

    // the drawing calls

    canvas.set_draw_color(Color::RGB(255,0,0));
    let my_rect: FRect = FRect::new(5.,5.,200.,200.);
    for layer  in  &scene.layers {
        for solid_rect in &layer.solid_rects {
            canvas.set_draw_color(solid_rect.fill_color);
            canvas.fill_rect(solid_rect.rect)?;
            canvas.draw_rect(my_rect)?;
        }

    }

    // present the scene
    canvas.present();
    Ok(())
}