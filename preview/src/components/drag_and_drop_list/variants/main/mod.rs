use super::super::component::*;
use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};
use dioxus::prelude::*;
use std::rc::Rc;

#[derive(Clone, Copy)]
struct TaskView {
    code: &'static str,
    title: &'static str,
    status: &'static str,
    assignee: &'static str,
    avatar: &'static str,
    due: &'static str,
    urgent: bool,
}

// Avatar images are the same GitHub portraits used by the Avatar
// component demo. Each task indexes into this list and the initials fall
// back in if the image fails to load.
const AVATARS: &[&str] = &[
    "https://avatars.githubusercontent.com/u/66571940?s=96&v=4",
    "https://avatars.githubusercontent.com/u/1007307?s=70&v=4",
    "https://avatars.githubusercontent.com/u/10237910?s=70&v=4",
];

const TASKS: &[TaskView] = &[
    TaskView {
        code: "LNC-128",
        title: "Ship Q2 product roadmap",
        status: "In progress",
        assignee: "JS",
        avatar: AVATARS[0],
        due: "Today",
        urgent: true,
    },
    TaskView {
        code: "LNC-142",
        title: "Redesign onboarding flow",
        status: "In review",
        assignee: "MP",
        avatar: AVATARS[1],
        due: "Apr 24",
        urgent: false,
    },
    TaskView {
        code: "LNC-147",
        title: "Audit payment webhook logs",
        status: "In progress",
        assignee: "KT",
        avatar: AVATARS[2],
        due: "Apr 29",
        urgent: false,
    },
    TaskView {
        code: "LNC-151",
        title: "Draft changelog for v2.4",
        status: "To do",
        assignee: "AR",
        avatar: AVATARS[0],
        due: "May 02",
        urgent: false,
    },
    TaskView {
        code: "LNC-156",
        title: "Migrate analytics to ClickHouse",
        status: "Blocked",
        assignee: "DL",
        avatar: AVATARS[1],
        due: "May 05",
        urgent: false,
    },
    TaskView {
        code: "LNC-160",
        title: "Archive legacy API endpoints",
        status: "Backlog",
        assignee: "SC",
        avatar: AVATARS[2],
        due: "Later",
        urgent: false,
    },
];

// Item height + gap (kept in sync with style.css).
const ITEM_H: f64 = 66.0;
const ITEM_GAP: f64 = 6.0;
const ITEM_UNIT: f64 = ITEM_H + ITEM_GAP;

#[component]
pub fn Demo() -> Element {
    let items: Vec<Element> = TASKS.iter().map(|t| task_item(*t)).collect();

    // Cursor state, fed into CSS via `--dx-ball-i` on the container.
    // `ball_i` is a fractional item index: 0.0 == centre of item 0.
    // The active-gate is handled entirely in CSS via `:has(.dx-drop-indicator)`
    // — see style.css — so we don't need a signal for it here.
    let mut ball_i = use_signal(|| 0.0f64);
    let mut list_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let container_style = format!("--dx-ball-i: {:.3};", ball_i());

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "dx-tasks-demo", style: "{container_style}",
            div { class: "dx-tasks-header",
                div {
                    h3 { class: "dx-tasks-title", "Launch priorities" }
                    p { class: "dx-tasks-subtitle",
                        "Drag to reorder · top is highest priority"
                    }
                }
                span { class: "dx-tasks-count", "6 active" }
            }
            div {
                class: "dx-tasks-list-wrap",
                onmounted: move |event| list_ref.set(Some(event.data())),
                ondragover: move |event: Event<DragData>| {
                    event.prevent_default();
                    async move {
                        let Some(md) = list_ref() else { return };
                        let Ok(rect) = md.get_client_rect().await else { return };
                        let rel_y = event.client_coordinates().y - rect.origin.y;
                        // `frac` is 0.0 at the vertical centre of item 0, advancing by 1.0 per item.
                        let frac = (rel_y - ITEM_H / 2.0) / ITEM_UNIT;
                        ball_i.set(frac);
                    }
                },
                DragAndDropList { items }
            }
        }
    }
}

fn task_item(t: TaskView) -> Element {
    let urgent_attr = if t.urgent { "true" } else { "false" };
    let avatar_label = format!("Assigned to {}", t.assignee);

    rsx! {
        div { key: "{t.code}", class: "dx-task-card",
            div { class: "dx-task-body",
                div { class: "dx-task-title", "{t.title}" }
                div { class: "dx-task-meta",
                    span { class: "dx-task-code", "{t.code}" }
                    span { class: "dx-task-sep", aria_hidden: "true" }
                    span { "{t.status}" }
                    span { class: "dx-task-sep", aria_hidden: "true" }
                    span {
                        class: "dx-task-due",
                        "data-urgent": "{urgent_attr}",
                        "{t.due}"
                    }
                }
            }
            Avatar { size: AvatarImageSize::Small, aria_label: "{avatar_label}",
                AvatarImage {
                    class: "dx-avatar-image",
                    src: "{t.avatar}",
                    alt: "{avatar_label}",
                    // Block the browser's native image drag so the LI can claim
                    // the gesture; without this, mouse-dragging the portrait
                    // starts an image drag instead of reordering the task.
                    draggable: "false",
                }
                AvatarFallback { class: "dx-avatar-fallback", "{t.assignee}" }
            }
        }
    }
}
