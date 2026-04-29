use dioxus::prelude::*;

use crate::components::avatar::{
    Avatar, AvatarFallback, AvatarImage, AvatarImageSize, AvatarShape,
};
use crate::components::button::{Button, ButtonVariant};
use crate::components::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemMediaVariant, ItemTitle,
};
use crate::components::select::{
    SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectMulti, SelectOption,
    SelectTrigger,
};
use crate::components::tabs::component::{TabList, TabTrigger, Tabs};
use crate::components::virtual_list::VirtualList;
use crate::dashboard::common::{
    lookup_message, FolderId, IconKind, LucideIcon, MessageState, MessageTag, TabId, TABS,
};

use super::avatars::avatar_profile_for_key;
use super::filters::tab_count;

#[derive(Clone)]
pub(super) enum ListRow {
    DayHeader(&'static str),
    Message(MessageState),
}

pub(super) fn flatten_rows(messages: &[MessageState]) -> Vec<ListRow> {
    let mut out = Vec::with_capacity(messages.len() + 4);
    let mut last_day: Option<&'static str> = None;
    for state in messages.iter() {
        let day = lookup_message(state.source_id).day;
        if last_day != Some(day) {
            out.push(ListRow::DayHeader(day));
            last_day = Some(day);
        }
        out.push(ListRow::Message(state.clone()));
    }
    out
}

fn estimate_message_row_height(state: &MessageState) -> u32 {
    let m = lookup_message(state.source_id);
    let snippet_lines = if m.snippet.len() > 78 { 2 } else { 1 };
    let has_meta_row = !state.tags.is_empty() || m.has_attachment || state.starred || state.flagged;

    32  // vertical padding + borders + margins
        + 21 // subject row (now the prominent line)
        + 16 // sender row
        + 19 * snippet_lines
        + if has_meta_row { 22 } else { 0 }
        + if has_meta_row { 12 } else { 8 } // content row gaps
}

#[component]
pub(super) fn ListPane(
    rows: ReadSignal<Vec<ListRow>>,
    messages_snapshot: ReadSignal<Vec<MessageState>>,
    active_folder_id: ReadSignal<FolderId>,
    active_search_query: ReadSignal<String>,
    active_selected_tags: ReadSignal<Vec<MessageTag>>,
    messages: Signal<Vec<MessageState>>,
    selected_id: Signal<String>,
    selected_uid: ReadSignal<String>,
    mut active_tab: Signal<TabId>,
    mut selected_tags: Signal<Vec<MessageTag>>,
    mut read_open: Signal<bool>,
) -> Element {
    let row_count = rows.read().len();
    let rows_for_estimate = rows;
    let rows_for_render = rows;
    let folder_id = *active_folder_id.read();
    let query = active_search_query.read().clone();
    let tags = active_selected_tags.read().clone();

    rsx! {
        section { class: "ec-list-pane",
            div { class: "ec-list-toolbar",
                Tabs {
                    default_value: TabId::All.as_str().to_string(),
                    horizontal: true,
                    on_value_change: move |v: String| {
                        if let Some(tab) = TabId::from_str(&v) {
                            active_tab.set(tab);
                        }
                        read_open.set(false);
                    },
                    TabList {
                        for (idx, tab) in TABS.iter().enumerate() {
                            TabTrigger {
                                key: "{tab.id.as_str()}",
                                value: tab.id.as_str().to_string(),
                                index: idx,
                                {tab.label}
                                span { class: "ec-muted", " {tab_count(&messages_snapshot.read(), folder_id, tab.id, query.as_str(), &tags)}" }
                            }
                        }
                    }
                }
                SelectMulti::<MessageTag> {
                    values: Some(tags.clone()),
                    default_values: vec![],
                    on_values_change: move |values| {
                        selected_tags.set(values);
                        read_open.set(false);
                    },
                    SelectTrigger {
                        class: "ec-filter-trigger",
                        aria_label: "Filter by tag",
                        LucideIcon { kind: IconKind::Filter }
                        if !tags.is_empty() {
                            span { class: "ec-filter-count", "{tags.len()}" }
                        }
                    }
                    SelectList {
                        class: "ec-filter-list",
                        aria_label: "Filter by tag",
                        SelectGroup {
                            SelectGroupLabel { "Tags" }
                            for (index, tag) in MessageTag::ALL.iter().enumerate() {
                                SelectOption::<MessageTag> {
                                    key: "{tag.label()}",
                                    index,
                                    value: *tag,
                                    text_value: "{tag.label()}",
                                    {tag.label()}
                                    SelectItemIndicator {}
                                }
                            }
                        }
                    }
                }
            }

            VirtualList {
                class: "ec-list-scroll",
                count: row_count,
                buffer: 6usize,
                estimate_size: move |idx: usize| match &rows_for_estimate.read()[idx] {
                    ListRow::DayHeader(_) => 34,
                    ListRow::Message(state) => estimate_message_row_height(state),
                },
                render_item: move |idx: usize| {
                    let row = rows_for_render.read()[idx].clone();
                    match row {
                        ListRow::DayHeader(day) => rsx! {
                            div { class: "ec-day", {day} }
                        },
                        ListRow::Message(state) => rsx! {
                            MessageRow {
                                key: "{state.uid}",
                                state: state.clone(),
                                selected_id,
                                selected_uid: selected_uid.read().clone(),
                                read_open,
                                messages,
                            }
                        },
                    }
                },
            }
        }
    }
}

#[component]
fn MessageRow(
    state: MessageState,
    mut selected_id: Signal<String>,
    selected_uid: String,
    mut read_open: Signal<bool>,
    mut messages: Signal<Vec<MessageState>>,
) -> Element {
    let is_selected = selected_uid == state.uid;
    let uid_for_click = state.uid.clone();
    let uid_for_star = state.uid.clone();
    let uid_for_trash = state.uid.clone();
    let selected_uid_for_trash = selected_uid.clone();
    // Read live state from the messages signal — the cloned `state` prop is
    // re-cloned by VirtualList from a snapshot taken at row creation time and
    // does not refresh on in-place mutations like toggling starred.
    let live = {
        let msgs = messages.read();
        msgs.iter()
            .find(|s| s.uid == state.uid)
            .cloned()
            .unwrap_or_else(|| state.clone())
    };
    let m = lookup_message(live.source_id);
    let starred = live.starred;
    let unread = live.unread;
    let flagged = live.flagged;
    let tags = live.tags.clone();
    let mut classes = String::from("ec-row");
    if unread {
        classes.push_str(" ec-unread");
    }
    if starred {
        classes.push_str(" ec-starred");
    }
    if flagged {
        classes.push_str(" ec-flagged");
    }

    rsx! {
        Item {
            class: classes,
            role: "option",
            tabindex: 0,
            onclick: move |_| {
                selected_id.set(uid_for_click.clone());
                read_open.set(true);
            },
            "aria-selected": is_selected,
            "data-selected": is_selected,

            ItemMedia { variant: ItemMediaVariant::Icon,
                Avatar {
                    size: AvatarImageSize::Small,
                    shape: AvatarShape::Circle,
                    AvatarImage {
                        src: "{avatar_profile_for_key(m.from_addr).src}",
                        alt: "{m.from}",
                    }
                    AvatarFallback { {m.initials} }
                }
            }
            ItemContent {
                ItemTitle {
                    span { class: "ec-row-subject", {m.subject} }
                }
                div { class: "ec-row-sender",
                    span { class: "ec-row-from", {m.from} }
                }
                ItemDescription { class: "ec-row-snippet", {m.snippet} }
                if !tags.is_empty() || m.has_attachment || flagged {
                    div { class: "ec-muted ec-row-tags",
                        if flagged {
                            LucideIcon { kind: IconKind::Flag, size: 12 }
                        }
                        for (i, tag) in tags.iter().enumerate() {
                            span { key: "{tag.label()}",
                                if i > 0 { " · " }
                                {tag.label()}
                            }
                        }
                        if m.has_attachment {
                            LucideIcon { kind: IconKind::Paperclip, size: 12 }
                        }
                    }
                }
            }
            ItemActions {
                span { class: "ec-muted ec-row-time", {m.time} }
                div { class: "ec-row-action-group",
                Button {
                    variant: ButtonVariant::Ghost,
                    r#type: "button",
                    class: "ec-row-action ec-row-action-trash",
                    aria_label: "Move to trash",
                    onclick: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        let mut msgs = messages.write();
                        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid_for_trash) {
                            entry.folder_id = FolderId::Trash;
                        }
                        drop(msgs);
                        if uid_for_trash == selected_uid_for_trash {
                            read_open.set(false);
                        }
                    },
                    LucideIcon { kind: IconKind::Trash, size: 16 }
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    r#type: "button",
                    class: "ec-row-action ec-row-action-star",
                    "data-active": starred,
                    aria_label: if starred { "Unstar message" } else { "Star message" },
                    onclick: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        let mut msgs = messages.write();
                        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid_for_star) {
                            entry.starred = !entry.starred;
                        }
                    },
                    LucideIcon {
                        kind: if starred { IconKind::StarFilled } else { IconKind::StarOutline },
                        size: 16,
                    }
                }
                }
            }
        }
    }
}
