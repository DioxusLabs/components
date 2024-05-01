use std::{fs, path::PathBuf};

const STYLES_IN: &str = "styles";

fn main() {
    println!("cargo:rerun-if-changed=styles");

    // Build scss
    let file_paths = get_files_recursive(STYLES_IN.into());

    for path in file_paths {
        let file_name = path.file_name().unwrap();
        let Some(raw_name) = file_name.to_str().unwrap().strip_suffix(".scss") else {
            continue;
        };

        let out_path = path.parent().unwrap().join(format!("{}.css", raw_name));

        let scss_options = grass::Options::default().style(grass::OutputStyle::Compressed);
        let css = grass::from_path(path, &scss_options).unwrap();

        fs::write(out_path, css).unwrap();
    }
}

fn get_files_recursive(dir: PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            files.append(&mut get_files_recursive(path));
        } else if path.is_file() {
            files.push(entry.path());
        }
    }

    files
}
