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
pub(super) fn AvatarExample() -> Element {
    let mut avatar_state = use_signal(|| "No state yet".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/avatar/style.css") }

        // Basic examples section
        div { class: "avatar-example-section",
            h4 { "Basic Examples" }
            div { class: "avatar-example",
                // Basic Avatar with image and fallback
                div { class: "avatar-item",
                    p { class: "avatar-label", "Basic Usage" }
                    Avatar {
                        class: "avatar",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 1: {:?}", state));
                        },

                        AvatarImage {
                            src: "https://github.com/DioxusLabs.png",
                            alt: "User avatar",
                        }

                        AvatarFallback { class: "avatar-fallback", "UA" }
                    }
                }

                // Avatar with error state (fallback shown)
                div { class: "avatar-item",
                    p { class: "avatar-label", "Error State" }
                    Avatar {
                        class: "avatar",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 2: {:?}", state));
                        },

                        AvatarImage {
                            src: "https://invalid-url.example/image.jpg",
                            alt: "Invalid image",
                        }

                        AvatarFallback { class: "avatar-fallback", "JD" }
                    }
                }

                // Avatar with emoji fallback
                div { class: "avatar-item",
                    p { class: "avatar-label", "Emoji Fallback" }
                    Avatar {
                        class: "avatar",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 3: {:?}", state));
                        },

                        AvatarImage {
                            src: "https://invalid-url.example/image.jpg",
                            alt: "Invalid image",
                        }

                        AvatarFallback { class: "avatar-fallback", "👤" }
                    }
                }

                // Avatar with different size
                div { class: "avatar-item",
                    p { class: "avatar-label", "Large Size" }
                    Avatar {
                        class: "avatar avatar-lg",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 4: {:?}", state));
                        },

                        AvatarImage {
                            src: "https://github.com/DioxusLabs.png",
                            alt: "Large avatar",
                        }

                        AvatarFallback { class: "avatar-fallback", "LG" }
                    }
                }
            }
        }
    }
}