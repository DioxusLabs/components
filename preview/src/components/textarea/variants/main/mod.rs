use super::super::component::*;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut description = use_signal(String::new);
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "1.5rem",

            p { id: "textarea-message", "Description here: {description}" }

            div {
                display: "grid",
                grid_template_columns: "repeat(2, 1fr)",
                gap: "1.5rem",
                min_width: "36rem",
                max_width: "60rem",

                div { display: "flex", flex_direction: "column", gap: ".5rem",
                    Label { html_for: "default", "Default" }
                    Textarea {
                        id: "default",
                        variant: TextareaVariant::Default,
                        placeholder: "Enter your description",
                        value: description,
                        oninput: move |e: FormEvent| description.set(e.value()),
                    }
                }

                div { display: "flex", flex_direction: "column", gap: ".5rem",
                    Label { html_for: "fade", "Fade" }
                    Textarea {
                        id: "fade",
                        variant: TextareaVariant::Fade,
                        placeholder: "Enter your description",
                        value: description,
                        oninput: move |e: FormEvent| description.set(e.value()),
                    }
                }

                div { display: "flex", flex_direction: "column", gap: ".5rem",
                    Label { html_for: "outline", "Outline" }
                    Textarea {
                        id: "outline",
                        variant: TextareaVariant::Outline,
                        placeholder: "Enter your description",
                        value: description,
                        oninput: move |e: FormEvent| description.set(e.value()),
                    }
                }

                div { display: "flex", flex_direction: "column", gap: ".5rem",
                    Label { html_for: "ghost", "Ghost" }
                    Textarea {
                        id: "ghost",
                        variant: TextareaVariant::Ghost,
                        placeholder: "Enter your description",
                        value: description,
                        oninput: move |e: FormEvent| description.set(e.value()),
                    }
                }
            }
        }
    }
}
