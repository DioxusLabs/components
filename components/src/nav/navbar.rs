use dioxus::prelude::*;

use super::Align;

const _: &str = manganis::mg!(file("./css-out/navbar.css"));
const HAMBURGER_ICON: &str = manganis::mg!(file("./images/align-text-pd.svg"));

#[derive(Clone)]
struct ItemSpacing(String);

#[derive(Clone, Copy)]
struct ItemAlign(Align);

props!(NavbarProps {
    #[props(into, optional, default = "3vw".to_string())]
    item_spacing: String,
    children: Element,
});

pub fn Navbar(props: NavbarProps) -> Element {
    let original_item_spacing = use_signal(|| props.item_spacing.clone());
    let mut item_spacing = use_context_provider(|| Signal::new(ItemSpacing(props.item_spacing)));
    let mut nav_open_mobile = use_signal(|| false);

    let on_hamburger_click = move |_| {
        nav_open_mobile.toggle();
        if nav_open_mobile() {
            item_spacing.set(ItemSpacing("0".to_string()));
        } else {
            item_spacing.set(ItemSpacing(original_item_spacing()));
        }
    };

    let children2 = props.children.clone();

    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            class: "dxc-navbar",

            {props.children}

            div {
                class: "dxc-navitem dxc-nav-hamburger",
                onclick: on_hamburger_click,
                img {
                    src: "{HAMBURGER_ICON}",
                }
            }
        }
        div {
            class: "dxc-navbar-vertical",
            class: if nav_open_mobile() { "open" },
            {children2}
        }
    }
}

props!(NavItemProps { children: Element });

pub fn NavItem(props: NavItemProps) -> Element {
    let item_spacing = use_context::<Signal<ItemSpacing>>();
    let align = try_consume_context::<ItemAlign>().unwrap_or_else(|| ItemAlign(Align::Left));

    let item_spacing = item_spacing();
    let spacing_style = match align.0 {
        Align::Left => format!("margin-right: {}", item_spacing.0),
        Align::Center => format!("margin-right: {}", item_spacing.0),
        Align::Right => format!("margin-left: {}", item_spacing.0),
    };

    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            style: "{spacing_style}",
            class: "dxc-navitem",

            {props.children}
        }
    }
}

props!(NavAlignLeftProps { children: Element });
pub fn NavAlignLeft(props: NavAlignLeftProps) -> Element {
    let _item_align = use_context_provider(|| ItemAlign(Align::Left));

    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            class: "dxc-nav-align-left",

            {props.children}
        }
    }
}

props!(NavAlignCenterProps { children: Element });
pub fn NavAlignCenter(props: NavAlignCenterProps) -> Element {
    let _item_align = use_context_provider(|| ItemAlign(Align::Center));

    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            class: "dxc-nav-align-center",

            {props.children}
        }
    }
}

props!(NavAlignRightProps { children: Element });
pub fn NavAlignRight(props: NavAlignRightProps) -> Element {
    let _item_align = use_context_provider(|| ItemAlign(Align::Right));

    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            class: "dxc-nav-align-right",

            {props.children}
        }
    }
}
