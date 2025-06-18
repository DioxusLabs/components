use std::{collections::HashMap, fmt::Write};

use crate::{
    focus::{FocusState, use_focus_provider},
    use_controlled, use_effect_cleanup, use_id_or, use_unique_id,
};
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
    // A list of [physical key codes, virtual key codes] to guess the keyboard layout
    known_key_positions: Signal<Vec<[char; 2]>>,
}

impl SelectContext {
    fn guess_keyboard_layout(&self) -> KeyboardLayout {
        let known_key_positions = self.known_key_positions.read();

        KeyboardLayout::guess(&*known_key_positions)
    }

    fn current_match(&self) -> Option<String> {
        let typeahead = self.typeahead_buffer.read();
        if typeahead.is_empty() {
            return None;
        }

        let typeahead = &*typeahead;

        let typeahead_characters: Box<[_]> = typeahead.chars().collect();
        let keyboard_layout = self.guess_keyboard_layout();

        let best_match = self
            .options
            .read()
            .iter()
            .map(|opt| {
                let value = opt.value.read();
                let value = &*value;
                let value_characters: Box<[_]> = value.chars().collect();

                let distance =
                    levenshtein_distance(&typeahead_characters, &value_characters, |a, b| {
                        keyboard_layout.substitution_cost(a, b)
                    });

                (distance, value.clone())
            })
            .max_by(|(d1, _), (d2, _)| f32::total_cmp(d1, d2))
            .map(|(_, value)| value);

        best_match
    }
}

fn levenshtein_distance(
    s1: &[char],
    s2: &[char],
    substitution_cost: impl Fn(char, char) -> f32,
) -> f32 {
    let mut dp = vec![vec![0.0; s2.len() + 1]; s1.len() + 1];

    for i in 0..=s1.len() {
        dp[i][0] = i as f32;
    }
    for j in 0..=s2.len() {
        dp[0][j] = j as f32;
    }

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            let cost = if s1[i - 1] == s2[j - 1] {
                0.0
            } else {
                substitution_cost(s1[i - 1], s2[j - 1])
            };

            dp[i][j] = f32::min(
                f32::min(dp[i - 1][j] + 1.0, dp[i][j - 1] + 1.0),
                dp[i - 1][j - 1] + cost,
            );
        }
    }

    dp[s1.len()][s2.len()]
}

