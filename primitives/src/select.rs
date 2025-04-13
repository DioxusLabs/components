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
    #[props(default = "Select an option")]
    placeholder: &'static str,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    // Use internal state for value if not controlled
    let mut internal_value = use_signal(|| props.value.map(|x| x()).unwrap_or(props.default_value));

    // Handle value changes
    let handle_change = move |event: Event<FormData>| {
        let value = event.value();
        let new_value = if value.is_empty() { None } else { Some(value) };
        internal_value.set(new_value.clone());
        props.on_value_change.call(new_value);
    };

    // Get the current value (either controlled or internal)
    let current_value = props.value.map(|v| v()).unwrap_or_else(|| internal_value());

    rsx! {
        select {
            // Standard HTML attributes
            name: props.name,
            disabled: (props.disabled)(),
            required: (props.required)(),

            // Handle value change
            value: current_value.clone().unwrap_or_default(),
            onchange: handle_change,

            // Pass through other attributes
            ..props.attributes,

            // Add placeholder option if needed
            if current_value.is_none() {
                option { value: "", selected: true, disabled: true, {props.placeholder} }
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

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectOption(props: SelectOptionProps) -> Element {
    rsx! {
        option {
            value: props.value.clone(),
            disabled: (props.disabled)(),
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

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    rsx! {
        optgroup {
            label: props.label.clone(),
            disabled: (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}
