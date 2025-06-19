use std::collections::HashMap;

use crate::{
    focus::{FocusState, use_focus_controlled_item, use_focus_provider},
    use_controlled, use_effect_cleanup, use_id_or, use_unique_id,
};
use dioxus::html::input_data::MouseButton;
use dioxus_lib::prelude::*;

#[derive(Clone, Copy)]
struct SelectContext {
    // The typeahead buffer for searching options
    typeahead_buffer: Signal<String>,
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
    // The ID of the list for ARIA attributes
    list_id: Signal<Option<String>>,
    // The focus state for the select
    focus_state: FocusState,
    // Whether the select is disabled
    disabled: ReadOnlySignal<bool>,
    // The placeholder text
    placeholder: ReadOnlySignal<String>,
}

impl SelectContext {
    fn select_current_item(&mut self) {
        // If the select is open, select the focused item
        if self.open.cloned() {
            if let Some(focused_index) = self.focus_state.current_focus() {
                let options = self.options.read();
                if let Some(option) = options.iter().find(|opt| opt.tab_index == focused_index) {
                    self.set_value.call(Some(option.value.clone()));
                    self.open.set(false);
                }
            }
        }
    }

    fn add_to_known_key_positions(&mut self, code: &Code, key: &Key) {
        if let (Some(code), Key::Character(key)) = (code_to_char(code), &key) {
            let chars = key.chars().collect::<Vec<_>>();
            if let &[key_as_char] = chars.as_slice() {
                self.known_key_positions.write().insert(code, key_as_char);
            }
        }
    }

    fn add_to_typeahead_buffer(&mut self, new_text: &str) {
        let mut typeahead_buffer = self.typeahead_buffer.write();
        // Add character to typeahead buffer
        typeahead_buffer.push_str(new_text);
        // Trim the typeahead buffer to the maximum length of the options
        let longest_option_length = self
            .options
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
}

fn best_match(
    keyboard_layout: &KeyboardLayout,
    typeahead: &str,
    options: &[OptionState],
) -> Option<usize> {
    if typeahead.is_empty() {
        return None;
    }

    let typeahead_characters: Box<[_]> = typeahead.chars().collect();

    options
        .iter()
        .map(|opt| {
            let value = &opt.value;
            let value_characters: Box<[_]> = value.chars().collect();
            let distance =
                normalized_distance(&typeahead_characters, &value_characters, keyboard_layout);
            (distance, opt.tab_index)
        })
        .min_by(|(d1, _), (d2, _)| f32::total_cmp(d1, d2))
        .map(|(_, value)| value)
}

fn normalized_distance(
    typeahead_characters: &[char],
    value_characters: &[char],
    keyboard_layout: &KeyboardLayout,
) -> f32 {
    // Only use the the start of the value characters
    let value_characters =
        &value_characters[..value_characters.len().min(typeahead_characters.len())];
    // Only use the end of the typeahead characters
    let typeahead_characters = &typeahead_characters[typeahead_characters
        .len()
        .saturating_sub(value_characters.len())..];

    levenshtein_distance(typeahead_characters, value_characters, |a, b| {
        keyboard_layout.substitution_cost(a, b)
    })
}

// The recency bias of the levenshtein distance function
fn recency_bias(char_index: usize, total_length: usize) -> f32 {
    ((char_index as f32 + 1.5).ln() / (total_length as f32 + 1.5).ln()).powi(4)
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

    let mut prev = 0.0;
    for j in 0..=value.len() {
        let new = prev + (1.0 - recency_bias(j, value.len())) * 0.5;
        prev = new;
        dp[0][j] = new;
    }
    let mut prev = 0.0;
    for (i, row) in dp.iter_mut().enumerate().take(typeahead.len() + 1) {
        let new = prev + recency_bias(i, typeahead.len()) * 0.5;
        prev = new;
        row[0] = new;
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
                    // Insertion is cheaper for old characters in the typeahead
                    dp[i - 1][j] + recency_bias(i, typeahead.len()),
                    // Deletion is cheaper for untyped characters in the value
                    dp[i][j - 1] + (1.0 - recency_bias(j, value.len())),
                ),
                // Substitution
                dp[i - 1][j - 1] + cost * 2.0 * recency_bias(i, typeahead.len()),
            );
        }
    }

    let result = dp[typeahead.len()][value.len()];

    let max_possible = dp[typeahead.len()][0].max(dp[0][value.len()]);

    // Normalize the result to a range of 0.0 to 1.0
    result / max_possible
}

