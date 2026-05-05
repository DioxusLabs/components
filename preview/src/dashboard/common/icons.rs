use dioxus::prelude::*;
use dioxus_primitives::icon::Icon;

#[derive(Clone, Copy, PartialEq)]
pub enum IconKind {
    Inbox,
    Send,
    Pen,
    Archive,
    Trash,
    StarOutline,
    StarFilled,
    Paperclip,
    Filter,
    ArrowLeft,
    Flag,
    Snooze,
    X,
}

#[component]
pub fn LucideIcon(kind: IconKind, #[props(default = 16)] size: u32) -> Element {
    let size_str = format!("{size}px");
    let (fill, paths) = match kind {
        IconKind::Inbox => (
            "none",
            rsx! {
                path { d: "M22 12h-6l-2 3h-4l-2-3H2" }
                path { d: "M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11Z" }
            },
        ),
        IconKind::Send => (
            "none",
            rsx! {
                path { d: "m22 2-7 20-4-9-9-4Z" }
                path { d: "M22 2 11 13" }
            },
        ),
        IconKind::Pen => (
            "none",
            rsx! {
                path { d: "M12 20h9" }
                path { d: "M16.5 3.5a2.121 2.121 0 1 1 3 3L7 19l-4 1 1-4Z" }
            },
        ),
        IconKind::Archive => (
            "none",
            rsx! {
                rect {
                    x: "2",
                    y: "4",
                    width: "20",
                    height: "5",
                    rx: "2",
                }
                path { d: "M4 9v9a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9" }
                path { d: "M10 13h4" }
            },
        ),
        IconKind::Trash => (
            "none",
            rsx! {
                path { d: "M3 6h18" }
                path { d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" }
                path { d: "M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" }
            },
        ),
        IconKind::StarOutline => (
            "none",
            rsx! {
                path { d: "m12 2 3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" }
            },
        ),
        IconKind::StarFilled => (
            "currentColor",
            rsx! {
                path { d: "m12 2 3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" }
            },
        ),
        IconKind::Paperclip => (
            "none",
            rsx! {
                path { d: "m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 17.99 8.83l-8.59 8.57a2 2 0 1 1-2.83-2.83l8.49-8.48" }
            },
        ),
        IconKind::Filter => (
            "none",
            rsx! {
                polygon { points: "22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3" }
            },
        ),
        IconKind::ArrowLeft => (
            "none",
            rsx! {
                path { d: "m12 19-7-7 7-7" }
                path { d: "M19 12H5" }
            },
        ),
        IconKind::Flag => (
            "none",
            rsx! {
                path { d: "M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z" }
                line {
                    x1: "4",
                    y1: "22",
                    x2: "4",
                    y2: "15",
                }
            },
        ),
        IconKind::Snooze => (
            "none",
            rsx! {
                circle { cx: "12", cy: "13", r: "8" }
                path { d: "M5 3 2 6" }
                path { d: "m22 6-3-3" }
                path { d: "M12 9v4l2 2" }
            },
        ),
        IconKind::X => (
            "none",
            rsx! {
                path { d: "M18 6 6 18" }
                path { d: "m6 6 12 12" }
            },
        ),
    };

    rsx! {
        Icon {
            width: "{size_str}",
            height: "{size_str}",
            fill,
            stroke_width: 1.75,
            "aria-hidden": "true",
            {paths}
        }
    }
}
