use dioxus::prelude::*;
use dioxus_components::{Accordian, AccordianItem};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Accordian {
            title: "FAQ",
            AccordianItem {
                title: "Q: Dioxus won't compile on desktop!",
                content: "That isn't a question, but do you have all required libraries installed for your system?"
            }
        }
    }
}
