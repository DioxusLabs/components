use dioxus::prelude::*;
use dioxus_components::{Accordian, AccordianItem};


fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Navbar {
        //     NavItem {
        //         align: Align::Left,
        //         src: "{DIOXUS_LOGO}",
        //         "Components"
        //     }
        //     NavItem {
        //         align: Align::Left,
        //         to: "guides",
        //         "Guides"
        //     }
        //     NavItem {
        //         align: Align::Left,
        //         Link {
        //             to: "docs",
        //             "Docs"
        //         }
        //     }
        //     NavItem {
        //         align: Align::Right,
        //         "Build cool stuff ✌️"
        //     }
        // }

        // Hero {

        // }


        Accordian {
            id: "hi",
            title: "Got Questions?",
            AccordianItem {
                title: "What is this?",
                "This is just a preview of Dioxus Components! Pretty cool, right?"
            }

            AccordianItem {
                title: "How do I use it?",
                "Check out our ",
                a {
                    href: "https://github.com/DioxusLabs/components",
                    "GitHub"
                },
                "!"
            }
        }
    }
}
