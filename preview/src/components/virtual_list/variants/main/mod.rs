use super::super::component::*;
use dioxus::prelude::*;

const INLINE_STYLE: &str = r#".dx-virtual-list-container {
  position: relative;
  max-height: 9.5rem;
  width: 18rem;
  contain: layout paint;
  overflow-y: auto;
}

.dx-virtual-list-demo {
  display: flex;
  flex-direction: column;
  margin: 0 auto;
  gap: 0.75rem;
}

.dx-virtual-list-demo .dx-virtual-list-subtitle {
  margin: 0;
  margin-bottom: 0.5rem;
  color: var(--primary-color-9);
  font-size: 0.85rem;
  text-align: center;
}

.dx-virtual-list-card {
  padding: 0.6rem 0.75rem;
  border: 1px solid var(--primary-color-6);
  border-radius: 0.5rem;
  background: var(--primary-color-2);
}

.dx-virtual-list-card h3 {
  margin: 0 0 0.2rem;
  color: var(--primary-color-12);
  font-size: 0.875rem;
}

.dx-virtual-list-card p {
  margin: 0;
  color: var(--primary-color-11);
  font-size: 0.8rem;
  line-height: 1.35;
}"#;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "dx-virtual-list-demo",
            p { class: "dx-virtual-list-subtitle", "Virtualized 2,000 rows" }
            style { {INLINE_STYLE} }
            VirtualList {
                count: 2000usize,
                buffer: 12usize,
                estimate_size: |idx| {
                    let base_height = 56;
                    let repeats: usize = idx % 6;
                    let lines = repeats.div_ceil(2);
                    let per_line = 18;
                    base_height + lines as u32 * per_line
                },
                render_item: move |idx: usize| {
                    rsx! {
                        article { class: "dx-virtual-list-card",
                            h3 { "Item #{idx + 1}" }
                            p { "Row index = {idx}" }
                        }
                    }
                },
            }
        }
    }
}
