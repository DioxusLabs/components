use crate::{use_id_or, use_unique_id};
use dioxus_lib::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    /// The controlled value of the select
    #[props(default)]
    value: Option<Signal<Option<String>>>,

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

    /// Optional ID for the select
    #[props(default)]
    id: ReadOnlySignal<Option<String>>,

    /// Optional placeholder text
    #[props(default = String::from("Select an option"))]
    placeholder: String,

    /// Optional label for the select (for accessibility)
    #[props(default)]
    aria_label: Option<String>,

    /// Optional ID of the element that labels this select (for accessibility)
    #[props(default)]
    aria_labelledby: Option<String>,

    /// Optional ID of the element that describes this select (for accessibility)
    #[props(default)]
    aria_describedby: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    // Use internal state for value if not controlled
    let mut internal_value = use_signal(|| props.value.map(|x| x()).unwrap_or(props.default_value));

    // Generate unique IDs for accessibility if not provided
    let select_id = use_unique_id();

    // Create an ID for the select element using use_id_or
    let id_value = use_id_or(select_id, props.id);

    // Handle value changes
    let handle_change = move |event: Event<FormData>| {
        let value = event.value();
        let new_value = if value.is_empty() { None } else { Some(value) };
        internal_value.set(new_value.clone());
        props.on_value_change.call(new_value);
    };

    // Get the current value (either controlled or internal)
    let current_value = props.value.map(|v| v()).unwrap_or_else(|| internal_value());

    // Determine if a value is selected for ARIA
    let has_selection = current_value.is_some();

    // Create option ID for aria-activedescendant
    let active_option_id =
        has_selection.then(|| format!("option-{}", current_value.clone().unwrap()));

    rsx! {
        select {
            // Standard HTML attributes
            id: id_value,
            name: props.name,
            disabled: (props.disabled)(),
            required: (props.required)(),

            // Handle value change
            value: current_value.clone().unwrap_or_default(),
            onchange: handle_change,

            // ARIA attributes
            role: "combobox",
            aria_haspopup: "listbox",
            aria_expanded: "false", // Native select handles expansion state
            aria_autocomplete: "none",
            aria_required: (props.required)().to_string(),
            aria_label: props.aria_label.clone(),
            aria_labelledby: props.aria_labelledby.clone(),
            aria_describedby: props.aria_describedby.clone(),
            aria_invalid: "false", // Could be made dynamic with form validation
            aria_activedescendant: active_option_id,

            // Pass through other attributes
            ..props.attributes,

            // Add placeholder option if needed
            if current_value.is_none() {
                option {
                    value: "",
                    selected: true,
                    disabled: true,
                    role: "option",
                    aria_selected: "false",
                    {props.placeholder}
                }
            }

            // Render children (options)
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectOptionProps {
    /// The value of the option
    value: String,

    /// Whether the option is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Optional ID for the option
    #[props(default)]
    id: ReadOnlySignal<Option<String>>,

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
    let option_id = use_signal(|| format!("option-{}", props.value));

    // Use use_id_or to handle the ID
    let id = use_id_or(option_id, props.id);

    rsx! {
        option {
            id,
            value: props.value.clone(),
            disabled: (props.disabled)(),

            // ARIA attributes
            role: "option",
            aria_selected: "false", // Will be set to true by the browser when selected
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
        optgroup {
            id,
            label: props.label.clone(),
            disabled: (props.disabled)(),

            // ARIA attributes
            role: "group",
            aria_disabled: (props.disabled)().to_string(),
            aria_label: props.aria_label.clone(),
            aria_roledescription: props.aria_roledescription.clone(),

            ..props.attributes,
            {props.children}
        }
    }
}
