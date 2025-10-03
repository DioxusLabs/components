use std::sync::OnceLock;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::PathBuf::from(out_dir);
    println!("cargo:rerun-if-changed=src/components");
    // Process all markdown files and highlight code files in each component folder
    for folder in std::fs::read_dir("src/components").unwrap().flatten() {
        if !folder.file_type().unwrap().is_dir() {
            continue;
        }
        let folder_path = folder.path();
        walk_highlight_dir(&folder_path, &out_dir).unwrap();
    }

    // Process the main dx-components-theme.css file
    let theme_css_path = std::path::PathBuf::from("assets/dx-components-theme.css");
    for theme in ["base16-ocean.dark", "base16-ocean.light"] {
        let html = highlight_file_to(&theme_css_path, theme);
        let out_file_path = out_dir.join(format!("dx-components-theme.css.{theme}.html"));
        std::fs::write(out_file_path, html).unwrap();
    }
}

fn walk_highlight_dir(dir: &std::path::Path, out_dir: &std::path::Path) -> std::io::Result<()> {
    let folder_name = dir.file_name().unwrap();
    let folder_name = folder_name.to_string_lossy();
    let out_folder = out_dir.join(&*folder_name);
    std::fs::create_dir_all(&out_folder).unwrap();
    for file in std::fs::read_dir(dir).unwrap().flatten() {
        if file.file_type().unwrap().is_dir() {
            walk_highlight_dir(&file.path(), &out_folder)?;
            continue;
        }
        if file.path().extension() == Some(std::ffi::OsStr::new("md")) {
            let markdown = process_markdown_to_html(&file.path());
            let out_file_path = out_folder.join(file.file_name()).with_extension("html");
            std::fs::write(out_file_path, markdown).unwrap();
            continue;
        }
        let file_name = file.file_name();
        let file_name = file_name.to_string_lossy();
        for theme in ["base16-ocean.dark", "base16-ocean.light"] {
            let html = highlight_file_to(&file.path(), theme);
            let out_file_path = out_folder.join(format!("{file_name}.{theme}.html"));
            std::fs::write(out_file_path, html).unwrap();
        }
    }
    Ok(())
}

fn highlight_file_to(file_path: &std::path::Path, theme: &str) -> String {
    println!("cargo:rerun-if-changed={}", file_path.display());
    use std::io::BufRead;
    use syntect::easy::HighlightFile;
    use syntect::highlighting::{Style, ThemeSet};
    use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
    use syntect::parsing::SyntaxSet;
    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();
    let ss = SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines);
    let ts = THEME_SET.get_or_init(ThemeSet::load_defaults);
    let mut all_html = String::new();
    let mut highlighter = HighlightFile::new(file_path, ss, &ts.themes[theme]).unwrap();
    let mut line = String::new();
    while highlighter.reader.read_line(&mut line).unwrap_or_default() > 0 {
        {
            let regions: Vec<(Style, &str)> = highlighter
                .highlight_lines
                .highlight_line(&line, ss)
                .unwrap();
            let html =
                styled_line_to_highlighted_html(&regions[..], IncludeBackground::No).unwrap();
            all_html += &html;
        }
        line.clear();
    }
    all_html
}

fn process_markdown_to_html(markdown_path: &std::path::Path) -> String {
    println!("cargo:rerun-if-changed={}", markdown_path.display());
    use pulldown_cmark::{Options, Parser};
    let markdown_input =
        std::fs::read_to_string(markdown_path).expect("Failed to read markdown file");
    let mut options = Options::empty();
    options.insert(Options::ENABLE_GFM);
    let parser = Parser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}
