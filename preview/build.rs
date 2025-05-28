use std::sync::OnceLock;

fn main() {
    // Read all main.rs and style.css files from /src/components and generate HTML for the highlighted code
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::PathBuf::from(out_dir);
    println!("cargo:rerun-if-changed=src/components");
    for folder in std::fs::read_dir("src/components").unwrap().flatten() {
        // Skip if not a directory
        if !folder.file_type().unwrap().is_dir() {
            continue;
        }
        let folder_name = folder.file_name();
        let folder_name = folder_name.to_string_lossy();
        let out_folder = out_dir.join(&*folder_name);
        std::fs::create_dir_all(&out_folder).unwrap();

        for file in std::fs::read_dir(folder.path()).unwrap().flatten() {
            if file.file_type().unwrap().is_dir() {
                continue; // Skip directories
            }
            let file_name = file.file_name();
            let file_name = file_name.to_string_lossy();
            let html = highlight_file(&file.path());
            let out_file_path = out_folder.join(format!("{}.html", file_name));
            std::fs::write(out_file_path, html).unwrap();
        }
    }
}

fn highlight_file(file_path: &std::path::Path) -> String {
    use std::io::BufRead;
    use syntect::easy::HighlightFile;
    use syntect::highlighting::{Style, ThemeSet};
    use syntect::html::{IncludeBackground, styled_line_to_highlighted_html};
    use syntect::parsing::SyntaxSet;

    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

    let ss = SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines);
    let ts = THEME_SET.get_or_init(ThemeSet::load_defaults);
    let mut all_html = String::new();

    let mut highlighter =
        HighlightFile::new(&file_path, &ss, &ts.themes["base16-ocean.dark"]).unwrap();
    let mut line = String::new();
    while highlighter.reader.read_line(&mut line).unwrap_or_default() > 0 {
        {
            let regions: Vec<(Style, &str)> = highlighter
                .highlight_lines
                .highlight_line(&line, &ss)
                .unwrap();
            let html =
                styled_line_to_highlighted_html(&regions[..], IncludeBackground::No).unwrap();
            all_html += &html;
        }
        line.clear();
    }

    all_html
}
