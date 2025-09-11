use super::{ComponentDemoData, ComponentVariantDemoData, HighlightedCode};
macro_rules! examples {
    ($($name:ident $([$($variant:ident),*])?),*) => {
        $(
            mod $name {
                pub(crate) mod component;
                pub(crate) mod variants {
                    pub(crate) mod main;
                    $(
                        $(
                            pub(crate) mod $variant;
                        )*
                    )?
                }
            }
        )*
        pub (crate) static DEMOS: &[ComponentDemoData] = &[
            $(
                ComponentDemoData {
                    name: stringify!($name),
                    docs: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/docs.html")),
                    component: HighlightedCode {
                        light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/component.rs.base16-ocean.light.html")),
                        dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/component.rs.base16-ocean.dark.html")),
                    },
                    style: HighlightedCode {
                        light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.base16-ocean.light.html")),
                        dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.base16-ocean.dark.html")),
                    },
                    variants: &[
                        ComponentVariantDemoData {
                            name: "main",
                            rs_highlighted: HighlightedCode {
                                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/main/mod.rs.base16-ocean.light.html")),
                                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/main/mod.rs.base16-ocean.dark.html")),
                            },
                            component: $name::variants::main::Demo,
                        },
                        $(
                            $(
                                ComponentVariantDemoData {
                                    name: stringify!($variant),
                                    rs_highlighted: HighlightedCode {
                                        light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/", stringify!($variant), "/mod.rs.base16-ocean.light.html")),
                                        dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/", stringify!($variant), "/mod.rs.base16-ocean.dark.html")),
                                    },
                                    component: $name::variants::$variant::Demo,
                                },
                            )*
                        )?
                    ]
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
    calendar[simple, internationalized],
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
