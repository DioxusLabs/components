use dioxus::prelude::*;

use crate::components::avatar::{
    Avatar, AvatarFallback, AvatarImage, AvatarImageSize, AvatarShape,
};
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::dropdown_menu::component::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::select::{
    SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectMulti, SelectOption,
    SelectTrigger,
};
use crate::components::textarea::Textarea;
use crate::components::toolbar::component::{
    Toolbar, ToolbarButton, ToolbarGroup, ToolbarSeparator,
};
use crate::dashboard::common::{
    lookup_message, FolderId, IconKind, LucideIcon, MessageState, MessageTag,
    AVATAR_PROFILE_OPTIONS,
};

use super::avatars::avatar_profile_for_key;

#[component]
pub(super) fn ReadPane(
    selected: ReadSignal<MessageState>,
    total_count: ReadSignal<usize>,
    selected_index: ReadSignal<usize>,
    mut messages: Signal<Vec<MessageState>>,
    selected_id: Signal<String>,
    mut read_open: Signal<bool>,
) -> Element {
    let mut reply_draft = use_signal(String::new);
    let selected_value = selected.read().clone();
    let selected_static = lookup_message(selected_value.source_id);
    let counter = format!("{} of {}", selected_index.read(), total_count.read());

    let archive_selected = move |_| {
        let uid = selected_id.read().clone();
        let mut msgs = messages.write();
        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid) {
            entry.folder_id = FolderId::Archive;
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
            entry.folder_id = FolderId::Trash;
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
            entry.folder_id = FolderId::Inbox;
            entry.snoozed = false;
        }
        drop(msgs);
        read_open.set(false);
    };
    let mut move_to_trash_selected = move |_| {
        let uid = selected_id.read().clone();
        let mut msgs = messages.write();
        if let Some(entry) = msgs.iter_mut().find(|s| s.uid == uid) {
            entry.folder_id = FolderId::Trash;
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

    rsx! {
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
                        if selected_value.flagged {
                            LucideIcon { kind: IconKind::Flag }
                            " Flagged"
                        } else {
                            LucideIcon { kind: IconKind::Flag }
                            " Flag"
                        }
                    }
                    ToolbarButton { index: 5usize, on_click: toggle_star_selected,
                        if selected_value.starred {
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
                                if selected_value.unread { "Mark as read" } else { "Mark as unread" }
                            }
                            DropdownMenuItem::<&'static str> {
                                value: "move-to-inbox",
                                index: 1usize,
                                disabled: selected_value.folder_id == FolderId::Inbox,
                                on_select: move |_| move_to_inbox_selected(()),
                                "Move to Inbox"
                            }
                            DropdownMenuItem::<&'static str> {
                                value: "move-to-trash",
                                index: 2usize,
                                disabled: selected_value.folder_id == FolderId::Trash,
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
                                        for tag in selected_value.tags.iter() {
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
                                            key: "{selected_value.uid}-tagedit",
                                            values: Some(selected_value.tags.clone()),
                                            default_values: selected_value.tags.clone(),
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
