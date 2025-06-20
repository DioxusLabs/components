use super::{ComponentDemoData, HighlightedCode};
macro_rules! examples {
    ($($name:ident),*) => {
        $(mod $name;)* pub (crate) static DEMOS: &[ComponentDemoData] = &[
            $(
                ComponentDemoData {
                    name: stringify!($name),
                    docs: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/docs.html")),
                    rs_highlighted: HighlightedCode {
                        light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/mod.rs.base16-ocean.light.html")),
                        dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/mod.rs.base16-ocean.dark.html")),
                    },
                    css_highlighted: HighlightedCode {
                        light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.base16-ocean.light.html")),
                        dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.base16-ocean.dark.html")),
                    },
                    component: $name::Demo,
                },
            )*
        ];
    };
}
examples!(
    accordion,
    alert_dialog,
    aspect_ratio,
    avatar,
    button,
    calendar,
    checkbox,
    collapsible,
    context_menu,
    dialog,
    dropdown_menu,
    hover_card,
    input,
    label,
    menubar,
    navbar,
    popover,
    progress,
    radio_group,
    scroll_area,
    select,
    separator,
    slider,
    switch,
    tabs,
    toast,
    toggle_group,
    toggle,
    toolbar,
    tooltip
);
