use dioxus::prelude::*;

use crate::theme::use_theme;

const _: &str = manganis::mg!(file("./css-out/branding.css"));

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
