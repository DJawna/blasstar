use std::fmt::Display;

use sdl3::{
    pixels::Color,
    render::{Canvas, FPoint, FRect, Texture, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
};

use crate::game_state::Ui;

pub struct SolidRect {
    pub rect: FRect,
    pub fill_color: Color,
}

pub struct TexturedRect {
    pub render_rect: FRect,
    pub texture_source_rect: FRect,
}

pub struct TextureRectRenderingUnit<'texture> {
    pub texture_rects: Vec<TexturedRect>,
    pub texture: Texture<'texture>,
}

pub struct Layer<'texture> {
    pub solid_rects: Vec<SolidRect>,
    pub texture_units: Vec<TextureRectRenderingUnit<'texture>>,
}
pub struct Scene<'texture> {
    // the layers of the scene starting with the lowest layer
    pub layers: Vec<Layer<'texture>>,
}

impl Display for Ui {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ui_string = match self {
            Ui::Start => "Start",
            Ui::_Game => "Game",
            Ui::_Settings => "Settings",
        };
        f.write_fmt(format_args!("Ui: {}", ui_string))
    }
}

pub struct DebugUiTextures<'a> {
    pub debug_info_labels: Texture<'a>,
}

pub struct StartUiTextures<'a> {
    pub start_new_game_label: Texture<'a>,
    pub exit_game_label: Texture<'a>,
}

pub struct UiTexture<'a> {
    pub debug: DebugUiTextures<'a>,
    pub start: StartUiTextures<'a>,
}

pub struct DrawSystem<'a> {
    canvas: Canvas<Window>,
    ui_textures: UiTexture<'a>,
}

impl<'a> DrawSystem<'a> {
    pub fn init(
        ui_textures: UiTexture<'a>,
        canvas: Canvas<Window>,
    ) -> Result<DrawSystem<'a>, anyhow::Error> {
        Ok(DrawSystem {
            canvas,
            ui_textures,
        })
    }

    pub fn draw_function(
        &mut self,
        scene: &Scene,
        debug_mode: bool,
        ui: &Ui,
    ) -> Result<(), anyhow::Error> {
        // reset the background
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // the drawing calls
        for layer in &scene.layers {
            for solid_rect in &layer.solid_rects {
                self.canvas.set_draw_color(solid_rect.fill_color);
                self.canvas.fill_rect(solid_rect.rect)?;
            }

            for textur_unit in &layer.texture_units {
                for texture_rect in &textur_unit.texture_rects {
                    self.canvas.copy_ex(
                        &textur_unit.texture,
                        texture_rect.texture_source_rect,
                        texture_rect.render_rect,
                        0f64,
                        FPoint { x: 0f32, y: 0f32 },
                        false,
                        false,
                    )?;
                }
            }
        }

        // draw the ui
        if debug_mode {
            self.draw_debug()?;
        }

        self.draw_ui(ui)?;

        // present the scene
        self.canvas.present();
        Ok(())
    }

    fn draw_ui(&mut self, ui: &Ui) -> Result<(), anyhow::Error> {
        match ui {
            Ui::Start => self.draw_start_menu(),
            _ => Err(anyhow::Error::msg(format!(
                "This ui: {} is not yet implemented",
                ui
            ))),
        }
    }

    fn draw_debug(&mut self) -> Result<(), anyhow::Error> {
        let dest_rect = FRect {
            x: 20f32,
            y: 20f32,
            w: 200f32,
            h: 20f32,
        };
        self.canvas.copy(
            &self.ui_textures.debug.debug_info_labels,
            None,
            Some(dest_rect),
        )?;

        Ok(())
    }

    fn draw_start_menu(&mut self) -> Result<(), anyhow::Error> {
        let start_new_game_rect = FRect {
            x: 60f32,
            y: 40f32,
            w: 200f32,
            h: 20f32,
        };

        let exit_rect = FRect {
            x: 60f32,
            y: 80f32,
            w: 200f32,
            h: 20f32,
        };
        self.canvas.copy(
            &self.ui_textures.start.start_new_game_label,
            None,
            Some(start_new_game_rect),
        )?;

        self.canvas.copy(
            &self.ui_textures.start.exit_game_label,
            None,
            Some(exit_rect),
        )?;

        let color = self.canvas.draw_color();

        self.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        self.canvas.draw_line(
            FPoint {
                x: start_new_game_rect.x,
                y: start_new_game_rect.y + start_new_game_rect.h,
            },
            FPoint {
                x: start_new_game_rect.x + start_new_game_rect.w,
                y: start_new_game_rect.y + start_new_game_rect.h,
            },
        )?;

        self.canvas.set_draw_color(color);

        Ok(())
    }
}

// Draws text to texture
pub fn draw_text<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &Font<'_, 'static>,
    text: &str,
) -> Result<Texture<'a>, anyhow::Error> {
    let surface = font
        .render(text)
        .blended(Color::RGBA(0, 255, 0, 255))
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| anyhow::Error::msg(e.to_string()))
}
