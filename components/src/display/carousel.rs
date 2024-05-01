use dioxus::prelude::*;

use crate::theme::use_theme;

const _: &str = manganis::mg!(file("./styles/core/carousel.css"));
const CARD_ARROW_LEFT_IMG: &str = manganis::mg!(file("./images/card-arrow-left-pd.svg"));
const CARD_ARROW_RIGHT_IMG: &str = manganis::mg!(file("./images/card-arrow-right-pd.svg"));

#[derive(Clone, Copy)]
struct ItemCounter(i32);

props!(CarouselProps { children: Element });

pub fn Carousel(props: CarouselProps) -> Element {
    let item_counter = use_context_provider(|| Signal::new(ItemCounter(-1)));
    let mut current_item = use_signal(|| 0);

    let unique_id = crate::use_unique_id();

    // Mouse scrolling
    let mut mouse_down = use_signal(|| false);
    let mut start_x = use_signal(|| 0.0);
    let mut scroll_left = use_signal(|| 0.0);

    let mouse_is_up = move |_| mouse_down.set(false);
    let mouse_is_down = move |e: Event<MouseData>| async move {
        if let Some(id) = unique_id() {
            mouse_down.set(true);

            let page_x = e.page_coordinates().x;
            let (new_scroll_left, offset_left) = get_lefts(id).await;

            start_x.set(page_x - offset_left);
            scroll_left.set(new_scroll_left);
        }
    };

    let on_mouse_move = move |e: Event<MouseData>| async move {
        if !mouse_down() {
            return;
        }

        if let Some(id) = unique_id() {
            let page_x = e.page_coordinates().x;
            let (_, offset_left) = get_lefts(id.clone()).await;

            let x = page_x - offset_left;
            let walk = x - start_x();
            let final_value = scroll_left() - walk;

            let eval = eval(
                r#"
                let elementId = await dioxus.recv();  
                let element = document.getElementById(elementId);
                let value = await dioxus.recv();
                element.scrollLeft = value;
                "#,
            );
            eval.send(id.into()).unwrap();
            eval.send(final_value.into()).unwrap();
        }
    };

    // Button Handlers
    let on_button_left = move |_| async move {
        let new_index = current_item() - 1;
        if new_index < 0 {
            return;
        }
        current_item.set(new_index);
        if let Some(id) = unique_id() {
            scrollTo(id, new_index).await;
        }
    };
    let on_button_right = move |_| async move {
        let new_index = current_item() + 1;

        if new_index > item_counter().0 {
            return;
        }
        current_item.set(new_index);
        if let Some(id) = unique_id() {
            scrollTo(id, new_index).await;
        }
    };

    let at_min = current_item() == 0;
    let at_max = current_item() == item_counter().0;

    let theme = use_theme();

    rsx! {
        div {
            class: "dxc-carousel {theme().0}",
            div {
                class: "dxc-carousel-btn first {theme().0}",
                button {
                    class: if at_min { "disabled" },
                    onclick: on_button_left,
                    img { src: "{CARD_ARROW_LEFT_IMG}"}
                }
            }
            div {
                id: if let Some(id) = unique_id() { "{id}" },
                class: "dxc-carousel-display {theme().0}",
                onmousemove: on_mouse_move,
                onmousedown: mouse_is_down,
                onmouseup: mouse_is_up,
                onmouseleave: mouse_is_up,

                {props.children}
            }
            div {
                class: "dxc-carousel-btn last {theme().0}",
                button {
                    class: if at_max { "disabled" },
                    onclick: on_button_right,
                    img { src: "{CARD_ARROW_RIGHT_IMG}"}
                }
            }
        }
    }
}

props!(CarouselItemProps { children: Element });

pub fn CarouselItem(props: CarouselItemProps) -> Element {
    use_counter();
    let theme = use_theme();

    rsx! {
        div {
            class: "dxc-carousel-item {theme().0}",
            {props.children} 
        }
    }
}

// Tells the parent that an item exists.
fn use_counter() {
    let mut counter = use_context::<Signal<ItemCounter>>();
    let mut counter_incremented = use_signal(|| false);

    use_effect(move || {
        if !counter_incremented() {
            let count = counter().0;
            counter.set(ItemCounter(count + 1));
            counter_incremented.set(true);
        }
    });
}

async fn get_lefts(id: String) -> (f64, f64) {
    let mut eval = eval(
        r#"
        let elementId = await dioxus.recv();  
        let element = document.getElementById(elementId);
        let scrollLeft = element.scrollLeft;
        let offsetLeft = element.offsetLeft;
        dioxus.send(scrollLeft);
        dioxus.send(offsetLeft);
        "#,
    );

    eval.send(id.into()).unwrap();
    let scroll_left = eval.recv().await.unwrap().as_f64().unwrap();
    let offset_left = eval.recv().await.unwrap().as_f64().unwrap();

    (scroll_left, offset_left)
}

async fn scrollTo(parent_id: String, index: i32) {
    // Eval does not send the number 0 for some reason.
    // We work around this by incrementing here and decrementing in the eval.
    let index = index + 1;
    let eval = eval(
        r#"
        let parentId = await dioxus.recv();
        let elIndex = await dioxus.recv() - 1;

        let parent = document.getElementById(parentId);
        let children = parent.children;

        let childEl = children[elIndex];
        childEl.scrollIntoView({ behavior: "smooth", inline: "center"});

        "#,
    );
    eval.send(parent_id.into()).unwrap();
    eval.send(index.into()).unwrap();
    eval.await.unwrap();
}
