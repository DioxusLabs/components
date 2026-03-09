use super::super::component::*;
use dioxus::prelude::*;

const INLINE_STYLE: &str = r#".recycle-list-container {
  position: relative;
  max-height: 36rem;
  contain: layout paint;
  overflow-y: auto;
}

.recycle-list-demo {
  display: flex;
  flex-direction: column;
  margin: 0 auto;
  gap: 0.75rem;
}

.recycle-list-demo .recycle-list-subtitle {
  margin: 0;
  margin-bottom: 0.75rem;
  color: var(--primary-color-9);
  font-size: 0.9rem;
}

.recycle-list-card {
  padding: 0.75rem 0.9rem;
  border: 1px solid var(--primary-color-6);
  border-radius: 0.625rem;
  background: var(--primary-color-2);
}

.recycle-list-card h3 {
  margin: 0 0 0.3rem;
  color: var(--primary-color-12);
  font-size: 0.95rem;
}

.recycle-list-card p {
  margin: 0;
  color: var(--primary-color-11);
  font-size: 0.875rem;
  line-height: 1.4;
}"#;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "recycle-list-demo",
            p { class: "recycle-list-subtitle", "Scroll this page to verify virtualized rendering with dynamic row heights." }
            style { {INLINE_STYLE} }
            RecycleList {
                count: 2000,
                buffer: 12,
                render_item: move |idx: usize| {
                    let extra_text = "Extra content to vary row height. ".repeat(idx % 6);
                    rsx! {
                        article { class: "recycle-list-card",
                            h3 { "#{idx + 1} - Item {idx + 1}" }
                            p { "Virtualized row preview. Index = {idx}" }
                            p { "{extra_text}" }
                        }
                    }
                },
            }
        }
    }
}
