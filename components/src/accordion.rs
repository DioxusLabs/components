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
    let unique_id = crate::use_unique_id();
    let mut is_closed = use_signal(|| true);
    let mut text_height = use_signal(|| String::from("0"));

    let button_clicked = move |_| async move {
        if let Some(id) = unique_id() {
            if is_closed() {
                let mut eval = eval(
                    r#"
                let elementId = await dioxus.recv();  
                let element = document.getElementById(elementId);
                let scrollHeight = element.scrollHeight;
                console.log(scrollHeight);
                dioxus.send(scrollHeight);
                "#,
                );

                eval.send(id.into()).unwrap();
                let scroll_height = eval.recv().await.unwrap().to_string();
                text_height.set(scroll_height);
            }
        }
        is_closed.toggle();
    };


    let final_height = match is_closed() {
        true => "0px".to_string(),
        false => format!("{}px", text_height()),
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
                id: if let Some(id) = unique_id() { "{id}" },
                class: "dxc-accordian-text",
                class: if !is_closed() { "active" },
                height: final_height,

                {props.children}
            }
        }
    }
}
