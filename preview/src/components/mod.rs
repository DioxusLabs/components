use super::{ComponentDemoData, HighlightedCode};

macro_rules! examples {
    ($($name:ident),*) => {
        $(
            mod $name;
        )*

        pub(crate) static DEMOS: &[ComponentDemoData] = &[
            $(
                ComponentDemoData {
                    name: stringify!($name),
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
    aspect_ratio,
    avatar,
    calendar,
    context_menu,
    checkbox,
    dropdown_menu,
    hover_card,
    menubar,
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
    toolbar,
    tooltip
);
