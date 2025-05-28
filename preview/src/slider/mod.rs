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
pub(super) fn SliderExample() -> Element {
    let mut value = use_signal(|| SliderValue::Single(50.0));
    let range_value = use_signal(|| SliderValue::Range(25.0, 75.0));

    rsx! {
        div { class: "slider-example",
            // Single value slider
            div {
                label { "Single Value Slider" }
                div { style: "display: flex; align-items: center; gap: 1rem;",
                    Slider {
                        class: "slider",
                        value,
                        horizontal: true,
                        on_value_change: move |v| {
                            value.set(v);
                        },

                        SliderTrack { class: "slider-track",
                            SliderRange { class: "slider-range" }
                            SliderThumb { class: "slider-thumb" }
                        }
                    }
                    input {
                        r#type: "text",
                        readonly: true,
                        value: match value() {
                            SliderValue::Single(v) => format!("{:.1}", v),
                            _ => String::new(),
                        },
                    }
                }
            }
                // Range slider
        // div {
        //     label { "Range Slider" }
        //     div { style: "display: flex; align-items: center; gap: 1rem;",
        //         Slider {
        //             class: "slider",
        //             value: range_value,
        //             on_value_change: move |v| {
        //                 range_value.set(v);
        //             },
        //
        //             SliderTrack { class: "slider-track",
        //                 SliderRange { class: "slider-range" }
        //                 SliderThumb { class: "slider-thumb", index: 0usize }
        //                 SliderThumb { class: "slider-thumb", index: 1usize }
        //             }
        //         }
        //         input {
        //             r#type: "text",
        //             readonly: true,
        //             value: match range_value() {
        //                 SliderValue::Range(start, end) => format!("{:.1}, {:.1}", start, end),
        //                 _ => String::new(),
        //             },
        //         }
        //     }
        // }
        }
    }
}
