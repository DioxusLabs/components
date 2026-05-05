use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let frameworks: &[(&str, &str)] = &[
        ("next", "Next.js"),
        ("svelte", "SvelteKit"),
        ("nuxt", "Nuxt.js"),
        ("remix", "Remix"),
        ("astro", "Astro"),
        ("solid", "SolidStart"),
        ("dioxus", "Dioxus"),
    ];

    rsx! {
        ComboboxMulti::<String> {
            default_values: vec!["next".into(), "dioxus".into()],
            ComboboxInput {
                placeholder: "Pick frameworks...",
                aria_label: "Pick frameworks",
            }
            ComboboxContent {
                ComboboxList { aria_label: "Frameworks",
                    ComboboxEmpty { "No framework found." }
                    {
                        frameworks.iter().enumerate().map(|(i, (value, label))| rsx! {
                            ComboboxOption::<String> {
                                index: i,
                                value: value.to_string(),
                                text_value: label.to_string(),
                                {*label}
                                ComboboxItemIndicator {}
                            }
                        })
                    }
                }
            }
        }
    }
}