#[test]
fn test_levenshtein_distance() {
    let s1: Vec<char> = "kitten".chars().collect();
    let s2: Vec<char> = "sitting".chars().collect();

    let distance = levenshtein_distance(&s1, &s2, |a, b| if a == b { 0.0 } else { 1.0 });
    assert_eq!(distance, 0.5158963); // kitten -> sitting requires 3 edits, but the distance is scaled by recency bias and normalized

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

#[test]
fn test_normalized_distance() {
    let typeahead: Vec<char> = "goodhe".chars().collect();
    let string1: Vec<char> = "hello".chars().collect();
    let string2: Vec<char> = "goodbye".chars().collect();
    let distance1 = normalized_distance(&typeahead, &string1, &KeyboardLayout::ColemakDH);
    println!("Distance from 'goodhe' to 'hello': {}", distance1);
    let distance2 = normalized_distance(&typeahead, &string2, &KeyboardLayout::ColemakDH);
    println!("Distance from 'goodhe' to 'goodbye': {}", distance2);
    assert!(
        distance1 < distance2,
        "Distance to 'hello' should be less than distance to 'goodbye'"
    );

    let typeahead: Vec<char> = "orangwat".chars().collect();
    let string1: Vec<char> = "watermelon".chars().collect();
    let string2: Vec<char> = "orange".chars().collect();
    let distance1 = normalized_distance(&typeahead, &string1, &KeyboardLayout::ColemakDH);
    println!("Distance from 'orangwat' to 'watermelon': {}", distance1);
    let distance2 = normalized_distance(&typeahead, &string2, &KeyboardLayout::ColemakDH);
    println!("Distance from 'orangwat' to 'orange': {}", distance2);
    assert!(
        distance1 < distance2,
        "Distance to 'watermelon' should be less than distance to 'orange'"
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

fn code_to_char(code: &Code) -> Option<char> {
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
    #[props(default = ReadOnlySignal::new(Signal::new(String::from("Select an option"))))]
    placeholder: ReadOnlySignal<String>,

    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    roving_loop: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let open = use_signal(|| false);

    let mut typeahead_buffer = use_signal(String::new);
    let options = use_signal(Vec::default);
    let known_key_positions = use_signal(Default::default);
    let list_id = use_signal(|| None);

    let keyboard_layout = use_memo(move || {
        let known_key_positions = known_key_positions.read();

        KeyboardLayout::guess(&known_key_positions)
    });

    let best_match = use_memo(move || {
        let typeahead = typeahead_buffer.read();
        let options = options.read();
        let keyboard_layout = keyboard_layout.read();

        best_match(&keyboard_layout, &typeahead, &options)
    });

    let mut focus_state = use_focus_provider(props.roving_loop);

    // Set the focused item to the best match if it exists
    use_effect(move || {
        if let Some(focused_value) = &*best_match.read() {
            focus_state.set_focus(Some(*focused_value));
        }
    });

    // Clear the typeahead buffer when the select is closed
    use_effect(move || {
        if !open() {
            typeahead_buffer.take();
        }
    });

    use_context_provider(|| SelectContext {
        typeahead_buffer,
        open,
        value,
        set_value,
        options,
        known_key_positions,
        list_id,
        focus_state,
        disabled: props.disabled,
        placeholder: props.placeholder,
    });

    rsx! {
        div {
            // Data attributes
            "data-state": if open() { "open" } else { "closed" },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectTriggerProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    let mut ctx: SelectContext = use_context();

    let mut open = ctx.open;

    rsx! {
        button {
            // Standard HTML attributes
            disabled: (ctx.disabled)(),

            onclick: move |_| {
                open.toggle();
            },
            onkeydown: move |event| {
                match event.key() {
                    Key::ArrowUp => {
                        open.set(true);
                        ctx.focus_state.focus_last();
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    Key::ArrowDown => {
                        open.set(true);
                        ctx.focus_state.focus_first();
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    _ => {}
                }
            },

            // ARIA attributes
            aria_haspopup: "listbox",
            aria_expanded: open(),
            aria_controls: ctx.list_id,

            // Pass through other attributes
            ..props.attributes,

            // Add placeholder option if needed
            span {
                "data-placeholder": ctx.value.read().is_none(),
                {ctx.value.cloned().unwrap_or_else(|| ctx.placeholder.cloned())}
            }

            // Render children (options)
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectListProps {
    /// The ID of the list for ARIA attributes
    #[props(default)]
    id: ReadOnlySignal<Option<String>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectList(props: SelectListProps) -> Element {
    let mut ctx: SelectContext = use_context();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);
    use_effect(move || {
        ctx.list_id.set(Some(id()));
    });

    let mut open = ctx.open;
    let mut listbox_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let focused = move || open() && !ctx.focus_state.any_focused();

    use_effect(move || {
        let Some(listbox_ref) = listbox_ref() else {
            return;
        };
        if focused() {
            spawn(async move {
                _ = listbox_ref.set_focus(true);
            });
        }
    });

    let onkeydown = move |event: KeyboardEvent| {
        let key = event.key();
        let code = event.code();
        ctx.add_to_known_key_positions(&code, &key);

        let mut arrow_key_navigation = |event: KeyboardEvent| {
            // Clear the typeahead buffer
            ctx.typeahead_buffer.take();
            event.prevent_default();
            event.stop_propagation();
        };

        match key {
            Key::Character(new_text) => {
                if new_text == " " {
                    ctx.select_current_item();
                    event.prevent_default();
                    event.stop_propagation();
                    return;
                }

                ctx.add_to_typeahead_buffer(&new_text);
            }
            Key::ArrowUp => {
                arrow_key_navigation(event);

                ctx.focus_state.focus_prev();
            }
            Key::End => {
                arrow_key_navigation(event);

                ctx.focus_state.focus_last();
            }
            Key::ArrowDown => {
                arrow_key_navigation(event);

                ctx.focus_state.focus_next();
            }
            Key::Home => {
                arrow_key_navigation(event);

                ctx.focus_state.focus_first();
            }
            Key::Enter => {
                ctx.select_current_item();
                open.set(false);
                event.prevent_default();
                event.stop_propagation();
            }
            _ => {}
        }
    };

    rsx! {
        div {
            id,
            role: "listbox",
            tabindex: if focused() { "0" } else { "-1" },

            // Data attributes
            "data-state": if open() { "open" } else { "closed" },

            onmounted: move |evt| listbox_ref.set(Some(evt.data())),
            onkeydown,
            onblur: move |_| {
                if focused() {
                    open.set(false);
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, Copy)]
struct SelectOptionContext {
    /// If the option is selected
    selected: Memo<bool>,
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
    let value = props.value;

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
        ctx.options.write().retain(|opt| *opt.id != *id.read());
    });

    let onmounted = use_focus_controlled_item(props.index);
    let focused = move || ctx.focus_state.is_focused(index());
    let disabled = ctx.disabled.cloned() || props.disabled.cloned();
    let selected = use_memo(move || ctx.value.read().as_ref() == Some(&props.value.read()));

    use_context_provider(|| SelectOptionContext { selected });

    rsx! {
        div {
            role: "option",
            id,
            tabindex: if focused() { "0" } else { "-1" },
            onmounted,

            // ARIA attributes
            aria_selected: selected(),
            aria_disabled: disabled,
            aria_label: props.aria_label.clone(),
            aria_roledescription: props.aria_roledescription.clone(),

            onpointerdown: move |event| {
                if !disabled && event.trigger_button() == Some(MouseButton::Primary) {
                    ctx.set_value.call(Some(props.value.read().clone()));
                    ctx.open.set(false);
                }
            },
            onblur: move |_| {
                if focused() {
                    ctx.focus_state.blur();
                    ctx.open.set(false);
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectItemIndicatorProps {
    children: Element,
}

#[component]
pub fn SelectItemIndicator(props: SelectItemIndicatorProps) -> Element {
    let ctx: SelectOptionContext = use_context();
    if !(ctx.selected)() {
        return rsx! {};
    }
    rsx! {
        {props.children}
    }
}

#[derive(Clone, Copy)]
struct SelectGroupContext {
    labeled_by: Signal<Option<String>>,
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectGroupProps {
    /// Whether the group is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Optional ID for the group
    #[props(default)]
    id: ReadOnlySignal<Option<String>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    let ctx: SelectContext = use_context();
    let disabled = ctx.disabled.cloned() || props.disabled.cloned();

    let labeled_by = use_signal(|| None);

    use_context_provider(|| SelectGroupContext { labeled_by });

    rsx! {
        div {
            role: "group",

            // ARIA attributes
            aria_disabled: disabled,
            aria_labelledby: labeled_by,

            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectGroupLabelProps {
    id: ReadOnlySignal<Option<String>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    let mut ctx: SelectGroupContext = use_context();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    use_effect(move || {
        ctx.labeled_by.set(Some(id()));
    });

    rsx! {
        div {
            // Set the ID for the label
            id,
            ..props.attributes,
            {props.children}
        }
    }
}
