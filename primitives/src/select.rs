use std::collections::HashMap;

use crate::{use_controlled, use_effect_cleanup, use_id_or, use_unique_id};
use dioxus_lib::prelude::*;

#[derive(Clone, Copy)]
struct SelectContext {
    // The typeahead buffer for searching options
    typeahead_buffer: Signal<String>,
    // The focused item in the select
    focused_item: Signal<Option<usize>>,
    // If the select is open
    open: Signal<bool>,
    // The currently selected value
    value: Memo<Option<String>>,
    // Set the value
    set_value: Callback<Option<String>>,
    // A list of options with their states
    options: Signal<Vec<OptionState>>,
    // Known key positions for the keyboard layout
    known_key_positions: Signal<HashMap<char, char>>,
}

fn best_match(
    keyboard_layout: &KeyboardLayout,
    typeahead: &str,
    options: &Vec<OptionState>,
) -> Option<usize> {
    if typeahead.is_empty() {
        return None;
    }

    let typeahead_characters: Box<[_]> = typeahead.chars().collect();

    let best_match = options
        .iter()
        .map(|opt| {
            let value = &opt.value;
            let value_characters: Box<[_]> = value.chars().collect();
            // Only use the the start of the value characters
            let value_characters =
                &value_characters[..value_characters.len().min(typeahead_characters.len())];
            // Only use the end of the typeahead characters
            let typeahead_characters = &typeahead_characters[typeahead_characters
                .len()
                .saturating_sub(value_characters.len())..];

            let distance =
                levenshtein_distance(&typeahead_characters, &value_characters, |a, b| {
                    keyboard_layout.substitution_cost(a, b)
                });
            let max_distance =
                (typeahead_characters.len() as f32).max(value_characters.len() as f32);
            let distance = distance / max_distance;

            (distance, opt.tab_index)
        })
        .min_by(|(d1, _), (d2, _)| f32::total_cmp(d1, d2))
        .map(|(_, value)| value);

    best_match
}

// The recency bias of the levenshtein distance function
fn recency_bias(char_index: usize, total_length: usize) -> f32 {
    ((char_index as f32 + 1.5).ln() / (total_length as f32 + 1.5).ln()).powi(2)
}

// We use a weighted Levenshtein distance to account for the recency of characters
// More recent characters have a higher weight, while older characters have a lower weight
// The first few characters in the value are weighted more heavily
//
// When substitution is required, the substitution is cheaper for characters that are closer together on the keyboard
fn levenshtein_distance(
    typeahead: &[char],
    value: &[char],
    substitution_cost: impl Fn(char, char) -> f32,
) -> f32 {
    let mut dp = vec![vec![0.0; value.len() + 1]; typeahead.len() + 1];

    // Weight more recent typeahead characters heavily
    for i in 0..=typeahead.len() {
        dp[i][0] = i as f32 * recency_bias(i, typeahead.len());
    }
    for j in 0..=value.len() {
        dp[0][j] = j as f32 * 0.5f32.max(1.0 - recency_bias(j, value.len()));
    }

    for i in 1..=typeahead.len() {
        for j in 1..=value.len() {
            let cost = if typeahead[i - 1] == value[j - 1] {
                0.0
            } else {
                substitution_cost(typeahead[i - 1], value[j - 1])
            };

            dp[i][j] = f32::min(
                f32::min(
                    // Insertion
                    dp[i - 1][j] + recency_bias(i, typeahead.len()),
                    // Deletion
                    dp[i][j - 1] + 0.5f32.max(1.0 - recency_bias(i, typeahead.len())),
                ),
                // Substitution
                dp[i - 1][j - 1] + cost,
            );
        }
    }

    dp[typeahead.len()][value.len()]
}

