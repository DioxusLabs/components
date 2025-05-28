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
pub(super) fn MenubarExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/menubar/style.css") }
        div { class: "menubar-example",
            Menubar { class: "menubar",
                MenubarMenu { class: "menubar-menu", index: 0usize,
                    MenubarTrigger { class: "menubar-trigger", "File" }
                    MenubarContent { class: "menubar-content",
                        MenubarItem {
                            class: "menubar-item",
                            value: "new".to_string(),
                            on_select: move |value| {
                                eval(&format!("console.log('Selected: {}')", value));
                            },
                            "New"
                        }
                        MenubarItem {
                            class: "menubar-item",
                            value: "open".to_string(),
                            on_select: move |value| {
                                eval(&format!("console.log('Selected: {}')", value));
                            },
                            "Open"
                        }
                        MenubarItem {
                            class: "menubar-item",
                            value: "save".to_string(),
                            on_select: move |value| {
                                eval(&format!("console.log('Selected: {}')", value));
                            },
                            "Save"
                        }
                    }
                }

                MenubarMenu { class: "menubar-menu", index: 1usize,
                    MenubarTrigger { class: "menubar-trigger", "Edit" }
                    MenubarContent { class: "menubar-content",
                        MenubarItem {
                            class: "menubar-item",
                            value: "cut".to_string(),
                            on_select: move |value| {
                                eval(&format!("console.log('Selected: {}')", value));
                            },
                            "Cut"
                        }
                        MenubarItem {
                            class: "menubar-item",
                            value: "copy".to_string(),
                            on_select: move |value| {
                                eval(&format!("console.log('Selected: {}')", value));
                            },
                            "Copy"
                        }
                        MenubarItem {
                            class: "menubar-item",
                            value: "paste".to_string(),
                            on_select: move |value| {
                                eval(&format!("console.log('Selected: {}')", value));
                            },
                            "Paste"
                        }
                    }
                }
            }
        }
    }
}