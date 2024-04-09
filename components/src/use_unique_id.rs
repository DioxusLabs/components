use std::sync::Mutex;
use dioxus::prelude::*;

static mut CURRENT_ID: Mutex<usize> = Mutex::new(0);
const ID_PREFIX: &str = "dxc-uniq-";

pub fn use_unique_id() -> Signal<Option<String>> {
    let mut id = use_signal(|| None);

    if id().is_none() {
        let mut current_id = unsafe { CURRENT_ID.lock().unwrap() };
        *current_id += 1;

        let new_id = format!("{}{}", ID_PREFIX, *current_id);
        id.set(Some(new_id));
    }

    id
}