use std::{fs, path::PathBuf};

const STYLES_IN: &str = "src/styles";
const STYLES_OUT: &str = "css-out";

fn main() {
    println!("cargo:rerun-if-changed=src/styles");

    // Build scss
    let files = fs::read_dir(STYLES_IN).unwrap();

    for file in files {
        let file = file.unwrap();
        let path = file.path();
        let file_name = file.file_name();
        let raw_name = file_name.to_str().unwrap().strip_suffix(".scss").unwrap();

        let out_path = PathBuf::from(STYLES_OUT).join(format!("{}.css", raw_name));

        std::process::Command::new("sass.cmd")
            .arg("--no-source-map")
            .arg("--style=compressed")
            .arg(path)
            .arg(out_path)
            .spawn()
            .expect("You need `sass` installed to build this crate.");
    }
}
