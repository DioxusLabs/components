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
pub(super) fn ToolbarExample() -> Element {
    let mut text_style = use_signal(Vec::new);
    let mut text_align = use_signal(|| String::from("left"));

    let mut toggle_style = move |style: &str| {
        let mut current_styles = text_style();
        if current_styles.contains(&style.to_string()) {
            current_styles.retain(|s| s != style);
        } else {
            current_styles.push(style.to_string());
        }
        text_style.set(current_styles);
    };

    let mut set_align = move |align: &str| {
        text_align.set(align.to_string());
    };

    let text_classes = use_memo(move || {
        let mut classes = Vec::new();
        for style in text_style() {
            match style.as_str() {
                "bold" => classes.push("toolbar-bold"),
                "italic" => classes.push("toolbar-italic"),
                "underline" => classes.push("toolbar-underline"),
                _ => {}
            }
        }
        classes.join(" ")
    });

    let text_align_style = use_memo(move || format!("text-align: {};", text_align()));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/toolbar/style.css") }

        div { class: "toolbar-example",
            h3 { "Text Formatting Toolbar" }

            Toolbar { class: "toolbar", aria_label: "Text formatting",

                ToolbarButton {
                    class: "toolbar-button",
                    index: 0usize,
                    on_click: move |_| toggle_style("bold"),
                    "Bold"
                }

                ToolbarButton {
                    class: "toolbar-button",
                    index: 1usize,
                    on_click: move |_| toggle_style("italic"),
                    "Italic"
                }

                ToolbarButton {
                    class: "toolbar-button",
                    index: 2usize,
                    on_click: move |_| toggle_style("underline"),
                    "Underline"
                }

                ToolbarSeparator { class: "toolbar-separator" }

                ToolbarButton {
                    class: "toolbar-button",
                    index: 3usize,
                    on_click: move |_| set_align("left"),
                    "Align Left"
                }

                ToolbarButton {
                    class: "toolbar-button",
                    index: 4usize,
                    on_click: move |_| set_align("center"),
                    "Align Center"
                }

                ToolbarButton {
                    class: "toolbar-button",
                    index: 5usize,
                    on_click: move |_| set_align("right"),
                    "Align Right"
                }
            }

            div { class: "toolbar-content",
                p { class: text_classes, style: text_align_style,
                    "This is a sample text that will be formatted according to the toolbar buttons you click. Try clicking the buttons above to see how the text formatting changes."
                }
            }
        }
    }
}