use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::dashboard::common::{
    seed_message_states, FolderId, MessageState, MessageTag, TabId, FOLDERS,
};

use super::filters::{message_matches_filters, message_matches_folder};

#[derive(Clone, PartialEq, Store)]
pub(super) struct EmailClientState {
    pub(super) messages: BTreeMap<String, MessageState>,
    pub(super) message_order: Vec<String>,
    pub(super) active_folder: FolderId,
    pub(super) active_tab: TabId,
    pub(super) search_query: String,
    pub(super) selected_tags: Vec<MessageTag>,
    pub(super) selected_id: String,
    pub(super) read_open: bool,
}

impl EmailClientState {
    pub(super) fn new() -> Self {
        let seeded = seed_message_states();
        let message_order = seeded.iter().map(|message| message.uid.clone()).collect();
        let messages = seeded
            .into_iter()
            .map(|message| (message.uid.clone(), message))
            .collect();

        Self {
            messages,
            message_order,
            active_folder: FolderId::Inbox,
            active_tab: TabId::All,
            search_query: String::new(),
            selected_tags: Vec::new(),
            selected_id: String::from("m1#0"),
            read_open: false,
        }
    }
}

pub(super) fn active_folder_label(state: Store<EmailClientState>) -> &'static str {
    let active_folder = state.active_folder().cloned();
    FOLDERS
        .iter()
        .find(|folder| folder.id == active_folder)
        .map(|folder| folder.label)
        .unwrap_or("Inbox")
}

pub(super) fn visible_message_ids(state: Store<EmailClientState>) -> Vec<String> {
    let active_folder = state.active_folder().cloned();
    let active_tab = state.active_tab().cloned();
    let search_query = state.search_query().cloned();
    let selected_tags = state.selected_tags().cloned();
    let messages_store = state.messages();
    let order_store = state.message_order();
    let messages = messages_store.read();
    let order = order_store.read();

    order
        .iter()
        .filter(|uid| {
            messages.get(uid.as_str()).is_some_and(|message| {
                message_matches_filters(
                    message,
                    active_folder,
                    active_tab,
                    search_query.as_str(),
                    &selected_tags,
                )
            })
        })
        .cloned()
        .collect()
}

pub(super) fn selected_message_uid(
    state: Store<EmailClientState>,
    visible_ids: &[String],
) -> String {
    let selected_id = state.selected_id().cloned();
    if visible_ids.iter().any(|uid| uid == &selected_id) {
        return selected_id;
    }

    visible_ids
        .first()
        .cloned()
        .or_else(|| state.message_order().read().first().cloned())
        .expect("seed_message_states is non-empty")
}

pub(super) fn selected_message_index(selected_uid: &str, visible_ids: &[String]) -> usize {
    visible_ids
        .iter()
        .position(|uid| uid == selected_uid)
        .map(|index| index + 1)
        .unwrap_or(1)
}

pub(super) fn folder_count(state: Store<EmailClientState>, folder_id: FolderId) -> u32 {
    let messages_store = state.messages();
    let count = messages_store
        .read()
        .values()
        .filter(|message| message_matches_folder(message, folder_id))
        .count() as u32;
    count
}

pub(super) fn tab_count(
    state: Store<EmailClientState>,
    tab_id: TabId,
    visible_query: &str,
    selected_tags: &[MessageTag],
) -> u32 {
    let active_folder = state.active_folder().cloned();
    let messages_store = state.messages();
    let count = messages_store
        .read()
        .values()
        .filter(|message| {
            message_matches_filters(message, active_folder, tab_id, visible_query, selected_tags)
        })
        .count() as u32;
    count
}

pub(super) fn message_snapshot(state: Store<EmailClientState>, uid: &str) -> Option<MessageState> {
    state
        .messages()
        .get(uid.to_string())
        .map(|message| message.read().clone())
}

pub(super) fn set_active_folder(state: Store<EmailClientState>, folder_id: FolderId) {
    state.active_folder().set(folder_id);
    close_read_pane(state);
}

pub(super) fn set_active_tab(state: Store<EmailClientState>, tab_id: TabId) {
    state.active_tab().set(tab_id);
    close_read_pane(state);
}

pub(super) fn set_search_query(state: Store<EmailClientState>, query: String) {
    state.search_query().set(query);
    close_read_pane(state);
}

pub(super) fn set_selected_tags(state: Store<EmailClientState>, tags: Vec<MessageTag>) {
    state.selected_tags().set(tags);
    close_read_pane(state);
}

pub(super) fn select_message(state: Store<EmailClientState>, uid: String) {
    state.selected_id().set(uid);
    state.read_open().set(true);
}

pub(super) fn close_read_pane(state: Store<EmailClientState>) {
    state.read_open().set(false);
}

pub(super) fn update_message(
    state: Store<EmailClientState>,
    uid: String,
    update: impl FnOnce(&mut MessageState),
) {
    if let Some(mut message) = state.messages().get(uid) {
        let mut message = message.write();
        update(&mut message);
    }
}
