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
    let _ctx = use_context_provider(|| Signal::new(ItemSpacing(props.item_spacing)));

    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            class: "dxc-navbar",

            {props.children}

            div {
                class: "dxc-navitem dxc-nav-hamburger",
                img {
                    src: "{HAMBURGER_ICON}",
                }
            }
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
