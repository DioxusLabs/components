use dioxus::prelude::*;

use crate::components::avatar::{
    Avatar, AvatarFallback, AvatarImage, AvatarImageSize, AvatarProfile, AvatarShape,
    AVATAR_PROFILE_OPTIONS,
};
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::input::Input;
use crate::components::item::{
    Item, ItemContent, ItemDescription, ItemMedia, ItemMediaVariant, ItemTitle,
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
    IconKind, LucideIcon, Message, MessageTag, DEFAULT_MESSAGE_FOLDER_ID, FOLDERS, MESSAGES,
    MESSAGE_PROPERTIES, TABS,
};

const STYLE: &str = include_str!("email_client.css");
const EMAIL_REPEAT_COUNT: usize = 5;

#[component]
pub fn EmailClient() -> Element {
    let active_folder = use_signal(|| String::from("inbox"));
    let mut active_tab = use_signal(|| String::from("all"));
    let mut search_query = use_signal(String::new);
    let mut selected_tags = use_signal(Vec::<MessageTag>::new);
    let selected_id = use_signal(|| String::from("m3"));
    let mut read_open = use_signal(|| false);
    let active_folder_id = active_folder.read().clone();
    let active_tab_id = active_tab.read().clone();
    let active_search_query = search_query.read().clone();
    let active_selected_tags = selected_tags.read().clone();

    let folder_label: String = FOLDERS
        .iter()
        .find(|f| f.id == active_folder_id.as_str())
        .map(|f| f.label.to_string())
        .unwrap_or_else(|| "Inbox".to_string());

    let visible_messages = filtered_messages(
        active_folder_id.as_str(),
        active_tab_id.as_str(),
        active_search_query.as_str(),
        &active_selected_tags,
    );

    let selected: Message = message_pool()
        .find(|m| {
            m.id == selected_id.read().as_str()
                && message_matches_filters(
                    m,
                    active_folder_id.as_str(),
                    active_tab_id.as_str(),
                    active_search_query.as_str(),
                    &active_selected_tags,
                )
        })
        .or_else(|| visible_messages.first().copied())
        .cloned()
        .unwrap_or_else(|| MESSAGES[0].clone());

    let rows = flatten_rows(&visible_messages);
    let row_count = rows.len();
    let total_count = visible_messages.len();
    let selected_index = visible_messages
        .iter()
        .position(|m| m.id == selected.id)
        .map(|i| i + 1)
        .unwrap_or(1);
    let counter = format!("{} of {}", selected_index, total_count);

    rsx! {
        document::Style { {STYLE} }

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
                            for f in FOLDERS.iter() {
                                FolderItem {
                                    key: "{f.id}",
                                    folder_id: f.id,
                                    label: f.label,
                                    icon: f.icon,
                                    count: folder_count(f.id),
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
                    h1 { class: "ec-title",
                        {folder_label}
                        span { class: "ec-muted", " · {total_count}" }
                    }
                    Input {
                        class: "ec-search",
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
                    Button { variant: ButtonVariant::Ghost,
                        LucideIcon { kind: IconKind::Refresh }
                    }
                    SelectMulti::<MessageTag> {
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

                div { class: if read_open() { "ec-main ec-reading" } else { "ec-main" },
                    section { class: "ec-list-pane",
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
                                        span { class: "ec-muted", " {tab_count(active_folder_id.as_str(), tab.id, active_search_query.as_str(), &active_selected_tags)}" }
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
                                move |idx: usize| match rows[idx] {
                                    ListRow::DayHeader(_) => 34,
                                    ListRow::Message(msg) => estimate_message_row_height(msg),
                                }
                            },
                            render_item: move |idx: usize| match rows[idx] {
                                ListRow::DayHeader(day) => rsx! {
                                    div { class: "ec-day", {day} }
                                },
                                ListRow::Message(msg) => {
                                    rsx! {
                                        MessageRow {
                                            msg: msg.clone(),
                                            selected_id,
                                            selected_message_id: selected.id,
                                            read_open,
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
                                ToolbarButton { index: 1usize, on_click: move |_| {},
                                    LucideIcon { kind: IconKind::Archive }
                                    " Archive"
                                }
                                ToolbarButton { index: 2usize, on_click: move |_| {},
                                    LucideIcon { kind: IconKind::Snooze }
                                    " Snooze"
                                }
                                ToolbarButton { index: 3usize, on_click: move |_| {},
                                    LucideIcon { kind: IconKind::Trash }
                                    " Delete"
                                }
                            }
                            ToolbarSeparator {}
                            ToolbarGroup {
                                ToolbarButton { index: 4usize, on_click: move |_| {},
                                    LucideIcon { kind: IconKind::Flag }
                                    " Flag"
                                }
                                ToolbarButton { index: 5usize, on_click: move |_| {},
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
                                ToolbarButton { index: 6usize, on_click: move |_| {},
                                    LucideIcon { kind: IconKind::More }
                                }
                            }
                        }

                        article { class: "ec-read-body ec-thread",
                            Card { class: "ec-thread-hero",
                                CardHeader {
                                    div { class: "ec-thread-hero-main",
                                        div {
                                            CardTitle { {selected.subject} }
                                            CardDescription {
                                                div { class: "ec-thread-hero-meta",
                                                    span {
                                                        {format!(
                                                            "{} message{} in this thread",
                                                            selected.thread_count,
                                                            if selected.thread_count > 1 { "s" } else { "" },
                                                        )}
                                                    }
                                                    for tag in selected.tags.iter() {
                                                        Badge {
                                                            key: "{tag.label()}",
                                                            variant: BadgeVariant::Secondary,
                                                            {tag.label()}
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if selected.thread_count > 1 {
                                Card { class: "ec-thread-msg",
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
                                                    span { class: "ec-thread-msg-addr", "to {selected.from}" }
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

                            Card { class: "ec-thread-msg ec-thread-msg-current",
                                CardContent { class: "ec-thread-msg-content",
                                    div { class: "ec-thread-msg-head",
                                        Avatar {
                                            size: AvatarImageSize::Small,
                                            shape: AvatarShape::Circle,
                                            AvatarImage {
                                                src: "{avatar_profile_for_key(selected.from_addr).src}",
                                                alt: "{selected.from}",
                                            }
                                            AvatarFallback { {selected.initials} }
                                        }
                                        div { class: "ec-thread-msg-meta",
                                            div { class: "ec-thread-msg-sender",
                                                span { class: "ec-thread-msg-name", {selected.from} }
                                                span { class: "ec-thread-msg-addr",
                                                    {selected.from_addr}
                                                }
                                            }
                                            span { class: "ec-thread-msg-time", {selected.full_time} }
                                        }
                                    }
                                    div { class: "ec-thread-msg-body",
                                        for (i, para) in selected.body.split("\n\n").enumerate() {
                                            p { key: "{i}", {para.to_string()} }
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
                                            placeholder: format!("Reply to {}…", selected.from),
                                            rows: "2",
                                        }
                                    }
                                    div { class: "ec-thread-compose-actions",
                                        Button { variant: ButtonVariant::Secondary,
                                            LucideIcon { kind: IconKind::Paperclip, size: 14 }
                                        }
                                        Button { variant: ButtonVariant::Primary,
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

fn message_pool() -> impl Iterator<Item = &'static Message> {
    MESSAGES
        .iter()
        .cycle()
        .take(MESSAGES.len() * EMAIL_REPEAT_COUNT)
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

#[derive(Clone, Copy)]
enum ListRow {
    DayHeader(&'static str),
    Message(&'static Message),
}

fn flatten_rows(messages: &[&'static Message]) -> Vec<ListRow> {
    let mut out = Vec::with_capacity(messages.len() + 4);
    let mut last_day: Option<&'static str> = None;
    for m in messages.iter() {
        if last_day != Some(m.day) {
            out.push(ListRow::DayHeader(m.day));
            last_day = Some(m.day);
        }
        out.push(ListRow::Message(m));
    }
    out
}

fn message_folder_id(msg: &Message) -> &'static str {
    MESSAGE_PROPERTIES
        .iter()
        .find(|properties| properties.message_id == msg.id)
        .map(|properties| properties.folder_id)
        .unwrap_or(DEFAULT_MESSAGE_FOLDER_ID)
}

fn message_matches_folder(msg: &Message, folder_id: &str) -> bool {
    match folder_id {
        "starred" => msg.starred,
        id => message_folder_id(msg) == id,
    }
}

fn message_matches_tab(msg: &Message, tab_id: &str) -> bool {
    match tab_id {
        "unread" => msg.unread,
        "flagged" => msg.starred,
        _ => true,
    }
}

fn message_matches_search(msg: &Message, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return true;
    }

    msg.from.to_lowercase().contains(&query)
        || msg.from_addr.to_lowercase().contains(&query)
        || msg.subject.to_lowercase().contains(&query)
        || msg.snippet.to_lowercase().contains(&query)
        || msg.body.to_lowercase().contains(&query)
        || msg.tags.iter().any(|tag| tag.label().contains(&query))
        || (msg.has_attachment && "attachment".contains(&query))
}

fn message_matches_selected_tags(msg: &Message, selected_tags: &[MessageTag]) -> bool {
    selected_tags.is_empty()
        || msg
            .tags
            .iter()
            .any(|tag| selected_tags.iter().any(|selected| selected == tag))
}

fn message_matches_filters(
    msg: &Message,
    folder_id: &str,
    tab_id: &str,
    query: &str,
    selected_tags: &[MessageTag],
) -> bool {
    message_matches_folder(msg, folder_id)
        && message_matches_tab(msg, tab_id)
        && message_matches_search(msg, query)
        && message_matches_selected_tags(msg, selected_tags)
}

fn filtered_messages(
    folder_id: &str,
    tab_id: &str,
    query: &str,
    selected_tags: &[MessageTag],
) -> Vec<&'static Message> {
    message_pool()
        .filter(|msg| message_matches_filters(msg, folder_id, tab_id, query, selected_tags))
        .collect()
}

fn folder_count(folder_id: &str) -> u32 {
    message_pool()
        .filter(|msg| message_matches_folder(msg, folder_id))
        .count() as u32
}

fn tab_count(folder_id: &str, tab_id: &str, query: &str, selected_tags: &[MessageTag]) -> u32 {
    message_pool()
        .filter(|msg| message_matches_filters(msg, folder_id, tab_id, query, selected_tags))
        .count() as u32
}

fn estimate_message_row_height(msg: &Message) -> u32 {
    let snippet_lines = if msg.snippet.len() > 78 { 2 } else { 1 };
    let has_meta_row = !msg.tags.is_empty() || msg.has_attachment;

    32  // vertical padding + borders + margins
        + 19 // sender/title row
        + 18 // subject row
        + 21 * snippet_lines
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
    msg: Message,
    selected_id: Signal<String>,
    selected_message_id: &'static str,
    mut read_open: Signal<bool>,
) -> Element {
    let mid = msg.id;
    let is_selected = selected_message_id == mid;
    let mut classes = String::from("ec-row");
    if msg.unread {
        classes.push_str(" ec-unread");
    }

    rsx! {
        Item {
            class: classes,
            onclick: move |_| {
                selected_id.set(mid.to_string());
                read_open.set(true);
            },
            "aria-selected": if is_selected { "true" } else { "false" },
            "data-selected": if is_selected { "true" } else { "false" },

            ItemMedia { variant: ItemMediaVariant::Icon,
                Avatar {
                    size: AvatarImageSize::Small,
                    shape: AvatarShape::Circle,
                    AvatarImage {
                        src: "{avatar_profile_for_key(msg.from_addr).src}",
                        alt: "{msg.from}",
                    }
                    AvatarFallback { {msg.initials} }
                }
            }
            ItemContent {
                ItemTitle {
                    span { {msg.from} }
                    span { class: "ec-muted ec-row-time", {msg.time} }
                }
                div {
                    {msg.subject}
                    if msg.thread_count > 1 {
                        span { class: "ec-muted", {format!(" ·{}", msg.thread_count)} }
                    }
                }
                ItemDescription { {msg.snippet} }
                if !msg.tags.is_empty() || msg.has_attachment {
                    div { class: "ec-muted ec-row-tags",
                        for (i, tag) in msg.tags.iter().enumerate() {
                            span { key: "{tag.label()}",
                                if i > 0 { " · " }
                                {tag.label()}
                            }
                        }
                        if msg.has_attachment {
                            LucideIcon { kind: IconKind::Paperclip, size: 12 }
                        }
                    }
                }
            }
        }
    }
}
