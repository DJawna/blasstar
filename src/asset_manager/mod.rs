use sdl3::ttf::{Font, Sdl3TtfContext};

pub struct AssetManager {
    ttf_context: Sdl3TtfContext,
}

impl AssetManager {
    pub fn init() -> Result<AssetManager, anyhow::Error> {
        let ttf_context = sdl3::ttf::init().map_err(|e| anyhow::Error::msg(e.to_string()))?;

        Ok(AssetManager {
            ttf_context: ttf_context,
        })
    }

    fn load_font<'a>(
        &'a self,
        path: &str,
        point_size: f32,
    ) -> Result<Font<'a, 'static>, anyhow::Error> {
        Ok(self.ttf_context.load_font(path, point_size)?)
    }

    pub fn load_default_font<'a>(
        &'a self,
        point_size: f32,
    ) -> Result<Font<'a, 'static>, anyhow::Error> {
        self.load_font("./assets/fonts/orbitron-latin-500-normal.ttf", point_size)
    }
}