#[test]
fn test_levenshtein_distance() {
    let s1: Vec<char> = "kitten".chars().collect();
    let s2: Vec<char> = "sitting".chars().collect();

    let distance = levenshtein_distance(&s1, &s2, |a, b| if a == b { 0.0 } else { 1.0 });
    assert_eq!(distance, 2.5); // kitten -> sitting requires 3 edits, but the distance is scaled by recency bias

    let s1: Vec<char> = "kitten".chars().collect();
    let s2: Vec<char> = "litten".chars().collect();
    let keyboard_layout = KeyboardLayout::Qwerty;
    let qwerty_distance =
        levenshtein_distance(&s1, &s2, |a, b| keyboard_layout.substitution_cost(a, b));

    let keyboard_layout = KeyboardLayout::ColemakDH;
    let colemack_distance =
        levenshtein_distance(&s1, &s2, |a, b| keyboard_layout.substitution_cost(a, b));
    println!("QWERTY distance: {}", qwerty_distance);
    println!("ColemakDH distance: {}", colemack_distance);
    assert!(
        qwerty_distance < colemack_distance,
        "ColemakDH should have a higher distance than QWERTY for the same characters"
    );
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum KeyboardLayout {
    Qwerty,
    ColemakDH,
    Colemak,
    Dvorak,
    Workman,
    Azerty,
    Qwertz,
    Unknown,
}

impl KeyboardLayout {
    const KNOWN_KEYBOARD_LAYOUTS: &[(KeyboardLayout, [[char; 10]; 4])] = &[
        (KeyboardLayout::Qwerty, QWERTY_KEYBOARD_LAYOUT),
        (KeyboardLayout::ColemakDH, COLEMACK_DH_KEYBOARD_LAYOUT),
        (KeyboardLayout::Colemak, COLEMAK_KEYBOARD_LAYOUT),
        (KeyboardLayout::Dvorak, DVORAK_KEYBOARD_LAYOUT),
        (KeyboardLayout::Workman, WORKMAN_KEYBOARD_LAYOUT),
        (KeyboardLayout::Azerty, AZERTY_KEYBOARD_LAYOUT),
        (KeyboardLayout::Qwertz, QWERTZ_KEYBOARD_LAYOUT),
    ];

    fn guess(known_key_positions: &HashMap<char, char>) -> Self {
        if known_key_positions.is_empty() {
            return Self::Unknown;
        }

        let mut matching = Self::KNOWN_KEYBOARD_LAYOUTS.to_vec();
        for (physical_position, virtual_position) in known_key_positions {
            let position_in_qwerty = Self::Qwerty.char_position(*physical_position);

            let Some(position_in_qwerty) = position_in_qwerty else {
                return Self::Unknown;
            };

            matching.retain(|(_, layout_keys)| {
                if let Some(&virtual_char) =
                    layout_keys[position_in_qwerty.0].get(position_in_qwerty.1)
                {
                    virtual_char == *virtual_position
                } else {
                    false
                }
            });
        }

        if let Some((layout, _)) = matching.first() {
            *layout
        } else {
            Self::Unknown
        }
    }

    fn substitution_cost(&self, a: char, b: char) -> f32 {
        if a == b {
            return 0.0;
        }

        let position_a = self.char_position(a);
        let position_b = self.char_position(b);

        match (position_a, position_b) {
            (Some((row_a, col_a)), Some((row_b, col_b))) => {
                let row_diff = (row_a as f32 - row_b as f32).abs();
                let col_diff = (col_a as f32 - col_b as f32).abs();
                // Use Manhattan distance for simplicity and scale to a max of 1.0
                0.5 + (row_diff + col_diff) / 28.0
            }
            _ => 1.0,
        }
    }

    fn char_position(&self, character: char) -> Option<(usize, usize)> {
        let matrix = match self {
            KeyboardLayout::Qwerty => &QWERTY_KEYBOARD_LAYOUT,
            KeyboardLayout::ColemakDH => &COLEMACK_DH_KEYBOARD_LAYOUT,
            KeyboardLayout::Colemak => &COLEMAK_KEYBOARD_LAYOUT,
            KeyboardLayout::Dvorak => &DVORAK_KEYBOARD_LAYOUT,
            KeyboardLayout::Workman => &WORKMAN_KEYBOARD_LAYOUT,
            KeyboardLayout::Azerty => &AZERTY_KEYBOARD_LAYOUT,
            KeyboardLayout::Qwertz => &QWERTZ_KEYBOARD_LAYOUT,
            KeyboardLayout::Unknown => return None,
        };

        matrix.iter().enumerate().find_map(|(row, row_values)| {
            row_values
                .iter()
                .position(|&c| c == character)
                .map(|col| (row, col))
        })
    }
}

fn code_to_char(code: Code) -> Option<char> {
    match code {
        Code::Digit1 => Some('1'),
        Code::Digit2 => Some('2'),
        Code::Digit3 => Some('3'),
        Code::Digit4 => Some('4'),
        Code::Digit5 => Some('5'),
        Code::Digit6 => Some('6'),
        Code::Digit7 => Some('7'),
        Code::Digit8 => Some('8'),
        Code::Digit9 => Some('9'),
        Code::Digit0 => Some('0'),
        Code::KeyA => Some('a'),
        Code::KeyB => Some('b'),
        Code::KeyC => Some('c'),
        Code::KeyD => Some('d'),
        Code::KeyE => Some('e'),
        Code::KeyF => Some('f'),
        Code::KeyG => Some('g'),
        Code::KeyH => Some('h'),
        Code::KeyI => Some('i'),
        Code::KeyJ => Some('j'),
        Code::KeyK => Some('k'),
        Code::KeyL => Some('l'),
        Code::KeyM => Some('m'),
        Code::KeyN => Some('n'),
        Code::KeyO => Some('o'),
        Code::KeyP => Some('p'),
        Code::KeyQ => Some('q'),
        Code::KeyR => Some('r'),
        Code::KeyS => Some('s'),
        Code::KeyT => Some('t'),
        Code::KeyU => Some('u'),
        Code::KeyV => Some('v'),
        Code::KeyW => Some('w'),
        Code::KeyX => Some('x'),
        Code::KeyY => Some('y'),
        Code::KeyZ => Some('z'),
        Code::Period => Some('.'),
        Code::Comma => Some(','),
        Code::Slash => Some('/'),
        Code::Semicolon => Some(';'),
        _ => None,
    }
}

// QWERTY
static QWERTY_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
    ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'],
    ['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
];

// Colemak-DH
static COLEMACK_DH_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['q', 'w', 'f', 'p', 'b', 'j', 'l', 'u', 'y', ';'],
    ['a', 'r', 's', 't', 'g', 'm', 'n', 'e', 'i', 'o'],
    ['x', 'c', 'd', 'v', 'z', 'k', 'h', ',', '.', '/'],
];

