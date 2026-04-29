use dioxus::prelude::*;

use crate::components::input::Input;
use crate::components::separator::Separator;
use crate::components::sidebar::{SidebarInset, SidebarProvider, SidebarTrigger};
use crate::theme::DarkModeToggle;

mod avatars;
mod filters;
mod list_pane;
mod read_pane;
mod sidebar;
mod state;

use list_pane::ListPane;
use read_pane::ReadPane;
use sidebar::EmailSidebar;
use state::{
    active_folder_label, selected_message_index, selected_message_uid, set_search_query,
    visible_message_ids, EmailClientState, EmailClientStateStoreExt,
};

#[component]
pub fn EmailClient() -> Element {
    let state = use_store(EmailClientState::new);

    let visible_ids = use_memo(move || visible_message_ids(state));
    let selected_uid = use_memo(move || selected_message_uid(state, &visible_ids.read()));
    let total_count = use_memo(move || visible_ids.read().len());
    let selected_index =
        use_memo(move || selected_message_index(selected_uid.read().as_str(), &visible_ids.read()));

    let folder_label = active_folder_label(state);
    let read_open = state.read_open().cloned();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./email_client.css") }

        SidebarProvider {
            EmailSidebar { state }

            SidebarInset {
                header { class: "ec-topbar",
                    SidebarTrigger {}
                    Separator { horizontal: false, decorative: true }
                    h1 { class: "ec-title", {folder_label} }
                    Input {
                        r#type: "search",
                        "aria-label": "Search mail",
                        name: "mail-search",
                        value: state.search_query(),
                        oninput: move |event: FormEvent| {
                            set_search_query(state, event.value());
                        },
                        placeholder: "Search mail, people, attachments…",
                    }
                    DarkModeToggle {}
                }

                div { class: if read_open { "ec-main ec-reading" } else { "ec-main" },
                    ListPane { state, visible_ids, selected_uid }

                    ReadPane {
                        state,
                        selected_uid,
                        total_count,
                        selected_index,
                    }
                }
            }
        }
    }
}
