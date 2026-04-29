use dioxus::prelude::*;

use crate::components::avatar::{
    Avatar, AvatarFallback, AvatarImage, AvatarImageSize, AvatarProfile, AvatarShape,
    AVATAR_PROFILE_OPTIONS,
};
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::dropdown_menu::component::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::input::Input;
use crate::components::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemMediaVariant, ItemTitle,
};
use crate::components::select::{
    SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectMulti, SelectOption,
    SelectTrigger,
};
use crate::components::separator::Separator;
use crate::components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel,
    SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuBadge, SidebarMenuButton,
    SidebarMenuButtonSize, SidebarMenuItem, SidebarProvider, SidebarRail, SidebarTrigger,
    SidebarVariant,
};
use crate::components::tabs::component::{TabList, TabTrigger, Tabs};
use crate::components::textarea::Textarea;
use crate::components::toolbar::component::{
    Toolbar, ToolbarButton, ToolbarGroup, ToolbarSeparator,
};
use crate::components::virtual_list::VirtualList;
use crate::dashboard::common::{
    lookup_message, seed_message_states, IconKind, LucideIcon, MessageState, MessageTag, FOLDERS,
    TABS,
};
use crate::theme::DarkModeToggle;

#[component]
pub fn EmailClient() -> Element {
    let mut messages = use_signal(seed_message_states);
    let active_folder = use_signal(|| String::from("inbox"));
    let mut active_tab = use_signal(|| String::from("all"));
    let mut search_query = use_signal(String::new);
    let mut selected_tags = use_signal(Vec::<MessageTag>::new);
    let selected_id = use_signal(|| String::from("m1#0"));
    let mut read_open = use_signal(|| false);
    let mut reply_draft = use_signal(String::new);
    let active_folder_id = active_folder.read().clone();
    let active_tab_id = active_tab.read().clone();
    let active_search_query = search_query.read().clone();
    let active_selected_tags = selected_tags.read().clone();

    let folder_label: String = FOLDERS
        .iter()
        .find(|f| f.id == active_folder_id.as_str())
        .map(|f| f.label.to_string())
        .unwrap_or_else(|| "Inbox".to_string());

    let messages_snapshot = messages.read().clone();

    let visible_messages = filtered_messages(
        &messages_snapshot,
        active_folder_id.as_str(),
        active_tab_id.as_str(),
        active_search_query.as_str(),
        &active_selected_tags,
    );

    let selected_uid_read = selected_id.read().clone();
    let selected: MessageState = visible_messages
        .iter()
        .find(|s| s.uid == selected_uid_read)
        .or_else(|| visible_messages.first())
        .cloned()
        .unwrap_or_else(|| {
            messages_snapshot
                .first()
                .cloned()
                .expect("seed_message_states is non-empty")
        });
    let selected_static = lookup_message(selected.source_id);
    let selected_uid = selected.uid.clone();

    let rows = flatten_rows(&visible_messages);
    let row_count = rows.len();
    let total_count = visible_messages.len();
    let selected_index = visible_messages
        .iter()
        .position(|s| s.uid == selected.uid)
        .map(|i| i + 1)
        .unwrap_or(1);
    let counter = format!("{} of {}", selected_index, total_count);

    let archive_selected = move |_| {
        let uid = selected_id.read().clone();
        let mut msgs = messages.write();
        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid) {
            entry.folder_id = "archive".to_string();
            entry.unread = false;
        }
        drop(msgs);
        read_open.set(false);
    };
    let snooze_selected = move |_| {
        let uid = selected_id.read().clone();
        let mut msgs = messages.write();
        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid) {
            entry.snoozed = true;
        }
        drop(msgs);
        read_open.set(false);
    };
    let delete_selected = move |_| {
        let uid = selected_id.read().clone();
        let mut msgs = messages.write();
        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid) {
            entry.folder_id = "trash".to_string();
            entry.unread = false;
        }
        drop(msgs);
        read_open.set(false);
    };
    let toggle_flag_selected = move |_| {
        let uid = selected_id.read().clone();
        let mut msgs = messages.write();
        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid) {
            entry.flagged = !entry.flagged;
        }
    };
    let toggle_star_selected = move |_| {
        let uid = selected_id.read().clone();
        let mut msgs = messages.write();
        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid) {
            entry.starred = !entry.starred;
        }
    };
    let mut toggle_unread_selected = move |_| {
        let uid = selected_id.read().clone();
        let mut msgs = messages.write();
        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid) {
            entry.unread = !entry.unread;
        }
    };
    let mut move_to_inbox_selected = move |_| {
        let uid = selected_id.read().clone();
        let mut msgs = messages.write();
        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid) {
            entry.folder_id = "inbox".to_string();
            entry.snoozed = false;
        }
        drop(msgs);
        read_open.set(false);
    };
    let mut move_to_trash_selected = move |_| {
        let uid = selected_id.read().clone();
        let mut msgs = messages.write();
        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid) {
            entry.folder_id = "trash".to_string();
        }
        drop(msgs);
        read_open.set(false);
    };
    let remove_tag_from_selected = move |tag: MessageTag| {
        let uid = selected_id.read().clone();
        let mut msgs = messages.write();
        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid) {
            entry.tags.retain(|t| *t != tag);
        }
    };
    let set_selected_tags = move |new_tags: Vec<MessageTag>| {
        let uid = selected_id.read().clone();
        let mut msgs = messages.write();
        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid) {
            entry.tags = new_tags;
        }
    };

    let folder_counts: Vec<Option<u32>> = FOLDERS
        .iter()
        .map(|f| Some(folder_count(&messages_snapshot, f.id)))
        .collect();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./email_client.css") }

        SidebarProvider {
            Sidebar {
                variant: SidebarVariant::Sidebar,
                collapsible: SidebarCollapsible::Icon,

                SidebarHeader {
                    SidebarMenu { SidebarMenuItem {
                        SidebarMenuButton {
                            size: SidebarMenuButtonSize::Lg,
                            tooltip: rsx! { "Mail · you@yourcompany.com" },
                            Avatar {
                                size: AvatarImageSize::Small,
                                shape: AvatarShape::Rounded,
                                AvatarImage {
                                    src: "{AVATAR_PROFILE_OPTIONS[2].src}",
                                    alt: "Mail",
                                }
                                AvatarFallback { "M" }
                            }
                            div { class: "dx-sidebar-info-block",
                                span { class: "dx-sidebar-info-title", "Mail" }
                                span { class: "dx-sidebar-info-subtitle", "you@yourcompany.com" }
                            }
                        }
                    } }
                }

                SidebarContent {
                    SidebarGroup {
                        SidebarMenu { SidebarMenuItem {
                            SidebarMenuButton {
                                class: "ec-compose",
                                tooltip: rsx! { "Compose (C)" },
                                LucideIcon { kind: IconKind::Pen }
                                span { "Compose" }
                            }
                        } }
                    }

                    SidebarGroup {
                        SidebarGroupLabel { "Folders" }
                        SidebarMenu {
                            for (idx, f) in FOLDERS.iter().enumerate() {
                                FolderItem {
                                    key: "{f.id}",
                                    folder_id: f.id,
                                    label: f.label,
                                    icon: f.icon,
                                    count: folder_counts.get(idx).copied().flatten(),
                                    active_folder,
                                    read_open,
                                }
                            }
                        }
                    }
                }

                SidebarFooter {
                    SidebarMenu { SidebarMenuItem {
                        SidebarMenuButton {
                            size: SidebarMenuButtonSize::Lg,
                            tooltip: rsx! { "You" },
                            Avatar {
                                size: AvatarImageSize::Small,
                                shape: AvatarShape::Rounded,
                                AvatarImage {
                                    src: "{AVATAR_PROFILE_OPTIONS[0].src}",
                                    alt: "You",
                                }
                                AvatarFallback { "Y" }
                            }
                            div { class: "dx-sidebar-info-block",
                                span { class: "dx-sidebar-info-title", "You" }
                                span { class: "dx-sidebar-info-subtitle", "you@yourcompany.com" }
                            }
                        }
                    } }
                }

                SidebarRail {}
            }

            SidebarInset {
                header { class: "ec-topbar",
                    SidebarTrigger {}
                    Separator { horizontal: false, decorative: true }
                    h1 { class: "ec-title", {folder_label} }
                    Input {
                        r#type: "search",
                        "aria-label": "Search mail",
                        name: "mail-search",
                        value: search_query,
                        oninput: move |event: FormEvent| {
                            search_query.set(event.value());
                            read_open.set(false);
                        },
                        placeholder: "Search mail, people, attachments…",
                    }
                    DarkModeToggle {}
                }

                div { class: if read_open() { "ec-main ec-reading" } else { "ec-main" },
                    section { class: "ec-list-pane",
                        div { class: "ec-list-toolbar",
                            Tabs {
                                default_value: "all".to_string(),
                                horizontal: true,
                                on_value_change: move |v: String| {
                                    active_tab.set(v);
                                    read_open.set(false);
                                },
                                TabList {
                                    for (idx, tab) in TABS.iter().enumerate() {
                                        TabTrigger {
                                            key: "{tab.id}",
                                            value: tab.id.to_string(),
                                            index: idx,
                                            {tab.label}
                                            span { class: "ec-muted", " {tab_count(&messages_snapshot, active_folder_id.as_str(), tab.id, active_search_query.as_str(), &active_selected_tags)}" }
                                        }
                                    }
                                }
                            }
                            SelectMulti::<MessageTag> {
                                values: Some(active_selected_tags.clone()),
                                default_values: vec![],
                                on_values_change: move |values| {
                                    selected_tags.set(values);
                                    read_open.set(false);
                                },
                                SelectTrigger {
                                    class: "ec-filter-trigger",
                                    aria_label: "Filter by tag",
                                    LucideIcon { kind: IconKind::Filter }
                                    if !active_selected_tags.is_empty() {
                                        span { class: "ec-filter-count", "{active_selected_tags.len()}" }
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
                            estimate_size: {
                                let rows = rows.clone();
                                move |idx: usize| match &rows[idx] {
                                    ListRow::DayHeader(_) => 34,
                                    ListRow::Message(state) => estimate_message_row_height(state),
                                }
                            },
                            render_item: move |idx: usize| match &rows[idx] {
                                ListRow::DayHeader(day) => rsx! {
                                    div { class: "ec-day", {*day} }
                                },
                                ListRow::Message(state) => {
                                    rsx! {
                                        MessageRow {
                                            key: "{state.uid}",
                                            state: state.clone(),
                                            selected_id,
                                            selected_uid: selected_uid.clone(),
                                            read_open,
                                            messages,
                                        }
                                    }
                                }
                            },
                        }
                    }

                    section { class: "ec-read-pane",
                        Toolbar { aria_label: "Message actions",
                            ToolbarGroup {
                                ToolbarButton { index: 0usize, on_click: move |_| read_open.set(false),
                                    LucideIcon { kind: IconKind::ArrowLeft }
                                }
                            }
                            ToolbarSeparator {}
                            ToolbarGroup {
                                ToolbarButton { index: 1usize, on_click: archive_selected,
                                    LucideIcon { kind: IconKind::Archive }
                                    " Archive"
                                }
                                ToolbarButton { index: 2usize, on_click: snooze_selected,
                                    LucideIcon { kind: IconKind::Snooze }
                                    " Snooze"
                                }
                                ToolbarButton { index: 3usize, on_click: delete_selected,
                                    LucideIcon { kind: IconKind::Trash }
                                    " Delete"
                                }
                            }
                            ToolbarSeparator {}
                            ToolbarGroup {
                                ToolbarButton { index: 4usize, on_click: toggle_flag_selected,
                                    if selected.flagged {
                                        LucideIcon { kind: IconKind::Flag }
                                        " Flagged"
                                    } else {
                                        LucideIcon { kind: IconKind::Flag }
                                        " Flag"
                                    }
                                }
                                ToolbarButton { index: 5usize, on_click: toggle_star_selected,
                                    if selected.starred {
                                        LucideIcon { kind: IconKind::StarFilled }
                                        " Starred"
                                    } else {
                                        LucideIcon { kind: IconKind::StarOutline }
                                        " Star"
                                    }
                                }
                            }
                            div { class: "ec-toolbar-end",
                                span { class: "ec-muted", {counter} }
                                DropdownMenu { default_open: false,
                                    DropdownMenuTrigger {
                                        r#as: move |attrs: Vec<Attribute>| rsx! {
                                            ToolbarButton {
                                                index: 6usize,
                                                attributes: attrs,
                                                on_click: move |_| {},
                                                LucideIcon { kind: IconKind::More }
                                            }
                                        },
                                    }
                                    DropdownMenuContent {
                                        DropdownMenuItem::<&'static str> {
                                            value: "toggle-unread",
                                            index: 0usize,
                                            on_select: move |_| toggle_unread_selected(()),
                                            if selected.unread { "Mark as read" } else { "Mark as unread" }
                                        }
                                        DropdownMenuItem::<&'static str> {
                                            value: "move-to-inbox",
                                            index: 1usize,
                                            disabled: selected.folder_id == "inbox",
                                            on_select: move |_| move_to_inbox_selected(()),
                                            "Move to Inbox"
                                        }
                                        DropdownMenuItem::<&'static str> {
                                            value: "move-to-trash",
                                            index: 2usize,
                                            disabled: selected.folder_id == "trash",
                                            on_select: move |_| move_to_trash_selected(()),
                                            "Move to Trash"
                                        }
                                    }
                                }
                            }
                        }

                        article { class: "ec-read-body ec-thread",
                            Card { class: "ec-thread-hero",
                                CardHeader {
                                    div { class: "ec-thread-hero-main",
                                        div {
                                            CardTitle { {selected_static.subject} }
                                            CardDescription {
                                                div { class: "ec-thread-hero-meta",
                                                    span {
                                                        {format!(
                                                            "{} message{} in this thread",
                                                            selected_static.thread_count,
                                                            if selected_static.thread_count > 1 { "s" } else { "" },
                                                        )}
                                                    }
                                                    for tag in selected.tags.iter() {
                                                        Button {
                                                            variant: ButtonVariant::Ghost,
                                                            key: "{tag.label()}",
                                                            r#type: "button",
                                                            class: "ec-tag-remove",
                                                            "aria-label": "Remove tag {tag.label()}",
                                                            onclick: {
                                                                let tag = *tag;
                                                                let mut remove_tag = remove_tag_from_selected;
                                                                move |_| remove_tag(tag)
                                                            },
                                                            Badge {
                                                                variant: BadgeVariant::Secondary,
                                                                "{tag.label()} ×"
                                                            }
                                                        }
                                                    }
                                                    SelectMulti::<MessageTag> {
                                                        key: "{selected.uid}-tagedit",
                                                        values: Some(selected.tags.clone()),
                                                        default_values: selected.tags.clone(),
                                                        on_values_change: move |values: Vec<MessageTag>| {
                                                            let mut set_tags = set_selected_tags;
                                                            set_tags(values);
                                                        },
                                                        SelectTrigger {
                                                            class: "ec-tag-edit-trigger",
                                                            aria_label: "Add tag",
                                                            "+ Tag"
                                                        }
                                                        SelectList {
                                                            class: "ec-filter-list",
                                                            aria_label: "Edit tags",
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
                                            }
                                        }
                                    }
                                }
                            }

                            Card {
                                class: if selected_static.thread_count > 1 { "ec-thread-msg" } else { "ec-thread-msg ec-thread-msg-current" },
                                CardContent { class: "ec-thread-msg-content",
                                    div { class: "ec-thread-msg-head",
                                        Avatar {
                                            size: AvatarImageSize::Small,
                                            shape: AvatarShape::Circle,
                                            AvatarImage {
                                                src: "{avatar_profile_for_key(selected_static.from_addr).src}",
                                                alt: "{selected_static.from}",
                                            }
                                            AvatarFallback { {selected_static.initials} }
                                        }
                                        div { class: "ec-thread-msg-meta",
                                            div { class: "ec-thread-msg-sender",
                                                span { class: "ec-thread-msg-name", {selected_static.from} }
                                                span { class: "ec-thread-msg-addr",
                                                    {selected_static.from_addr}
                                                }
                                            }
                                            span { class: "ec-thread-msg-time", {selected_static.full_time} }
                                        }
                                    }
                                    div { class: "ec-thread-msg-body",
                                        for (i, para) in selected_static.body.split("\n\n").enumerate() {
                                            p { key: "{i}", {para.to_string()} }
                                        }
                                    }
                                }
                            }

                            if selected_static.thread_count > 1 {
                                Card { class: "ec-thread-msg ec-thread-msg-current",
                                    CardContent { class: "ec-thread-msg-content",
                                        div { class: "ec-thread-msg-head",
                                            Avatar {
                                                size: AvatarImageSize::Small,
                                                shape: AvatarShape::Circle,
                                                AvatarImage {
                                                    src: "{AVATAR_PROFILE_OPTIONS[0].src}",
                                                    alt: "You",
                                                }
                                                AvatarFallback { "Y" }
                                            }
                                            div { class: "ec-thread-msg-meta",
                                                div { class: "ec-thread-msg-sender",
                                                    span { class: "ec-thread-msg-name", "You" }
                                                    span { class: "ec-thread-msg-addr", "to {selected_static.from}" }
                                                }
                                                span { class: "ec-thread-msg-time",
                                                    "earlier today"
                                                }
                                            }
                                        }
                                        div { class: "ec-thread-msg-body",
                                            p {
                                                "Thanks for sending this over — taking a look now and will circle back shortly."
                                            }
                                        }
                                    }
                                }
                            }

                            Card { class: "ec-thread-compose",
                                CardContent { class: "ec-thread-compose-content",
                                    div { class: "ec-thread-compose-row",
                                        Avatar {
                                            size: AvatarImageSize::Small,
                                            shape: AvatarShape::Circle,
                                            AvatarImage {
                                                src: "{AVATAR_PROFILE_OPTIONS[0].src}",
                                                alt: "You",
                                            }
                                            AvatarFallback { "Y" }
                                        }
                                        Textarea {
                                            placeholder: format!("Reply to {}…", selected_static.from),
                                            rows: "2",
                                            value: "{reply_draft}",
                                            oninput: move |event: FormEvent| reply_draft.set(event.value()),
                                        }
                                        div { class: "ec-thread-compose-actions",
                                            Button { variant: ButtonVariant::Secondary,
                                                LucideIcon { kind: IconKind::Paperclip, size: 14 }
                                            }
                                            Button {
                                                variant: ButtonVariant::Primary,
                                                disabled: reply_draft.read().trim().is_empty(),
                                                LucideIcon { kind: IconKind::Send, size: 14 }
                                                "Send"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FolderItem(
    folder_id: &'static str,
    label: &'static str,
    icon: IconKind,
    count: Option<u32>,
    mut active_folder: Signal<String>,
    mut read_open: Signal<bool>,
) -> Element {
    let is_active = active_folder.read().as_str() == folder_id;

    rsx! {
        SidebarMenuItem {
            SidebarMenuButton {
                is_active,
                tooltip: rsx! { {label} },
                as: move |attrs: Vec<Attribute>| rsx! {
                    button {
                        r#type: "button",
                        onclick: move |_| {
                            active_folder.set(folder_id.to_string());
                            read_open.set(false);
                        },
                        ..attrs,
                        LucideIcon { kind: icon }
                        span { {label} }
                    }
                },
            }
            if let Some(c) = count {
                SidebarMenuBadge { {format!("{}", c)} }
            }
        }
    }
}

#[derive(Clone)]
enum ListRow {
    DayHeader(&'static str),
    Message(MessageState),
}

fn flatten_rows(messages: &[MessageState]) -> Vec<ListRow> {
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

fn message_matches_folder(state: &MessageState, folder_id: &str) -> bool {
    if state.snoozed {
        return false;
    }
    match folder_id {
        "starred" => state.starred,
        id => state.folder_id == id,
    }
}

fn message_matches_tab(state: &MessageState, tab_id: &str) -> bool {
    match tab_id {
        "unread" => state.unread,
        "flagged" => state.flagged,
        _ => true,
    }
}

fn message_matches_search(state: &MessageState, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return true;
    }
    let m = lookup_message(state.source_id);
    m.from.to_lowercase().contains(&query)
        || m.from_addr.to_lowercase().contains(&query)
        || m.subject.to_lowercase().contains(&query)
        || m.snippet.to_lowercase().contains(&query)
        || m.body.to_lowercase().contains(&query)
        || state.tags.iter().any(|tag| tag.label().contains(&query))
        || (m.has_attachment && "attachment".contains(&query))
}

fn message_matches_selected_tags(state: &MessageState, selected_tags: &[MessageTag]) -> bool {
    selected_tags
        .iter()
        .all(|s| state.tags.iter().any(|tag| tag == s))
}

fn message_matches_filters(
    state: &MessageState,
    folder_id: &str,
    tab_id: &str,
    query: &str,
    selected_tags: &[MessageTag],
) -> bool {
    message_matches_folder(state, folder_id)
        && message_matches_tab(state, tab_id)
        && message_matches_search(state, query)
        && message_matches_selected_tags(state, selected_tags)
}

fn filtered_messages(
    messages: &[MessageState],
    folder_id: &str,
    tab_id: &str,
    query: &str,
    selected_tags: &[MessageTag],
) -> Vec<MessageState> {
    messages
        .iter()
        .filter(|s| message_matches_filters(s, folder_id, tab_id, query, selected_tags))
        .cloned()
        .collect()
}

fn folder_count(messages: &[MessageState], folder_id: &str) -> u32 {
    messages
        .iter()
        .filter(|s| message_matches_folder(s, folder_id))
        .count() as u32
}

fn tab_count(
    messages: &[MessageState],
    folder_id: &str,
    tab_id: &str,
    query: &str,
    selected_tags: &[MessageTag],
) -> u32 {
    messages
        .iter()
        .filter(|s| message_matches_filters(s, folder_id, tab_id, query, selected_tags))
        .count() as u32
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

fn avatar_profile_for_key(key: &str) -> &'static AvatarProfile {
    let index = key.bytes().fold(0usize, |hash, byte| {
        hash.wrapping_mul(31).wrapping_add(byte as usize)
    }) % AVATAR_PROFILE_OPTIONS.len();

    &AVATAR_PROFILE_OPTIONS[index]
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
    let uid_for_key = state.uid.clone();
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
            onkeydown: move |event: Event<KeyboardData>| {
                let key = event.key();
                if key == Key::Enter || key == Key::Character(" ".to_string()) {
                    event.prevent_default();
                    selected_id.set(uid_for_key.clone());
                    read_open.set(true);
                }
            },
            "aria-selected": if is_selected { "true" } else { "false" },
            "data-selected": if is_selected { "true" } else { "false" },

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
                    onkeydown: move |e: Event<KeyboardData>| e.stop_propagation(),
                    onclick: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        let mut msgs = messages.write();
                        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid_for_trash) {
                            entry.folder_id = "trash".to_string();
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
                    "data-active": if starred { "true" } else { "false" },
                    aria_label: if starred { "Unstar message" } else { "Star message" },
                    onkeydown: move |e: Event<KeyboardData>| e.stop_propagation(),
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
