use dioxus::prelude::*;

use crate::components::input::Input;
use crate::components::sidebar::{SidebarInset, SidebarProvider, SidebarTrigger};
use crate::components::separator::Separator;
use crate::dashboard::common::{
    seed_message_states, FolderId, MessageState, MessageTag, TabId, FOLDERS,
};
use crate::theme::DarkModeToggle;

mod avatars;
mod filters;
mod list_pane;
mod read_pane;
mod sidebar;

use filters::filtered_messages;
use list_pane::{flatten_rows, ListPane};
use read_pane::ReadPane;
use sidebar::EmailSidebar;

#[component]
pub fn EmailClient() -> Element {
    let messages = use_signal(seed_message_states);
    let active_folder = use_signal(|| FolderId::Inbox);
    let active_tab = use_signal(|| TabId::All);
    let mut search_query = use_signal(String::new);
    let selected_tags = use_signal(Vec::<MessageTag>::new);
    let selected_id = use_signal(|| String::from("m1#0"));
    let mut read_open = use_signal(|| false);

    let active_folder_id = *active_folder.read();
    let active_tab_id = *active_tab.read();
    let active_search_query = search_query.read().clone();
    let active_selected_tags = selected_tags.read().clone();

    let folder_label: String = FOLDERS
        .iter()
        .find(|f| f.id == active_folder_id)
        .map(|f| f.label.to_string())
        .unwrap_or_else(|| "Inbox".to_string());

    let messages_snapshot = messages.read().clone();

    let visible_messages = filtered_messages(
        &messages_snapshot,
        active_folder_id,
        active_tab_id,
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

    let rows = flatten_rows(&visible_messages);
    let total_count = visible_messages.len();
    let selected_index = visible_messages
        .iter()
        .position(|s| s.uid == selected.uid)
        .map(|i| i + 1)
        .unwrap_or(1);

    let selected_uid = selected.uid.clone();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./email_client.css") }

        SidebarProvider {
            EmailSidebar {
                messages_snapshot: messages_snapshot.clone(),
                active_folder,
                read_open,
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
                    ListPane {
                        rows,
                        messages_snapshot: messages_snapshot.clone(),
                        active_folder_id,
                        active_search_query: active_search_query.clone(),
                        active_selected_tags: active_selected_tags.clone(),
                        messages,
                        selected_id,
                        selected_uid,
                        active_tab,
                        selected_tags,
                        read_open,
                    }

                    ReadPane {
                        selected,
                        total_count,
                        selected_index,
                        messages,
                        selected_id,
                        read_open,
                    }
                }
            }
        }
    }
}
