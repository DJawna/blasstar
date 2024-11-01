use sdl2::{
    pixels::Color, 
    rect::Rect, 
    render::Canvas, 
    video::Window,
    ttf::{Font, Sdl2TtfContext, init}
};

struct Layer {
    simple_objects: Vec<SimpleObject>,
    textfields: Vec<TextField>
}

struct SimpleObject {
    rect: Rect,
    fill_color: Color,
    frame_color: Color,
}

struct TextField {
    rect: Rect,
    text: String,
    fill_color: Color,
    outline_color: Color
}

pub struct Scene {
    layers: Vec<Layer>
}

impl Layer{
    fn empty_layer() -> Layer {
        Layer{
            textfields : vec!(),
            simple_objects : vec!()
        }
    }

    fn add_textfield(& mut self, boundaries: Rect, content: &str, fill_color: Color, outline_color: Color, ttf_context: &Sdl2TtfContext) -> usize {
        let new_textfield = TextField{
            fill_color : fill_color,
            outline_color : outline_color, 
            rect : boundaries,
            text : content.to_string()
        };
        self.textfields.push(new_textfield);

        //ttf_context.load_font(path, 12);
        self.textfields.len()-1
    }

    fn draw_layer(&self,canvas: &mut Canvas<Window>) -> Result<(),String> {
        for object_index in 0..self.simple_objects.len() {
            let simple_object = &self.simple_objects[object_index];
            canvas.set_draw_color(simple_object.fill_color);
            canvas.fill_rect(simple_object.rect)?;
            canvas.set_draw_color(simple_object.frame_color);
            canvas.draw_rect(simple_object.rect)?;
        }

        /* 
        for text_field_index in 0..self.textfields.len() {
            let text_field = &self.textfields[text_field_index];

            canvas.fill_te
        }*/

        Ok(())
    }
}


impl Scene {
    pub fn empty_scene(number_of_layers: usize) -> Scene {
        let mut layers : Vec<Layer>= Vec::new();
        for _ in 0..number_of_layers {
            layers.push(Layer::empty_layer());
        }
        Scene {
            layers : layers
        }
    }

    pub fn add_textfield(& mut self, layer_index: usize, boundaries: Rect, content: &str, fill_color: Color, outline_color: Color) -> Result<usize, String> {
        if self.layers.len() < (layer_index + 1) {
            return Err(format!("the layer: {} does not exist", layer_index));
        }
        Ok(self.layers[layer_index].add_textfield(boundaries, content, fill_color, outline_color))
    }

    fn draw_layers(&self, canvas: &mut Canvas<Window>) {
        for layer_index in 0..self.layers.len() {
            self.layers[layer_index].draw_layer(canvas);
        }
    }
}

struct TextManager<'a> {
    sdlfont_cont : Sdl2TtfContext,
    prepared_fonts: Vec<Font<'a, 'a>>

}

impl <'a> TextManager<'a>{
    fn init() -> Result<TextManager<'a>, String> {
        let ctxt = init().map_err(|err| format!("an error happened: {}", err))?;

        Ok(TextManager{
            sdlfont_cont : ctxt,
            prepared_fonts: Vec::new()
        })

    }

    fn prepare_font(& mut self, path: String, point_size: u16) -> Result<usize, String> {
        let current_font  = self.sdlfont_cont.load_font(path, point_size)?;

        self.prepared_fonts.push(current_font);
        Ok(self.prepared_fonts.len() -1)
    }
}


pub fn draw_function(canvas: &mut Canvas<Window>, scene: &Scene) -> Result<(), String> {
    // reset the background
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // the drawing calls

    /*
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    let my_rect: Rect = Rect::new(5, 5, 200, 200);
    canvas.draw_rect(my_rect)?;
     */
    // render the layers:
    scene.draw_layers(canvas);
    // present the scene
    canvas.present();
    Ok(())
}


