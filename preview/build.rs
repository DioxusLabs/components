fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::PathBuf::from(out_dir);
    println!("cargo:rerun-if-changed=src/components");
    // Process all markdown files in each component folder.
    for folder in std::fs::read_dir("src/components").unwrap().flatten() {
        if !folder.file_type().unwrap().is_dir() {
            continue;
        }
        let folder_path = folder.path();
        walk_markdown_dir(&folder_path, &out_dir).unwrap();
    }
}

fn walk_markdown_dir(dir: &std::path::Path, out_dir: &std::path::Path) -> std::io::Result<()> {
    let folder_name = dir.file_name().unwrap();
    let folder_name = folder_name.to_string_lossy();
    let out_folder = out_dir.join(&*folder_name);
    std::fs::create_dir_all(&out_folder).unwrap();
    for file in std::fs::read_dir(dir).unwrap().flatten() {
        if file.file_type().unwrap().is_dir() {
            walk_markdown_dir(&file.path(), &out_folder)?;
            continue;
        }
        if file.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        if file.path().extension() == Some(std::ffi::OsStr::new("md")) {
            let markdown = process_markdown_to_html(&file.path());
            let out_file_path = out_folder.join(file.file_name()).with_extension("html");
            std::fs::write(out_file_path, markdown).unwrap();
            continue;
        }
    }
    Ok(())
}

fn process_markdown_to_html(markdown_path: &std::path::Path) -> String {
    println!("cargo:rerun-if-changed={}", markdown_path.display());
    use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
    let markdown_input =
        std::fs::read_to_string(markdown_path).expect("Failed to read markdown file");
    let mut options = Options::empty();
    options.insert(Options::ENABLE_GFM);
    let parser = Parser::new_ext(&markdown_input, options);
    let mut events = Vec::new();
    let mut code_block: Option<(CodeBlockKind<'_>, String)> = None;

    for event in parser {
        match (&mut code_block, event) {
            (None, Event::Start(Tag::CodeBlock(kind))) => {
                code_block = Some((kind, String::new()));
            }
            (Some((_, source)), Event::Text(text)) => {
                source.push_str(&text);
            }
            (Some((kind, source)), Event::End(TagEnd::CodeBlock)) => {
                events.push(Event::Html(CowStr::Boxed(
                    render_code_block_html(kind, source).into_boxed_str(),
                )));
                code_block = None;
            }
            (None, event) => events.push(event),
            (Some((_, source)), Event::Code(text)) => {
                source.push_str(&text);
            }
            (Some((_, source)), Event::Html(html) | Event::InlineHtml(html)) => {
                source.push_str(&html);
            }
            (Some(_), _) => {}
        }
    }

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, events.into_iter());
    html_output
}

fn render_code_block_html(kind: &pulldown_cmark::CodeBlockKind<'_>, source: &str) -> String {
    let language = match kind {
        pulldown_cmark::CodeBlockKind::Fenced(info) => {
            let slug = info.split_whitespace().next().unwrap_or_default();
            language_from_slug(slug)
        }
        pulldown_cmark::CodeBlockKind::Indented => None,
    };

    let Some(language) = language else {
        return render_plain_code_block(source);
    };

    let source = source.trim_end_matches('\n');
    let highlighted: dioxus_code::advanced::HighlightedSource =
        dioxus_code::SourceCode::new(language, source).into();

    let mut light_code = String::new();
    push_highlighted_code(&mut light_code, &highlighted, "dxc-github-light");

    let mut dark_code = String::new();
    push_highlighted_code(&mut dark_code, &highlighted, "dxc-github-dark");

    format!(
        r#"<div class="dx-preview-code-theme dx-preview-code-theme-light">{light_code}</div><div class="dx-preview-code-theme dx-preview-code-theme-dark">{dark_code}</div>"#
    )
}

fn push_highlighted_code(
    html: &mut String,
    highlighted: &dioxus_code::advanced::HighlightedSource,
    theme_class: &str,
) {
    html.push_str(r#"<pre class="dxc "#);
    html.push_str(theme_class);
    html.push_str(r#"" data-language=""#);
    html.push_str(highlighted.language().slug());
    html.push_str(r#""><code>"#);

    for segment in highlighted.segments() {
        if let Some(tag) = segment.tag() {
            html.push_str(r#"<span class="a-"#);
            html.push_str(tag);
            html.push_str(r#"">"#);
            push_escaped_html(html, segment.text());
            html.push_str("</span>");
        } else {
            push_escaped_html(html, segment.text());
        }
    }

    html.push_str("</code></pre>");
}

fn render_plain_code_block(source: &str) -> String {
    let mut html = String::from(r#"<pre><code>"#);
    push_escaped_html(&mut html, source.trim_end_matches('\n'));
    html.push_str("</code></pre>");
    html
}

fn language_from_slug(slug: &str) -> Option<dioxus_code::Language> {
    match slug {
        "" => Some(dioxus_code::Language::Rust),
        "rs" => Some(dioxus_code::Language::Rust),
        "rust" => Some(dioxus_code::Language::Rust),
        "css" => Some(dioxus_code::Language::Css),
        slug => dioxus_code::Language::from_slug(slug),
    }
}

fn push_escaped_html(output: &mut String, input: &str) {
    for char in input.chars() {
        match char {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            _ => output.push(char),
        }
    }
}
