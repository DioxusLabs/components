use super::super::component::*;
use dioxus::prelude::*;
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(EnumIter, Display)]
enum Animals {
    Cat,
    Cow,
    Dog,
    Fox,
    Pig,
}

impl Animals {
    const fn emoji(&self) -> &'static str {
        match self {
            Animals::Cat => "🐱",
            Animals::Cow => "🐮",
            Animals::Dog => "🐶",
            Animals::Fox => "🦊",
            Animals::Pig => "🐷",
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let items = Animals::iter()
        .map(|a| {
            rsx! {
                {format!("{} {a}", a.emoji())}
            }
        })
        .collect();

    rsx! {
        DragAndDropList { items }
    }
}
