use dioxus::prelude::*;

const _: &str = manganis::mg!(file("./css-out/branding.css"));

props!(LogoTextProps {
    #[props(into)]
    logo_src: String,

    #[props(into)]
    text: String,
});

pub fn LogoText(props: LogoTextProps) -> Element {
    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            class: "dxc-logotext",

            img {
                src: "{props.logo_src}",
            }
            p { "{props.text}" }
        }
    }
}
