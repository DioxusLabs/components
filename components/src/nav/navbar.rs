use super::Align;
use dioxus::prelude::*;
use dioxus_sdk::utils::window::use_window_size;

const _: &str = manganis::mg!(file("./css-out/navbar.css"));
const HAMBURGER_ICON: &str = manganis::mg!(file("./images/align-text-pd.svg"));

#[derive(Clone)]
struct ItemSpacing(String);

#[derive(Clone, Copy)]
struct ItemAlign(Align);

props!(NavbarProps {
    /// The minimum size in pixels before the navbar collapses into a vertical navbar.
    /// Defaults to `425px`, Chrome's Large Mobile size.
    #[props(optional, default = 425)]
    collapse_width: u32,
    /// The spacing between [`NavItem`]s.
    /// Defaults to `3vw`.
    #[props(into, optional, default = "3vw".to_string())]
    item_spacing: String,

    children: Element,
});

pub fn Navbar(props: NavbarProps) -> Element {
    // Spacing context provider for spacing between nav elements.
    let mut item_spacing =
        use_context_provider(|| Signal::new(ItemSpacing(props.item_spacing.clone())));

    // Handle nav opening/closing on mobile.
    let window_size = use_window_size();
    let mut mobile_mode = use_signal(|| false);
    let mut nav_open_mobile = use_signal(|| false);
    let item_spacing_prop = props.item_spacing.clone();

    use_memo(move || {
        if window_size().width > props.collapse_width {
            nav_open_mobile.set(false);
            mobile_mode.set(false);
            item_spacing.set(ItemSpacing(item_spacing_prop.clone()));
        } else {
            mobile_mode.set(true);
            item_spacing.set(ItemSpacing("NONE".to_string()));
        }
    });

    let on_hamburger_click = move |_| {
        nav_open_mobile.set(!nav_open_mobile());
    };

    // Handle item_spacing context updates from props
    use_memo(use_reactive((&props.item_spacing,), move |(spacing,)| {
        if !mobile_mode() {
            item_spacing.set(ItemSpacing(spacing));
        }
    }));

    let children2 = props.children.clone();

    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            class: "dxc-navbar",
            class: if mobile_mode() { "mobile" },

            {props.children},

            if mobile_mode() {
                div {
                    class: "dxc-navitem dxc-nav-hamburger",
                    onclick: on_hamburger_click,
                    img { src: "{HAMBURGER_ICON}" }
                }
            }
        }

        if nav_open_mobile() {
            div {
                class: "dxc-navbar-vertical",
                {children2}
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
