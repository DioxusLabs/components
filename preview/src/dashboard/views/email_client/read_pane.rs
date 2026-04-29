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
    lookup_message, FolderId, IconKind, LucideIcon, MessageTag, AVATAR_PROFILE_OPTIONS,
};

use super::avatars::avatar_profile_for_key;
use super::state::{close_read_pane, message_snapshot, update_message, EmailClientState};

#[component]
pub(super) fn ReadPane(
    state: Store<EmailClientState>,
    selected_uid: ReadSignal<String>,
    total_count: ReadSignal<usize>,
    selected_index: ReadSignal<usize>,
) -> Element {
    let mut reply_draft = use_signal(String::new);
    let selected_uid_value = selected_uid.read().clone();
    let selected_value =
        message_snapshot(state, &selected_uid_value).expect("selected message should exist");
    let selected_static = lookup_message(selected_value.source_id);
    let counter = format!("{} of {}", selected_index.read(), total_count.read());

    let archive_uid = selected_uid_value.clone();
    let archive_selected = move |_| {
        update_message(state, archive_uid.clone(), |entry| {
            entry.folder_id = FolderId::Archive;
            entry.unread = false;
        });
        close_read_pane(state);
    };
    let snooze_uid = selected_uid_value.clone();
    let snooze_selected = move |_| {
        update_message(state, snooze_uid.clone(), |entry| {
            entry.snoozed = true;
        });
        close_read_pane(state);
    };
    let delete_uid = selected_uid_value.clone();
    let delete_selected = move |_| {
        update_message(state, delete_uid.clone(), |entry| {
            entry.folder_id = FolderId::Trash;
            entry.unread = false;
        });
        close_read_pane(state);
    };
    let flag_uid = selected_uid_value.clone();
    let toggle_flag_selected = move |_| {
        update_message(state, flag_uid.clone(), |entry| {
            entry.flagged = !entry.flagged;
        });
    };
    let star_uid = selected_uid_value.clone();
    let toggle_star_selected = move |_| {
        update_message(state, star_uid.clone(), |entry| {
            entry.starred = !entry.starred;
        });
    };
    let unread_uid = selected_uid_value.clone();
    let toggle_unread_selected = move |_| {
        update_message(state, unread_uid.clone(), |entry| {
            entry.unread = !entry.unread;
        });
    };
    let inbox_uid = selected_uid_value.clone();
    let move_to_inbox_selected = move |_| {
        update_message(state, inbox_uid.clone(), |entry| {
            entry.folder_id = FolderId::Inbox;
            entry.snoozed = false;
        });
        close_read_pane(state);
    };
    let trash_uid = selected_uid_value.clone();
    let move_to_trash_selected = move |_| {
        update_message(state, trash_uid.clone(), |entry| {
            entry.folder_id = FolderId::Trash;
        });
        close_read_pane(state);
    };

    rsx! {
        section { class: "ec-read-pane",
            Toolbar { aria_label: "Message actions",
                ToolbarGroup {
                    ToolbarButton { index: 0usize, on_click: move |_| close_read_pane(state),
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
                                                    let uid = selected_uid_value.clone();
                                                    move |_| {
                                                        update_message(state, uid.clone(), |entry| {
                                                            entry.tags.retain(|t| *t != tag);
                                                        });
                                                    }
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
                                                update_message(state, selected_uid_value.clone(), |entry| {
                                                    entry.tags = values;
                                                });
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
