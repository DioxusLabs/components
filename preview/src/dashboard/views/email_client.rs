use dioxus::prelude::*;

use crate::components::avatar::{Avatar, AvatarFallback, AvatarImageSize, AvatarShape};
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::input::Input;
use crate::components::item::{
    Item, ItemContent, ItemDescription, ItemMedia, ItemMediaVariant, ItemTitle,
};
use crate::components::separator::Separator;
use crate::components::textarea::Textarea;
use crate::components::virtual_list::VirtualList;
use crate::components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel,
    SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuBadge, SidebarMenuButton,
    SidebarMenuButtonSize, SidebarMenuItem, SidebarProvider, SidebarRail, SidebarTrigger,
    SidebarVariant,
};
use crate::components::tabs::component::{TabList, TabTrigger, Tabs};
use crate::components::toolbar::component::{Toolbar, ToolbarButton, ToolbarGroup, ToolbarSeparator};
use crate::dashboard::common::{IconKind, LucideIcon, Message, FOLDERS, MESSAGES, TABS};

const STYLE: &str = include_str!("email_client.css");

#[component]
pub fn EmailClient() -> Element {
    let active_folder = use_signal(|| String::from("inbox"));
    let mut active_tab = use_signal(|| String::from("all"));
    let selected_id = use_signal(|| String::from("m3"));

    let folder_label: String = FOLDERS
        .iter()
        .find(|f| f.id == active_folder.read().as_str())
        .map(|f| f.label.to_string())
        .unwrap_or_else(|| "Inbox".to_string());

    let selected: Message = MESSAGES
        .iter()
        .find(|m| m.id == selected_id.read().as_str())
        .cloned()
        .unwrap_or_else(|| MESSAGES[0].clone());

    let rows = flatten_rows();
    let row_count = rows.len();
    let total_count = MESSAGES.len();
    let selected_index = MESSAGES
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
                            tooltip: rsx! { "Mail · dan@yourcompany.com" },
                            Avatar {
                                size: AvatarImageSize::Small,
                                shape: AvatarShape::Rounded,
                                AvatarFallback { "M" }
                            }
                            div { class: "dx-sidebar-info-block",
                                span { class: "dx-sidebar-info-title", "Mail" }
                                span { class: "dx-sidebar-info-subtitle", "dan@yourcompany.com" }
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
                                    count: f.count,
                                    active_folder,
                                }
                            }
                        }
                    }
                }

                SidebarFooter {
                    SidebarMenu { SidebarMenuItem {
                        SidebarMenuButton {
                            size: SidebarMenuButtonSize::Lg,
                            tooltip: rsx! { "Dan Kowalski" },
                            Avatar {
                                size: AvatarImageSize::Small,
                                shape: AvatarShape::Rounded,
                                AvatarFallback { "DK" }
                            }
                            div { class: "dx-sidebar-info-block",
                                span { class: "dx-sidebar-info-title", "Dan Kowalski" }
                                span { class: "dx-sidebar-info-subtitle", "dan@yourcompany.com" }
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
                        placeholder: "Search mail, people, attachments…",
                    }
                    Button { variant: ButtonVariant::Ghost,
                        LucideIcon { kind: IconKind::Refresh }
                    }
                    Button { variant: ButtonVariant::Ghost,
                        LucideIcon { kind: IconKind::Filter }
                    }
                }

                div { class: "ec-main",
                    section { class: "ec-list-pane",
                        Tabs {
                            default_value: "all".to_string(),
                            horizontal: true,
                            on_value_change: move |v: String| active_tab.set(v),
                            TabList {
                                for (idx, tab) in TABS.iter().enumerate() {
                                    TabTrigger {
                                        key: "{tab.id}",
                                        value: tab.id.to_string(),
                                        index: idx,
                                        {tab.label}
                                        span { class: "ec-muted", " {tab.count}" }
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
                                    ListRow::Message(mid) => MESSAGES
                                        .iter()
                                        .find(|m| m.id == mid)
                                        .map(estimate_message_row_height)
                                        .unwrap_or(132),
                                }
                            },
                            render_item: move |idx: usize| match rows[idx] {
                                ListRow::DayHeader(day) => rsx! {
                                    div { class: "ec-day", {day} }
                                },
                                ListRow::Message(mid) => {
                                    let msg = MESSAGES.iter().find(|m| m.id == mid).cloned().unwrap();
                                    rsx! { MessageRow { msg, selected_id } }
                                }
                            },
                        }
                    }

                    section { class: "ec-read-pane",
                        Toolbar { aria_label: "Message actions",
                            ToolbarGroup {
                                ToolbarButton { index: 0usize, on_click: move |_| {},
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
                                                            key: "{tag}",
                                                            variant: BadgeVariant::Secondary,
                                                            {*tag}
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        div { class: "ec-thread-hero-actions",
                                            Button { variant: ButtonVariant::Ghost,
                                                LucideIcon { kind: IconKind::Reply, size: 14 }
                                                "Reply"
                                            }
                                            Button { variant: ButtonVariant::Ghost,
                                                LucideIcon { kind: IconKind::Forward, size: 14 }
                                                "Forward"
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
                                                AvatarFallback { "DK" }
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
                                            AvatarFallback { "DK" }
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

                footer { class: "ec-status",
                    span { "● Connected · synced 11:43" }
                    span { class: "ec-status-shortcuts",
                        "J/K navigate · ⏎ open · E archive · R reply · C compose · ? help"
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
                        onclick: move |_| active_folder.set(folder_id.to_string()),
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
    Message(&'static str),
}

fn flatten_rows() -> Vec<ListRow> {
    let mut out = Vec::with_capacity(MESSAGES.len() + 4);
    let mut last_day: Option<&'static str> = None;
    for m in MESSAGES.iter() {
        if last_day != Some(m.day) {
            out.push(ListRow::DayHeader(m.day));
            last_day = Some(m.day);
        }
        out.push(ListRow::Message(m.id));
    }
    out
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

#[component]
fn MessageRow(msg: Message, selected_id: Signal<String>) -> Element {
    let mid = msg.id;
    let is_selected = selected_id.read().as_str() == mid;
    let mut classes = String::from("ec-row");
    if msg.unread {
        classes.push_str(" ec-unread");
    }

    rsx! {
        Item {
            class: classes,
            onclick: move |_| selected_id.set(mid.to_string()),
            "aria-selected": if is_selected { "true" } else { "false" },
            "data-selected": if is_selected { "true" } else { "false" },

            ItemMedia { variant: ItemMediaVariant::Icon,
                if msg.starred {
                    LucideIcon { kind: IconKind::StarFilled, size: 14 }
                } else if msg.unread {
                    span { class: "ec-dot" }
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
                            span { key: "{tag}",
                                if i > 0 { " · " }
                                {*tag}
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
