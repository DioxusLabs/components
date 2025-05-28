use dioxus::prelude::*;
use dioxus_primitives::hover_card::{HoverCard, HoverCardAlign, HoverCardContent, HoverCardSide, HoverCardTrigger};



#[component]
pub(super) fn HoverCardExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/hover_card/style.css") }

        div {
            class: "hover-card-example",
            style: "padding: 50px; display: flex; gap: 40px;",
            // User profile hover card
            HoverCard { class: "hover-card",
                HoverCardTrigger { class: "hover-card-trigger",
                    button { class: "user-trigger", "@johndoe" }
                }

                HoverCardContent { class: "hover-card-content", side: HoverCardSide::Bottom,
                    div { class: "user-card",
                        div { class: "user-card-header",
                            img {
                                class: "user-card-avatar",
                                src: "https://github.com/DioxusLabs.png",
                                alt: "User avatar",
                            }
                            div {
                                h4 { class: "user-card-name", "John Doe" }
                                p { class: "user-card-username", "@johndoe" }
                            }
                        }

                        p { class: "user-card-bio",
                            "Software developer passionate about Rust and web technologies. Building awesome UI components with Dioxus."
                        }

                        div { class: "user-card-stats",
                            div { class: "user-card-stat",
                                span { class: "user-card-stat-value", "142" }
                                span { class: "user-card-stat-label", "Posts" }
                            }
                            div { class: "user-card-stat",
                                span { class: "user-card-stat-value", "2.5k" }
                                span { class: "user-card-stat-label", "Followers" }
                            }
                            div { class: "user-card-stat",
                                span { class: "user-card-stat-value", "350" }
                                span { class: "user-card-stat-label", "Following" }
                            }
                        }
                    }
                }
            }

            // Product hover card
            HoverCard { class: "hover-card",
                HoverCardTrigger { class: "hover-card-trigger",
                    button { class: "product-trigger", "View Product" }
                }

                HoverCardContent {
                    class: "hover-card-content",
                    side: HoverCardSide::Right,
                    align: HoverCardAlign::Start,
                    div { class: "product-card",
                        img {
                            class: "product-card-image",
                            src: "https://images.unsplash.com/photo-1505740420928-5e560c06d30e",
                            alt: "Product image",
                        }
                        h4 { class: "product-card-title", "Wireless Headphones" }
                        p { class: "product-card-price", "$129.99" }
                        p { class: "product-card-description",
                            "High-quality wireless headphones with noise cancellation and 30-hour battery life."
                        }
                        div { class: "product-card-rating", "★★★★☆ (4.5)" }
                    }
                }
            }

            // Link hover card
            HoverCard { class: "hover-card",
                HoverCardTrigger { class: "hover-card-trigger",
                    a { href: "#", "Hover over this link" }
                }

                HoverCardContent {
                    class: "hover-card-content",
                    side: HoverCardSide::Top,
                    align: HoverCardAlign::Center,
                    div { style: "padding: 8px;",
                        p { style: "margin: 0;", "This link will take you to an external website." }
                    }
                }
            }
        }
    }
}
