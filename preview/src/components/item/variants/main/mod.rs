use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let img_url = "https://images.unsplash.com/photo-1633332755192-727a05c4013d?w=100&q=80";

    rsx! {
      div {
        display: "flex",
        flex_direction: "column",
        gap: "2rem",
        width: "100%",
        max_width: "500px",
        margin: "0 auto",
        padding: "2rem",

        // Section 1: Variants
        div {
          h2 { class: "mb-4 font-bold", "Variants" }
          ItemGroup {
            Item { variant: ItemVariant::Default,
              ItemMedia { variant: ItemMediaVariant::Icon,
                svg {
                  fill: "none",
                  stroke: "currentColor",
                  stroke_width: "2",
                  view_box: "0 0 24 24",
                  path { d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" }
                }
              }
              ItemContent {
                ItemTitle { "Default Variant" }
                ItemDescription { "Standard item without borders or background." }
              }
            }
            Item { variant: ItemVariant::Outline,
              ItemMedia { variant: ItemMediaVariant::Icon,
                svg {
                  fill: "none",
                  stroke: "currentColor",
                  stroke_width: "2",
                  view_box: "0 0 24 24",
                  path { d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" }
                }
              }
              ItemContent {
                ItemTitle { "Outline Variant" }
                ItemDescription { "Item with a subtle border." }
              }
            }

            Item { variant: ItemVariant::Muted,
              ItemMedia { variant: ItemMediaVariant::Icon,
                svg {
                  fill: "none",
                  stroke: "currentColor",
                  stroke_width: "2",
                  view_box: "0 0 24 24",
                  path { d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" }
                }
              }
              ItemContent {
                ItemTitle { "Muted Variant" }
                ItemDescription { "Item with a soft background color." }
              }
            }
          }
        }

        // Section 2: Sizes & Composition
        div {
          h2 { class: "mb-4 font-bold", "Sizes & Composition" }
          ItemGroup {
            // Full composition example
            Item {
              size: ItemSize::Default,
              variant: ItemVariant::Outline,
              ItemHeader {
                span { class: "text-xs font-semibold text-blue-500",
                  "Transaction"
                }
                span { class: "text-xs text-gray-500", "ID: #9921" }
              }
              ItemMedia { variant: ItemMediaVariant::Image,
                img { src: "{img_url}", alt: "User Avatar" }
              }
              ItemContent {
                ItemTitle { "Default Size" }
                ItemDescription { "Comprehensive layout with header, media, actions and footer." }
              }
              ItemActions {
                div { class: "font-mono font-bold text-red-500", "-$24.99" }
              }
              ItemFooter {
                button { class: "text-xs text-gray-500 hover:text-black",
                  "Dismiss"
                }
              }
            }

            Item { size: ItemSize::Sm, variant: ItemVariant::Outline,
              ItemMedia { variant: ItemMediaVariant::Image,
                img { src: "{img_url}", alt: "User Avatar" }
              }
              ItemContent {
                ItemTitle { "Small Size" }
                ItemDescription { "A compact size for dense layouts." }
              }
            }

            Item { size: ItemSize::Xs, variant: ItemVariant::Outline,
              ItemMedia { variant: ItemMediaVariant::Image,
                img { src: "{img_url}", alt: "User Avatar" }
              }
              ItemContent {
                ItemTitle { "Extra Small Size" }
                ItemDescription { "The most compact size available." }
              }
            }
          }
        }
      }
    }
}