// Colemak (mod-dhm standard)
static COLEMAK_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['q', 'w', 'f', 'p', 'g', 'j', 'l', 'u', 'y', ';'],
    ['a', 'r', 's', 't', 'd', 'h', 'n', 'e', 'i', 'o'],
    ['z', 'x', 'c', 'v', 'b', 'k', 'm', ',', '.', '/'],
];

// Dvorak
static DVORAK_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['\'', ',', '.', 'p', 'y', 'f', 'g', 'c', 'r', 'l'],
    ['a', 'o', 'e', 'u', 'i', 'd', 'h', 't', 'n', 's'],
    [';', 'q', 'j', 'k', 'x', 'b', 'm', 'w', 'v', 'z'],
];

// Workman
static WORKMAN_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['q', 'd', 'r', 'w', 'b', 'j', 'f', 'u', 'p', ';'],
    ['a', 's', 'h', 't', 'g', 'y', 'n', 'e', 'o', 'i'],
    ['z', 'x', 'm', 'c', 'v', 'k', 'l', ',', '.', '/'],
];

// AZERTY (France)
static AZERTY_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['a', 'z', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
    ['q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm'],
    ['w', 'x', 'c', 'v', 'b', 'n', ',', ';', ':', '!'],
];

// QWERTZ (Germany/Switzerland)
static QWERTZ_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['q', 'w', 'e', 'r', 't', 'z', 'u', 'i', 'o', 'p'],
    ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'],
    ['y', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
];

#[test]
fn test_detect_keyboard_layout() {
    // Default to an unknown layout
    let layout = KeyboardLayout::guess(&[].into());
    assert_eq!(layout, KeyboardLayout::Unknown);

    // If any keys match, use the query layout
    let layout = KeyboardLayout::guess(&[('q', 'q')].into());
    assert_eq!(layout, KeyboardLayout::Qwerty);

    // Otherwise, guess a good layout
    let layout = KeyboardLayout::guess(&[('d', 's')].into());
    assert_eq!(layout, KeyboardLayout::ColemakDH);

    let layout = KeyboardLayout::guess(&[('d', 's'), ('g', 'd')].into());
    assert_eq!(layout, KeyboardLayout::Colemak);
}

#[derive(Clone, Debug)]
struct OptionState {
    /// The tab index of the option
    tab_index: usize,
    /// The value of the option
    value: String,
    /// The id of the option
    id: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    /// The controlled value of the select
    #[props(default)]
    value: ReadOnlySignal<Option<Option<String>>>,

    /// The default value of the select
    #[props(default)]
    default_value: Option<String>,

