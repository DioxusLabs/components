use std::{fs, path::PathBuf};

const STYLES_IN: &str = "src/styles";
const STYLES_OUT: &str = "css-out";

fn main() {
    println!("cargo:rerun-if-changed=src/styles");

    // Build scss
    let files = fs::read_dir(STYLES_IN).unwrap();

    //Ignore error, usually it's just that the dir already exists.
    _ = fs::create_dir(STYLES_OUT);

    for file in files {
        let file = file.unwrap();
        let path = file.path();
        let file_name = file.file_name();
        let raw_name = file_name.to_str().unwrap().strip_suffix(".scss").unwrap();

        let out_path = PathBuf::from(STYLES_OUT).join(format!("{}.css", raw_name));

        let scss_options = grass::Options::default().style(grass::OutputStyle::Compressed);
        let css = grass::from_path(path, &scss_options).unwrap();
        fs::write(out_path, css).unwrap();
    }
}
