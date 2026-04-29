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
    lookup_message, FolderId, IconKind, LucideIcon, MessageState, MessageStateStoreExt, MessageTag,
    AVATAR_PROFILE_OPTIONS,
};

use super::avatars::avatar_profile_for_key;
use super::state::{EmailClientState, EmailClientStateStoreExt, EmailClientStateStoreImplExt};

#[component]
pub(super) fn ReadPane(
    mut state: Store<EmailClientState>,
    selected_uid: ReadSignal<String>,
    total_count: ReadSignal<usize>,
    selected_index: ReadSignal<usize>,
) -> Element {
    let mut reply_draft = use_signal(String::new);
    let selected_uid_value = selected_uid.read().clone();
    let Some(selected) = state.messages().get(selected_uid_value.clone()) else {
        return rsx! {};
    };
    let selected: Store<MessageState> = selected.into();
    let selected_static = lookup_message(selected.source_id().cloned());
    let selected_tags = selected.tags().cloned();
    let selected_folder = selected.folder_id().cloned();
    let selected_unread = selected.unread().cloned();
    let selected_starred = selected.starred().cloned();
    let selected_flagged = selected.flagged().cloned();
    let counter = format!("{} of {}", selected_index.read(), total_count.read());

    let archive_uid = selected_uid_value.clone();
    let archive_selected = move |_| {
        state.archive_message(archive_uid.clone());
    };
    let snooze_uid = selected_uid_value.clone();
    let snooze_selected = move |_| {
        state.snooze_message(snooze_uid.clone());
    };
    let delete_uid = selected_uid_value.clone();
    let delete_selected = move |_| {
        state.delete_message(delete_uid.clone());
    };
    let flag_uid = selected_uid_value.clone();
    let toggle_flag_selected = move |_| {
        state.toggle_message_flag(flag_uid.clone());
    };
    let star_uid = selected_uid_value.clone();
    let toggle_star_selected = move |_| {
        state.toggle_message_star(star_uid.clone());
    };
    let unread_uid = selected_uid_value.clone();
    let mut toggle_unread_selected = move |_| {
        state.toggle_message_unread(unread_uid.clone());
    };
    let inbox_uid = selected_uid_value.clone();
    let mut move_to_inbox_selected = move |_| {
        state.move_message_to_inbox(inbox_uid.clone());
    };
    let trash_uid = selected_uid_value.clone();
    let mut move_to_trash_selected = move |_| {
        state.move_message_to_trash(trash_uid.clone());
    };
    let tag_edit_uid = selected_uid_value.clone();

    rsx! {
        section { class: "ec-read-pane",
            Toolbar { aria_label: "Message actions",
                ToolbarGroup {
                    ToolbarButton { index: 0usize, on_click: move |_| state.close_read_pane(),
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
                        if selected_flagged {
                            LucideIcon { kind: IconKind::Flag }
                            " Flagged"
                        } else {
                            LucideIcon { kind: IconKind::Flag }
                            " Flag"
                        }
                    }
                    ToolbarButton { index: 5usize, on_click: toggle_star_selected,
                        if selected_starred {
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
                                if selected_unread { "Mark as read" } else { "Mark as unread" }
                            }
                            DropdownMenuItem::<&'static str> {
                                value: "move-to-inbox",
                                index: 1usize,
                                disabled: selected_folder == FolderId::Inbox,
                                on_select: move |_| move_to_inbox_selected(()),
                                "Move to Inbox"
                            }
                            DropdownMenuItem::<&'static str> {
                                value: "move-to-trash",
                                index: 2usize,
                                disabled: selected_folder == FolderId::Trash,
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
                                        for tag in selected_tags.iter() {
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
                                                        state.remove_message_tag(uid.clone(), tag);
                                                    }
                                                },
                                                Badge {
                                                    variant: BadgeVariant::Secondary,
                                                    "{tag.label()} ×"
                                                }
                                            }
                                        }
                                        SelectMulti::<MessageTag> {
                                            key: "{selected_uid_value}-tagedit",
                                            values: Some(selected_tags.clone()),
                                            default_values: selected_tags.clone(),
                                            on_values_change: move |values: Vec<MessageTag>| {
                                                state.set_message_tags(tag_edit_uid.clone(), values);
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
