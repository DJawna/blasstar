use std::error::Error;

use sdl3::{pixels::Color, render::{FRect, FPoint}, render::{Canvas, Texture}, video::Window};
pub struct SolidRect {
    pub rect: FRect,
    pub fill_color: Color
}

pub struct TexturedRect {
    pub render_rect: FRect,
    pub texture_source_rect: FRect
}


pub struct TextureRectRenderingUnit<'texture> {
    pub texture_rects: Vec<TexturedRect>,
    pub texture: Texture<'texture>
}

pub struct Layer<'texture> {
    pub solid_rects: Vec<SolidRect>,
    pub texture_units: Vec<TextureRectRenderingUnit<'texture>>

}
pub struct Scene<'texture> {
    // the layers of the scene starting with the lowest layer
    pub layers: Vec<Layer<'texture>>
}


pub fn draw_function(canvas: &mut Canvas<Window>, scene: &Scene) -> Result<(),Box<dyn Error>> {
    // reset the background
    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();

    // the drawing calls
    for layer  in  &scene.layers {
        for solid_rect in &layer.solid_rects {
            canvas.set_draw_color(solid_rect.fill_color);
            canvas.fill_rect(solid_rect.rect)?;
        }

        for textur_unit in &layer.texture_units {
            for texture_rect in &textur_unit.texture_rects {
                canvas.copy_ex(&textur_unit.texture, 
                    texture_rect.texture_source_rect, 
                    texture_rect.render_rect, 
                    0f64, 
                    FPoint{
                        x: 0f32,
                        y: 0f32
                    }, 
                    false, 
                    false)?;
            }
        }

    }


    // present the scene
    canvas.present();
    Ok(())
}