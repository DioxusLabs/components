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
use super::state::{
    close_read_pane, select_message, set_active_tab, set_selected_tags, tab_count, update_message,
    EmailClientState, EmailClientStateStoreExt,
};

#[derive(Clone, PartialEq)]
pub(super) enum ListRow {
    DayHeader(&'static str),
    Message(String),
}

pub(super) fn flatten_rows(state: Store<EmailClientState>, message_ids: &[String]) -> Vec<ListRow> {
    let messages_store = state.messages();
    let messages = messages_store.read();
    let mut out = Vec::with_capacity(message_ids.len() + 4);
    let mut last_day: Option<&'static str> = None;
    for uid in message_ids {
        let Some(message) = messages.get(uid.as_str()) else {
            continue;
        };
        let day = lookup_message(message.source_id).day;
        if last_day != Some(day) {
            out.push(ListRow::DayHeader(day));
            last_day = Some(day);
        }
        out.push(ListRow::Message(uid.clone()));
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
    state: Store<EmailClientState>,
    visible_ids: ReadSignal<Vec<String>>,
    selected_uid: ReadSignal<String>,
) -> Element {
    let rows = use_memo(move || flatten_rows(state, &visible_ids.read()));
    let row_count = rows.read().len();
    let rows_for_estimate = rows;
    let rows_for_render = rows;
    let query = state.search_query().cloned();
    let tags = state.selected_tags().cloned();

    rsx! {
        section { class: "ec-list-pane",
            div { class: "ec-list-toolbar",
                Tabs {
                    default_value: TabId::All.as_str().to_string(),
                    horizontal: true,
                    on_value_change: move |v: String| {
                        if let Some(tab) = TabId::from_str(&v) {
                            set_active_tab(state, tab);
                        }
                    },
                    TabList {
                        for (idx, tab) in TABS.iter().enumerate() {
                            TabTrigger {
                                key: "{tab.id.as_str()}",
                                value: tab.id.as_str().to_string(),
                                index: idx,
                                {tab.label}
                                span { class: "ec-muted", " {tab_count(state, tab.id, query.as_str(), &tags)}" }
                            }
                        }
                    }
                }
                SelectMulti::<MessageTag> {
                    values: Some(tags.clone()),
                    default_values: vec![],
                    on_values_change: move |values| {
                        set_selected_tags(state, values);
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
                    ListRow::Message(uid) => state
                        .messages()
                        .read()
                        .get(uid.as_str())
                        .map(estimate_message_row_height)
                        .unwrap_or(80),
                },
                render_item: move |idx: usize| {
                    let row = rows_for_render.read()[idx].clone();
                    match row {
                        ListRow::DayHeader(day) => rsx! {
                            div { class: "ec-day", {day} }
                        },
                        ListRow::Message(uid) => rsx! {
                            MessageRow {
                                key: "{uid}",
                                state,
                                uid,
                                selected_uid: selected_uid.read().clone(),
                            }
                        },
                    }
                },
            }
        }
    }
}

#[component]
fn MessageRow(state: Store<EmailClientState>, uid: String, selected_uid: String) -> Element {
    let Some(message) = state.messages().get(uid.clone()) else {
        return rsx! {};
    };
    let live = message.read().clone();
    let is_selected = selected_uid == uid;
    let uid_for_click = uid.clone();
    let uid_for_star = uid.clone();
    let uid_for_trash = uid.clone();
    let selected_uid_for_trash = selected_uid.clone();
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
                select_message(state, uid_for_click.clone());
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
                        update_message(state, uid_for_trash.clone(), |entry| {
                            entry.folder_id = FolderId::Trash;
                        });
                        if uid_for_trash == selected_uid_for_trash {
                            close_read_pane(state);
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
                        update_message(state, uid_for_star.clone(), |entry| {
                            entry.starred = !entry.starred;
                        });
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
