use dioxus::prelude::*;

use crate::theme::use_theme;

#[cfg(feature = "theme_minimal")]
const _: &str = manganis::mg!(file("./styles/minimal/branding.css"));
const _: &str = manganis::mg!(file("./styles/core/branding.css"));

props!(LogoTextProps {
    #[props(into)]
    logo_src: String,

    #[props(into)]
    text: String,
});

pub fn LogoText(props: LogoTextProps) -> Element {
    let theme = use_theme();

    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            class: "dxc-logotext {theme().0}",

            img {
                src: "{props.logo_src}",
            }
            p { "{props.text}" }
        }
    }
}
