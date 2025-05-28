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
pub(super) fn RadioGroupExample() -> Element {
    let mut value = use_signal(|| String::from("option1"));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/radio_group/style.css") }
        RadioGroup {
            class: "radio-group",
            value,
            on_value_change: move |new_value| {
                value.set(new_value);
            },

            RadioItem {
                class: "radio-item",
                value: "option1".to_string(),
                index: 0usize,
                "Option 1"
            }
            RadioItem {
                class: "radio-item",
                value: "option2".to_string(),
                index: 1usize,
                "Option 2"
            }
            RadioItem {
                class: "radio-item",
                value: "option3".to_string(),
                index: 2usize,
                "Option 3"
            }
        }

        div { style: "margin-top: 1rem;", "Selected value: {value()}" }
    }
}