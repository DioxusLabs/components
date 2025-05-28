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
pub(super) fn ContextMenuExample() -> Element {
    let mut selected_value = use_signal(String::new);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/context_menu/style.css") }
        div { class: "context-menu-example",
            ContextMenu {
                ContextMenuTrigger { class: "context-menu-trigger", "Right click here to open context menu" }

                ContextMenuContent { class: "context-menu-content",
                    ContextMenuItem {
                        class: "context-menu-item",
                        value: "edit".to_string(),
                        index: 0usize,
                        on_select: move |value| {
                            selected_value.set(value);
                        },
                        "Edit"
                    }

                    ContextMenuItem {
                        class: "context-menu-item",
                        value: "duplicate".to_string(),
                        index: 1usize,
                        on_select: move |value| {
                            selected_value.set(value);
                        },
                        "Duplicate"
                    }

                    ContextMenuItem {
                        class: "context-menu-item",
                        value: "delete".to_string(),
                        index: 2usize,
                        on_select: move |value| {
                            selected_value.set(value);
                        },
                        "Delete"
                    }
                }
            }

            div { class: "selected-value",
                if selected_value().is_empty() {
                    "No action selected"
                } else {
                    "Selected action: {selected_value()}"
                }
            }
        }
    }
}
