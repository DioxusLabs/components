use std::time::Duration;

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
pub(super) fn ToastExample() -> Element {
    // Get the toast API
    let toast_api = use_toast();

    rsx! {
        div { class: "toast-example",
            h3 { "Toast Notifications" }
            p { "Click the buttons below to show different types of toast notifications." }

            h4 { "Timed Toasts (auto-dismiss)" }
            div { class: "toast-buttons",
                button {
                    onclick: move |_| {
                        toast_api
                            .success(
                                "Success".to_string(),
                                Some(ToastOptions {
                                    duration: Some(Duration::from_secs(3)),
                                    ..Default::default()
                                }),
                            );
                    },
                    "Success (3s)"
                }

                button {
                    onclick: move |_| {
                        toast_api
                            .error(
                                "Error".to_string(),
                                Some(ToastOptions {
                                    duration: Some(Duration::from_secs(5)),
                                    ..Default::default()
                                }),
                            );
                    },
                    "Error (5s)"
                }

                button {
                    onclick: move |_| {
                        toast_api
                            .warning(
                                "Warning".to_string(),
                                Some(ToastOptions {
                                    description: Some("This action might cause issues".to_string()),
                                    duration: Some(Duration::from_secs(3)),
                                    ..Default::default()
                                }),
                            );
                    },
                    "Warning (3s)"
                }
            }

            h4 { "Permanent Toasts (manual close)" }
            div { class: "toast-buttons",
                button {
                    onclick: move |_| {
                        toast_api
                            .success(
                                "Important".to_string(),
                                Some(ToastOptions {
                                    permanent: true,
                                    ..Default::default()
                                }),
                            );
                    },
                    "Permanent Success"
                }

                button {
                    onclick: move |_| {
                        toast_api
                            .error(
                                "Critical Error".to_string(),
                                Some(ToastOptions {
                                    permanent: true,
                                    ..Default::default()
                                }),
                            );
                    },
                    "Permanent Error"
                }

                button {
                    onclick: move |_| {
                        toast_api
                            .info(
                                "Custom Toast".to_string(),
                                Some(ToastOptions {
                                    description: Some(
                                        "This is a custom toast with specific settings".to_string(),
                                    ),
                                    duration: Some(Duration::from_secs(10)),
                                    permanent: false,
                                }),
                            );
                    },
                    "Custom Info (10s)"
                }
            }
        }
    }
}
