use dioxus::prelude::*;
use dioxus_aria::{Button, Icon};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut muted = use_signal(|| false);

    let icon_src = match muted() {
        true => "https://i.fbcd.co/products/resized/resized-750-500/3cf763d411d8098d6d77892a93fb27ff802f7aed02b32ce350653463097d3c3b.jpg",
        false => "https://www.iconpacks.net/icons/1/free-microphone-icon-342-thumb.png",
    };

    let icon = Icon {
        src: icon_src.to_string(),
        width: 50,
        height: 50,
    };

    rsx! {
        Button {
            label: "Save",
        }

        Button {
            label: "Mute",
            on_toggled: move |_| {},
        }

        Button {
            label: "Mute",
            icon,
            on_toggled: move |val| muted.set(val),
        }
    }
}
