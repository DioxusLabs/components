//! Defines the [`OneTimePasswordInput`] component and its sub-components for building
//! accessible, composable one-time-password (OTP) inputs.

use crate::{use_controlled, use_unique_id};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct OtpCtx {
    value: Memo<String>,
    disabled: ReadSignal<bool>,
    active_index: Memo<Option<usize>>,
}

#[derive(Clone)]
struct PatternMatcher {
    pattern: String,
    class: Option<Vec<CharMatcher>>,
}

#[derive(Clone, PartialEq)]
enum CharMatcher {
    Char(char),
    Range(char, char),
    Digit,
}

impl PatternMatcher {
    fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            class: parse_full_value_char_class(pattern),
        }
    }

    fn matches(&self, value: &str) -> bool {
        value.is_empty()
            || self.class.as_ref().map_or(true, |class| {
                value.chars().all(|c| {
                    class.iter().any(|matcher| match matcher {
                        CharMatcher::Char(expected) => *expected == c,
                        CharMatcher::Range(start, end) => *start <= c && c <= *end,
                        CharMatcher::Digit => c.is_ascii_digit(),
                    })
                })
            })
    }
}

impl PartialEq for PatternMatcher {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
    }
}

fn parse_full_value_char_class(pattern: &str) -> Option<Vec<CharMatcher>> {
    let pattern = pattern
        .strip_prefix('^')
        .unwrap_or(pattern)
        .strip_suffix('$')
        .unwrap_or(pattern);
    let pattern = pattern
        .strip_suffix('*')
        .or_else(|| pattern.strip_suffix('+'))
        .unwrap_or(pattern);

    match pattern {
        r"\d" => Some(vec![CharMatcher::Digit]),
        _ if pattern.starts_with('[') && pattern.ends_with(']') => {
            parse_char_class(&pattern[1..pattern.len() - 1])
        }
        _ => None,
    }
}

fn parse_char_class(class: &str) -> Option<Vec<CharMatcher>> {
    let mut chars = class.chars().peekable();
    let mut matchers = Vec::new();

    while let Some(start) = chars.next() {
        let start = if start == '\\' {
            match chars.next()? {
                'd' => {
                    matchers.push(CharMatcher::Digit);
                    continue;
                }
                escaped => escaped,
            }
        } else {
            start
        };

        if chars.peek() == Some(&'-') {
            chars.next();
            let end = chars.next()?;
            matchers.push(CharMatcher::Range(start, end));
        } else {
            matchers.push(CharMatcher::Char(start));
        }
    }

    Some(matchers)
}

/// The props for the [`OneTimePasswordInput`] component.
#[derive(Props, Clone, PartialEq)]
pub struct OneTimePasswordInputProps {
    /// The controlled value of the OTP input.
    pub value: ReadSignal<Option<String>>,

    /// The default value when uncontrolled.
    #[props(default)]
    pub default_value: String,

    /// The maximum number of characters the input accepts (the total number of slots).
    pub maxlength: ReadSignal<usize>,

    /// HTML pattern attribute applied to the underlying input. Defaults to digits only.
    #[props(default = ReadSignal::new(Signal::new(String::from("[0-9]*"))))]
    pub pattern: ReadSignal<String>,

    /// Hint for the on-screen keyboard. Defaults to `"numeric"`.
    #[props(default = ReadSignal::new(Signal::new(String::from("numeric"))))]
    pub inputmode: ReadSignal<String>,

    /// Autocomplete hint applied to the underlying input. Defaults to `"one-time-code"`.
    #[props(default = ReadSignal::new(Signal::new(String::from("one-time-code"))))]
    pub autocomplete: ReadSignal<String>,

    /// Whether the input is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the input is required in a form.
    #[props(default)]
    pub required: ReadSignal<bool>,

    /// The name attribute used for form submission.
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Callback fired whenever the value changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Callback fired when the value reaches `maxlength`.
    #[props(default)]
    pub on_complete: Callback<String>,

    /// Additional attributes applied to the wrapper element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the input — typically [`OneTimePasswordGroup`], [`OneTimePasswordSlot`],
    /// and [`OneTimePasswordSeparator`] components.
    pub children: Element,
}