    /// Callback when the value changes
    #[props(default)]
    on_value_change: Callback<Option<String>>,

    /// Whether the select is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Whether the select is required
    #[props(default)]
    required: ReadOnlySignal<bool>,

    /// Name of the select for form submission
    #[props(default)]
    name: ReadOnlySignal<String>,

    /// Optional placeholder text
    #[props(default = String::from("Select an option"))]
    placeholder: String,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let mut open = use_signal(|| false);

    let mut typeahead_buffer = use_signal(|| String::new());
    let mut focused_item = use_signal(|| None);
    let options = use_signal(Default::default);
    let known_key_positions = use_signal(Default::default);

    let keyboard_layout = use_memo(move || {
        let known_key_positions = known_key_positions.read();

        KeyboardLayout::guess(&*known_key_positions)
    });

    let best_match = use_memo(move || {
        let typeahead = typeahead_buffer.read();
        let options = options.read();
        let keyboard_layout = keyboard_layout.read();

        best_match(&keyboard_layout, &typeahead, &options)
    });

    // Set the focused item to the best match if it exists
    use_effect(move || {
        if let Some(focused_value) = &*best_match.read() {
            focused_item.set(Some(*focused_value));
        }
    });

    use_context_provider(|| SelectContext {
        typeahead_buffer,
        open: open.clone(),
        value,
        set_value,
        options,
        focused_item,
        known_key_positions,
    });

    // Clear the typeahead buffer when the select is closed
    use_effect(move || {
        if !open() {
            typeahead_buffer.take();
        }
    });

    let current_value = value();

