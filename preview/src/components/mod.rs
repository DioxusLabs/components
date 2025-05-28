use super::ComponentDemoData;


macro_rules! examples {
    ($($name:ident),*) => {
        $(
            mod $name;
        )*

        pub(crate) static DEMOS: &[ComponentDemoData] = &[
            $(
                ComponentDemoData {
                    name: stringify!($name),
                    rs_source: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/mod.rs.html")),
                    css_source: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.html")),
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
    dropdown_menu,
    hover_card,
    form,
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