/// # OneTimePasswordInput
///
/// The `OneTimePasswordInput` is the root of an OTP entry. It renders a single, accessible
/// `<input>` element overlaid on top of its children so paste, autofill (`autocomplete="one-time-code"`),
/// IME composition, and screen readers continue to work, while child [`OneTimePasswordSlot`]s render
/// the visual representation of each character.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::otp::{
///     OneTimePasswordInput, OneTimePasswordGroup, OneTimePasswordSlot, OneTimePasswordSeparator,
/// };
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         OneTimePasswordInput { maxlength: 6usize,
///             OneTimePasswordGroup {
///                 OneTimePasswordSlot { index: 0usize }
///                 OneTimePasswordSlot { index: 1usize }
///                 OneTimePasswordSlot { index: 2usize }
///             }
///             OneTimePasswordSeparator {}
///             OneTimePasswordGroup {
///                 OneTimePasswordSlot { index: 3usize }
///                 OneTimePasswordSlot { index: 4usize }
///                 OneTimePasswordSlot { index: 5usize }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The wrapper sets the following data attributes:
/// - `data-disabled`: `true` or `false` depending on the `disabled` prop.
/// - `data-focused`: `true` or `false` depending on whether the underlying input has focus.
#[component]
pub fn OneTimePasswordInput(props: OneTimePasswordInputProps) -> Element {
    let maxlength = props.maxlength;
    let pattern = props.pattern;
    let on_complete = props.on_complete;
    let pattern_matcher = use_memo(move || PatternMatcher::new(&pattern()));

    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let input_id = use_unique_id();
    let mut is_focused = use_signal(|| false);
    let mut cursor = use_signal(|| 0usize);
    let (input_label_attributes, wrapper_attributes): (Vec<_>, Vec<_>) =
        props.attributes.iter().cloned().partition(|attr| {
            matches!(
                attr.name,
                "aria-label" | "aria_label" | "aria-labelledby" | "aria_labelledby"
            )
        });

    let active_index = use_memo(move || {
        if !is_focused() {
            return None;
        }
        let max = maxlength();
        if max == 0 {
            return None;
        }
        Some(cursor().min(max - 1))
    });

    use_context_provider(|| OtpCtx {
        value,
        disabled: props.disabled,
        active_index,
    });

    rsx! {
        div {
            role: "group",
            position: "relative",
            "data-disabled": props.disabled,
            "data-focused": is_focused,
            ..wrapper_attributes,

            {props.children}

            input {
                id: input_id,
                r#type: "text",
                inputmode: props.inputmode,
                autocomplete: props.autocomplete,
                pattern,
                name: props.name,
                disabled: props.disabled,
                aria_required: props.required,
                required: props.required,
                maxlength: maxlength() as i64,
                value,

                style: "position:absolute;top:0;left:0;right:0;bottom:0;width:100%;height:100%;opacity:1;color:transparent;background:transparent;caret-color:transparent;outline:none;border:none;padding:0;margin:0;text-align:center;font-family:inherit;font-size:inherit;cursor:text;user-select:none;",

                onkeydown: move |e: Event<KeyboardData>| {
                    if (props.disabled)() {
                        return;
                    }
                    let key = e.key();
                    let max = maxlength();
                    if max == 0 {
                        return;
                    }
                    let mods = e.modifiers();
                    let mut chars: Vec<char> = value.read().chars().collect();
                    let mut new_cursor = cursor();
                    let mut value_changed = false;

                    match key {
                        Key::ArrowLeft => {
                            new_cursor = new_cursor.saturating_sub(1);
                            e.prevent_default();
                        }
                        Key::ArrowRight => {
                            if new_cursor < max {
                                new_cursor += 1;
                            }
                            e.prevent_default();
                        }
                        Key::Home => {
                            new_cursor = 0;
                            e.prevent_default();
                        }
                        Key::End => {
                            new_cursor = chars.len();
                            e.prevent_default();
                        }
                        Key::Backspace => {
                            e.prevent_default();
                            if mods.ctrl() || mods.meta() {
                                if !chars.is_empty() {
                                    chars.clear();
                                    new_cursor = 0;
                                    value_changed = true;
                                }
                            } else {
                                let effective = new_cursor.min(chars.len());
                                if effective > 0 {
                                    chars.remove(effective - 1);
                                    new_cursor = effective - 1;
                                    value_changed = true;
                                }
                            }
                        }
                        Key::Delete => {
                            e.prevent_default();
                            if new_cursor < chars.len() {
                                chars.remove(new_cursor);
                                value_changed = true;
                            }
                        }
                        Key::Character(ref s)
                            if s.chars().count() == 1
                                && !mods.ctrl()
                                && !mods.meta()
                                && !mods.alt() =>
                        {
                            e.prevent_default();
                            let insert_at = new_cursor.min(chars.len());
                            if insert_at < max {
                                let c = s.chars().next().unwrap();
                                let mut next_chars = chars.clone();
                                if insert_at < next_chars.len() {
                                    next_chars[insert_at] = c;
                                } else {
                                    next_chars.push(c);
                                }
                                if next_chars.len() > max {
                                    next_chars.truncate(max);
                                }
                                let next_value: String = next_chars.iter().copied().collect();
                                if !pattern_matcher.read().matches(&next_value) {
                                    return;
                                }
                                if insert_at < chars.len() {
                                    chars[insert_at] = c;
                                } else {
                                    chars.push(c);
                                }
                                if chars.len() > max {
                                    chars.truncate(max);
                                }
                                new_cursor = (insert_at + 1).min(max);
                                value_changed = true;
                            }
                        }
                        _ => {}
                    }

                    if value_changed {
                        let new_value: String = chars.into_iter().collect();
                        set_value.call(new_value.clone());
                        if new_value.chars().count() == max {
                            on_complete.call(new_value);
                        }
                    }
                    if new_cursor != cursor() {
                        cursor.set(new_cursor);
                    }
                },

                oninput: move |e| {
                    // Catches paste, autofill, IME, and on-screen keyboards.
                    let raw = e.value();
                    let max = maxlength();
                    let filtered: String = raw.chars().take(max).collect();
                    if !pattern_matcher.read().matches(&filtered) {
                        return;
                    }
                    let len = filtered.chars().count();
                    if filtered != *value.read() {
                        set_value.call(filtered.clone());
                        if max > 0 && len == max {
                            on_complete.call(filtered);
                        }
                    }
                    cursor.set(len);
                },

                onfocus: move |_| {
                    is_focused.set(true);
                    cursor.set(value.read().chars().count());
                },
                onblur: move |_| is_focused.set(false),
                ..input_label_attributes,
            }
        }
    }
}

/// The props for the [`OneTimePasswordGroup`] component.
#[derive(Props, Clone, PartialEq)]
pub struct OneTimePasswordGroupProps {
    /// Additional attributes applied to the group element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The slots inside the group.
    pub children: Element,
}

/// # OneTimePasswordGroup
///
/// A visual grouping of [`OneTimePasswordSlot`]s. Used to render contiguous slots
/// separated by [`OneTimePasswordSeparator`]s.
#[component]
pub fn OneTimePasswordGroup(props: OneTimePasswordGroupProps) -> Element {
    rsx! {
        div {
            role: "presentation",
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`OneTimePasswordSlot`] component.
#[derive(Props, Clone, PartialEq)]
pub struct OneTimePasswordSlotProps {
    /// The position of this slot in the value (zero-based).
    pub index: ReadSignal<usize>,

    /// Additional attributes applied to the slot element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Optional children rendered after the character (for example, a custom caret element).
    /// The current character is exposed via the `data-char` attribute.
    pub children: Element,
}

/// # OneTimePasswordSlot
///
/// A single slot within a [`OneTimePasswordInput`]. Renders the character at `index` from the
/// shared value. Must be used inside a [`OneTimePasswordInput`].
///
/// ## Styling
///
/// The slot element exposes:
/// - `data-active`: `true` when this slot is the next one to receive input.
/// - `data-empty`: `true` when no character has been entered at this position.
/// - `data-disabled`: mirrors the parent's disabled state.
/// - `data-char`: the current character at this position (empty when none).
#[component]
pub fn OneTimePasswordSlot(props: OneTimePasswordSlotProps) -> Element {
    let ctx: OtpCtx = use_context();
    let index = props.index;

    let char_at = use_memo(move || {
        ctx.value
            .read()
            .chars()
            .nth(index())
            .map(|c| c.to_string())
            .unwrap_or_default()
    });
    let is_active = use_memo(move || ctx.active_index.cloned() == Some(index()));
    let is_empty = use_memo(move || char_at.read().is_empty());

    rsx! {
        div {
            role: "presentation",
            aria_hidden: "true",
            "data-active": is_active,
            "data-empty": is_empty,
            "data-disabled": ctx.disabled,
            "data-char": char_at,
            ..props.attributes,

            {char_at}
            {props.children}
        }
    }
}

/// The props for the [`OneTimePasswordSeparator`] component.
#[derive(Props, Clone, PartialEq)]
pub struct OneTimePasswordSeparatorProps {
    /// Additional attributes applied to the separator element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Optional children that replace the default separator content.
    pub children: Element,
}

/// # OneTimePasswordSeparator
///
/// A purely decorative separator placed between [`OneTimePasswordGroup`]s.
#[component]
pub fn OneTimePasswordSeparator(props: OneTimePasswordSeparatorProps) -> Element {
    rsx! {
        div {
            role: "separator",
            aria_hidden: "true",
            ..props.attributes,
            {props.children}
        }
    }
}
