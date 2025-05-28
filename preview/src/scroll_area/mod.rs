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
pub(super) fn ScrollAreaExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/scroll_area/style.css") }
        div { class: "scroll-area-demo",
            // Vertical scroll example
            div { class: "scroll-demo-section",
                h3 { "Vertical Scroll" }
                ScrollArea {
                    class: "demo-scroll-area",
                    direction: ScrollDirection::Vertical,

                    div { class: "scroll-content",
                        for i in 1..=20 {
                            p { "Scrollable content item {i}" }
                        }
                    }
                }
            }

            // Horizontal scroll example
            div { class: "scroll-demo-section",
                h3 { "Horizontal Scroll" }
                ScrollArea {
                    class: "demo-scroll-area",
                    direction: ScrollDirection::Horizontal,

                    div { class: "scroll-content-horizontal",
                        for i in 1..=20 {
                            span { "Column {i} " }
                        }
                    }
                }
            }

            // Both directions example
            div { class: "scroll-demo-section",
                h3 { "Both Directions" }
                ScrollArea {
                    class: "demo-scroll-area",
                    direction: ScrollDirection::Both,

                    div { class: "scroll-content-both",
                        for i in 1..=20 {
                            div {
                                for j in 1..=20 {
                                    span { "Cell {i},{j} " }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
