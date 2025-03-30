use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let font_output =Path::new(&out_dir).ancestors().nth(3).unwrap().join("assets").join("font");
    // create the font folder in output folder
    fs::create_dir_all(&font_output).unwrap();
    let font_assets_path= Path::new("./assets/fonts");

    fs::read_dir(font_assets_path)
    .unwrap()
    .for_each(|file | {
        let current_file = file.unwrap();
        fs::copy(current_file.path(), &font_output.join(current_file.file_name())).unwrap();
    });
}