#[test]
fn test_levenshtein_distance() {
    let s1: Vec<char> = "kitten".chars().collect();
    let s2: Vec<char> = "sitting".chars().collect();

    let distance = levenshtein_distance(&s1, &s2, |a, b| if a == b { 0.0 } else { 1.0 });
    assert_eq!(distance, 3.0); // kitten -> sitting requires 3 edits

    let s1: Vec<char> = "kitten".chars().collect();
    let s2: Vec<char> = "litten".chars().collect();
    let keyboard_layout = KeyboardLayout::Qwerty;
    let distance = levenshtein_distance(&s1, &s2, |a, b| keyboard_layout.substitution_cost(a, b));
    assert_eq!(distance, 0.071428575); // Using QWERTY layout, the distance is lower than 1.0 because the characters are close on the keyboard

    let keyboard_layout = KeyboardLayout::ColemakDH;
    let distance = levenshtein_distance(&s1, &s2, |a, b| keyboard_layout.substitution_cost(a, b));
    assert_eq!(distance, 0.21428572); // Using ColemakDH layout, the distance is higher because the characters are further apart on the keyboard
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum KeyboardLayout {
    Qwerty,
    ColemakDH,
    Colemak,
    Unknown,
}

impl KeyboardLayout {
    const KNOWN_KEYBOARD_LAYOUTS: &[(KeyboardLayout, [[char; 10]; 4])] = &[
        (KeyboardLayout::Qwerty, QWERTY_KEYBOARD_LAYOUT),
        (KeyboardLayout::ColemakDH, COLEMACK_DH_KEYBOARD_LAYOUT),
        (KeyboardLayout::Colemak, COLEMAK_KEYBOARD_LAYOUT),
    ];

    fn guess(known_key_positions: &[[char; 2]]) -> Self {
        if known_key_positions.is_empty() {
            return Self::Unknown;
        }

        let mut matching = Self::KNOWN_KEYBOARD_LAYOUTS.to_vec();
        for [physical_position, virtual_position] in known_key_positions {
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
        let position_a = self.char_position(a);
        let position_b = self.char_position(b);

        match (position_a, position_b) {
            (Some((row_a, col_a)), Some((row_b, col_b))) => {
                let row_diff = (row_a as f32 - row_b as f32).abs();
                let col_diff = (col_a as f32 - col_b as f32).abs();
                // Use Manhattan distance for simplicity and scale to a max of 1.0
                (row_diff + col_diff) / 14.0
            }
            _ => 1.0,
        }
    }

    fn char_position(&self, character: char) -> Option<(usize, usize)> {
        let matrix = match self {
            KeyboardLayout::Qwerty => &QWERTY_KEYBOARD_LAYOUT,
            KeyboardLayout::ColemakDH => &COLEMACK_DH_KEYBOARD_LAYOUT,
            KeyboardLayout::Colemak => &COLEMAK_KEYBOARD_LAYOUT,
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

static QWERTY_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
    ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'],
    ['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
];

static COLEMACK_DH_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['q', 'w', 'f', 'p', 'b', 'j', 'l', 'u', 'y', ';'],
    ['a', 'r', 's', 't', 'g', 'm', 'n', 'e', 'i', 'o'],
    ['x', 'c', 'd', 'v', 'z', 'k', 'h', ',', '.', '/'],
];

static COLEMAK_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['q', 'w', 'f', 'p', 'g', 'j', 'l', 'u', 'y', ';'],
    ['a', 'r', 's', 't', 'd', 'h', 'n', 'e', 'i', 'o'],
    ['z', 'x', 'c', 'v', 'b', 'k', 'm', ',', '.', '/'],
];

#[test]
fn test_detect_keyboard_layout() {
    // Default to an unknown layout
    let layout = KeyboardLayout::guess(&[]);
    assert_eq!(layout, KeyboardLayout::Unknown);

    // If any keys match, use the query layout
    let layout = KeyboardLayout::guess(&[['q', 'q']]);
    assert_eq!(layout, KeyboardLayout::Qwerty);

    // Otherwise, guess a good layout
    let layout = KeyboardLayout::guess(&[['d', 's']]);
    assert_eq!(layout, KeyboardLayout::ColemakDH);

    let layout = KeyboardLayout::guess(&[['d', 's'], ['g', 'd']]);
    assert_eq!(layout, KeyboardLayout::Colemak);
}

struct OptionMatch {
    range: std::ops::Range<usize>,
    value: String,
}

struct OptionState {
    /// The tab index of the option
    tab_index: ReadOnlySignal<usize>,
    /// The value of the option
    value: ReadOnlySignal<String>,
    /// The id of the option
    id: ReadOnlySignal<String>,
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
    let focused_item = use_signal(|| None);
    let options = use_signal(Default::default);
    let mut known_key_positions = use_signal(Vec::new);

    use_context_provider(|| SelectContext {
        typeahead_buffer,
        open: open.clone(),
        value,
        set_value,
        options,
        focused_item,
        known_key_positions,
    });

    let current_value = value();

    let onkeydown = move |event: KeyboardEvent| {
        let key = event.key();
        let code = event.code();
        if let (Some(code), Key::Character(key)) = (code_to_char(code), &key) {
            let chars = key.chars().collect::<Vec<_>>();
            if let &[key_as_char] = chars.as_slice() {
                known_key_positions.write().push([code, key_as_char]);
            }
        }

        match key {
            Key::Character(new_text) => {
                // Add character to typeahead buffer
                typeahead_buffer.write().push_str(&new_text);
            }
            _ => {}
        }
    };

    rsx! {
        button {
            // Standard HTML attributes
            name: props.name,
            disabled: (props.disabled)(),

            onclick: move |_| open.toggle(),
            onkeydown,

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
    let open: bool = ctx.open.cloned();

    rsx! {
        div {
            role: "listbox",
            aria_activedescendant: active_option_id,
            tabindex: if open { "0" } else { "-1" },

            // Data attributes
            "data-state": if open { "open" } else { "closed" },

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
            tab_index: index,
            value,
            id: id.into(),
        };

        // Add the option to the context's options
        ctx.options.write().push(option_state);
    });

    use_effect_cleanup(move || {
        ctx.options
            .write()
            .retain(|opt| &*opt.id.read() != &*id.read());
    });

    rsx! {
        div {
            role: "option",
            id,
            tabindex: "-1",

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
    let group_id = use_signal(|| format!("group-{}", props.label.to_lowercase().replace(" ", "-")));

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
