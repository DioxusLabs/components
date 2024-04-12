use dioxus::prelude::*;
use dioxus_components::{layout::*, display::*};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {

        Accordion { id: "hi", title: "Got Questions?",
            AccordionItem { title: "What is this?", "This is just a preview of Dioxus Components! Pretty cool, right?" }

            AccordionItem { title: "How do I use it?",
                "Check out our "
                a { href: "https://github.com/DioxusLabs/components", "GitHub" }
                "!"
            }
        }

        Row { id: "hi", style: "background-color: #f5f5f5; padding: 40px;",
            Column {
                h1 { "Hello, World!" }
                p { "Welcome to Dioxus Components!" }
            }
            Column {
                h1 { "Hello, World!" }
                p { "Welcome to Dioxus Components!" }
            }
        }
    }
}
