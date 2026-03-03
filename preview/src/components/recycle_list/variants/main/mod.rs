use super::super::component::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct DemoRow {
    title: String,
    summary: String,
    extra_lines: usize,
}

fn build_rows() -> Vec<DemoRow> {
    (0..2000)
        .map(|i| DemoRow {
            title: format!("Item {}", i + 1),
            summary: format!("Virtualized row preview. Index = {i}"),
            extra_lines: (i % 6) + 1,
        })
        .collect()
}

#[component]
pub fn Demo() -> Element {
    let rows = use_memo(build_rows);
    let rows_ref = rows.read();

    rsx! {
        div { class: "recycle-list-demo",
            p { class: "recycle-list-subtitle", "Scroll this page to verify virtualized rendering with dynamic row heights." }
            {
                RecycleList(RecycleListProps {
                    items: rows_ref.as_slice(),
                    buffer: 12,
                    render_item: move |row, idx| {
                        let extra_text = "Extra content to vary row height. ".repeat(row.extra_lines);
                        rsx! {
                            article { class: "recycle-list-card",
                                h3 { "#{idx + 1} - {row.title}" }
                                p { "{row.summary}" }
                                p { "{extra_text}" }
                            }
                        }
                    },
                    attributes: vec![],
                })
            }
        }
    }
}
