use dioxus::{document::eval, prelude::*};
use dioxus_primitives::{
    accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger},
    aspect_ratio::AspectRatio,
    avatar::{Avatar, AvatarFallback, AvatarImage},
    calendar::{Calendar, CalendarDate, CalendarGrid, CalendarHeader, CalendarNavigation},
    checkbox::{Checkbox, CheckboxIndicator},
    collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
    context_menu::{ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger},
    dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger},
    hover_card::{HoverCard, HoverCardAlign, HoverCardContent, HoverCardSide, HoverCardTrigger},
    menubar::{Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger},
    progress::{Progress, ProgressIndicator},
    radio_group::{RadioGroup, RadioItem},
    scroll_area::{ScrollArea, ScrollDirection},
    select::{Select, SelectGroup, SelectOption},
    separator::Separator,
    slider::{Slider, SliderRange, SliderThumb, SliderTrack, SliderValue},
    switch::{Switch, SwitchThumb},
    tabs::{TabContent, TabTrigger, Tabs},
    toast::{ToastOptions, ToastProvider, use_toast},
    toggle_group::{ToggleGroup, ToggleItem},
    toolbar::{Toolbar, ToolbarButton, ToolbarSeparator},
    tooltip::{Tooltip, TooltipContent, TooltipSide, TooltipTrigger},
};


#[component]
pub(super) fn SelectExample() -> Element {
    let mut selected = use_signal(|| None::<String>);

    // Debug output for selected value
    use_effect(move || {
        if let Some(value) = selected() {
            println!("Selected value: {}", value);
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/select/style.css") }
        div { class: "select-example",
            h3 { "Select Example" }

            // Basic select
            div { class: "select-container",
                // Label for the select
                label { class: "select-label", "Choose a fruit:" }

                // Native select element
                Select {
                    class: "select",
                    value: selected,
                    on_value_change: move |value| selected.set(value),
                    placeholder: "Select a fruit...",

                    // Fruits group
                    SelectGroup { label: "Fruits".to_string(),

                        SelectOption { value: "apple".to_string(), "Apple" }
                        SelectOption { value: "banana".to_string(), "Banana" }
                        SelectOption { value: "orange".to_string(), "Orange" }
                        SelectOption { value: "strawberry".to_string(), "Strawberry" }
                        SelectOption { value: "watermelon".to_string(), "Watermelon" }
                    }

                    // Other options group
                    SelectGroup { label: "Other".to_string(),

                        SelectOption { value: "other".to_string(), "Other" }
                    }
                }
            }

            // Display selected value
            div { class: "selected-value",
                if let Some(value) = selected() {
                    "Selected: {value}"
                } else {
                    "No selection"
                }
            }
        }
    }
}
