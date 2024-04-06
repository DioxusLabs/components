use dioxus::prelude::*;

const _: &str = manganis::mg!(file("./css-out/accordion.css"));
const ARROW_DOWN_IMG: &str = manganis::mg!(file("./src/images/arrow-down-pd.svg"));

props!(AccordionProps {
    #[props(into)]
    title: Option<String>,
    children: Element,
});

pub fn Accordion(props: AccordionProps) -> Element {
    rsx! {
        div {
            id: if let Some(id) = props.id { "{id}" },
            class: if let Some(class) = props.class { "{class}" },
            style: if let Some(style) = props.style { "{style}" },
            class: "dxc-accordian",

            if let Some(title) = props.title {
                h2 { "{title}" }
            }

            {props.children}
        }
    }
}

props!(AccordionItemProps {
    #[props(into)]
    title: String,
    children: Element,
});

pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let mut is_closed = use_signal(|| true);
    let mut text_height = use_signal(|| -1);
    let mut has_text_height = use_signal(|| false);

    let button_clicked = move |_| {
        is_closed.toggle();
    };


    let on_text_mount = move |evt: Event<MountedData>| async move {
        let rect = evt.get_client_rect().await.unwrap();
        let height = rect.height().ceil() as i32;
        text_height.set(height);
        has_text_height.set(true);
    };

    rsx! {
        div {
            id: if let Some(id) = props.id { "{id}" },
            class: if let Some(class) = props.class { "{class}" },
            style: if let Some(style) = props.style { "{style}" },
            class: "dxc-accordian-item",

            button {
                class: "dxc-accordian-button",
                class: if !is_closed() { "active" },
                onclick: button_clicked,

                p { "{props.title}" }
                img { 
                    src: "{ARROW_DOWN_IMG}"
                }
            }

            div {
                class: "dxc-accordian-text",
                height: if is_closed() && has_text_height() { "0px" } else if has_text_height() { "{text_height()}" },
                onmounted: on_text_mount,

                {props.children}
            }
        }
    }
}
