use dioxus::prelude::*;

const _STYLE: &str = manganis::mg!(file("css-out/accordian.css"));

props!(AccordianProps {
    #[props(into)]
    title: Option<String>,
    children: Element,
});

pub fn Accordian(props: AccordianProps) -> Element {
    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,

            if let Some(title) = props.title {
                h2 { {title} }
            }


            {props.children}
        }
    }
}

props!(AccordianItemProps {
    #[props(into)]
    title: String,
    children: Element,
});

pub fn AccordianItem(props: AccordianItemProps) -> Element {
    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,

            h3 { "{props.title}" }
            {props.children}
        }
    }
}