    rsx! {
        button {
            // Standard HTML attributes
            name: props.name,
            disabled: (props.disabled)(),

            onclick: move |_| open.toggle(),

            // Data attributes
            "data-state": if open() { "open" } else { "closed" },

            // ARIA attributes
            aria_haspopup: "listbox",
            aria_expanded: open(),
            aria_required: (props.required)().to_string(),

            // Pass through other attributes
            ..props.attributes,

            // Add placeholder option if needed
            match &current_value {
                Some(value) => rsx! {
                    "{value}"
                },
                None => rsx! {
                    "{props.placeholder}"
                }
            }

            // Render children (options)
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectListProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectList(props: SelectListProps) -> Element {
    let ctx: SelectContext = use_context();

    let active_option_id = use_signal(|| String::new());
    let mut open = ctx.open;
    let mut listbox_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    use_effect(move || {
        let Some(listbox_ref) = listbox_ref() else {
            return;
        };
        if open() {
            spawn(async move {
                _ = listbox_ref.set_focus(true);
            });
        }
    });

    let mut known_key_positions = ctx.known_key_positions;
    let mut focused_item = ctx.focused_item;
    let options = ctx.options;
    let set_value = ctx.set_value;
    let mut typeahead_buffer = ctx.typeahead_buffer;

    let onkeydown = move |event: KeyboardEvent| {
        let key = event.key();
        let code = event.code();
        if let (Some(code), Key::Character(key)) = (code_to_char(code), &key) {
            let chars = key.chars().collect::<Vec<_>>();
            if let &[key_as_char] = chars.as_slice() {
                known_key_positions.write().insert(code, key_as_char);
            }
        }

        let mut arrow_key_navigation = |event: KeyboardEvent| {
            // Clear the typeahead buffer
            typeahead_buffer.take();
            event.prevent_default();
            event.stop_propagation();
        };

        let mut focus_last_item = move || {
            let mut focused_item = focused_item.write();
            *focused_item = options.read().iter().map(|opt| opt.tab_index).max();
        };

        let mut focus_first_item = move || {
            let mut focused_item = focused_item.write();
            *focused_item = options.read().iter().map(|opt| opt.tab_index).min();
        };

        let select_current_item = move || {
            // If the select is open, select the focused item
            if open() {
                if let Some(focused_index) = focused_item.cloned() {
                    let options = options.read();
                    if let Some(option) = options.iter().find(|opt| opt.tab_index == focused_index)
                    {
                        set_value(Some(option.value.clone()));
                    }
                }
            }
        };

        match key {
            Key::Character(new_text) => {
                if new_text == " " {
                    select_current_item();
                    event.prevent_default();
                    event.stop_propagation();
                    return;
                }

                let mut typeahead_buffer = typeahead_buffer.write();
                // Add character to typeahead buffer
                typeahead_buffer.push_str(&new_text);
                // Trim the typeahead buffer to the maximum length of the options
                let longest_option_length = options
                    .read()
                    .iter()
                    .map(|opt| opt.value.chars().count())
                    .max()
                    .unwrap_or_default();
                let overflow_length = typeahead_buffer.len().saturating_sub(longest_option_length);
                if overflow_length > 0 {
                    *typeahead_buffer = typeahead_buffer
                        .chars()
                        .skip(overflow_length)
                        .take(longest_option_length)
                        .collect::<String>();
                }
            }
            Key::ArrowUp => {
                arrow_key_navigation(event);

                // Move focus up
                {
                    let mut focused_item = focused_item.write();
                    if let Some(item) = *focused_item {
                        if item > 0 {
                            *focused_item = Some(item - 1);
                            return;
                        }
                    }
                }
                focus_last_item();
            }
            Key::End => {
                arrow_key_navigation(event);
                focus_last_item();
            }
            Key::ArrowDown => {
                arrow_key_navigation(event);

                // Move focus down
                {
                    let mut focused_item = focused_item.write();
                    if let Some(item) = *focused_item {
                        if item < options.read().len() - 1 {
                            *focused_item = Some(item + 1);
                            return;
                        }
                    }
                }
                focus_first_item();
            }
            Key::Home => {
                arrow_key_navigation(event);
                focus_first_item();
            }
            Key::Enter => {
                select_current_item();
                open.toggle();
            }
            _ => {}
        }
    };

    let is_open = open();

    rsx! {
        div {
            role: "listbox",
            aria_activedescendant: active_option_id,
            tabindex: if is_open { "0" } else { "-1" },

            // Data attributes
            "data-state": if is_open { "open" } else { "closed" },

            onmounted: move |evt| listbox_ref.set(Some(evt.data())),
            onkeydown,
            onblur: move |_| {
                open.set(false);
            },

            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectOptionProps {
    /// The value of the option
    value: ReadOnlySignal<String>,

    /// Whether the option is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Optional ID for the option
    #[props(default)]
    id: ReadOnlySignal<Option<String>>,

    /// The index of the option in the list
    index: ReadOnlySignal<usize>,

    /// Optional label for the option (for accessibility)
    #[props(default)]
    aria_label: Option<String>,

    /// Optional description role for the option (for accessibility)
    #[props(default)]
    aria_roledescription: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectOption(props: SelectOptionProps) -> Element {
    // Generate a unique ID for this option for accessibility
    let option_id = use_unique_id();

    // Use use_id_or to handle the ID
    let id = use_id_or(option_id, props.id);

    let index = props.index;
    let value = props.value.clone();

    // Push this option to the context
    let mut ctx: SelectContext = use_context();
    use_effect(move || {
        let option_state = OptionState {
            tab_index: index(),
            value: value.cloned(),
            id: id(),
        };

        // Add the option to the context's options
        ctx.options.write().push(option_state);
    });

    use_effect_cleanup(move || {
        ctx.options.write().retain(|opt| &*opt.id != &*id.read());
    });

    let focused_item = ctx.focused_item.read();
    let focused = *focused_item == Some(index());

    rsx! {
        div {
            role: "option",
            id,
            tabindex: "-1",

            "data-focused": focused,

            // ARIA attributes
            aria_selected: ctx.value.read().as_ref() == Some(&props.value.read()),
            aria_disabled: (props.disabled)().to_string(),
            aria_label: props.aria_label.clone(),
            aria_roledescription: props.aria_roledescription.clone(),

            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectGroupProps {
    /// Label for the option group
    label: String,

    /// Whether the group is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Optional ID for the group
    #[props(default)]
    id: ReadOnlySignal<Option<String>>,

    /// Optional label for the group (for accessibility)
    #[props(default)]
    aria_label: Option<String>,

    /// Optional description role for the group (for accessibility)
    #[props(default)]
    aria_roledescription: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    // Generate a unique ID for this group
    let group_id = use_unique_id();

    // Use use_id_or to handle the ID
    let id = use_id_or(group_id, props.id);

    rsx! {
        div {
            role: "group",
            id,

            // ARIA attributes
            aria_disabled: (props.disabled)().to_string(),
            aria_label: props.aria_label.clone(),
            aria_roledescription: props.aria_roledescription.clone(),

            ..props.attributes,
            {props.children}
        }
    }
}
