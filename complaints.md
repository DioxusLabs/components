This file is to track any dx issues with Dioxus found while developing this library.

### Unused props don't emit unused warnings.
https://github.com/DioxusLabs/dioxus/issues/3919

### Setting default of signal prop is verbose.
https://github.com/DioxusLabs/dioxus/issues/3920

It's verbose to set a `Signal` or `ReadSignal`'s default value through props.
```rust
#[derive(Props, Clone, PartialEq)]
pub struct SomeProps {

    // This sets bool to be false
    #[props(default)] 
    value: ReadSignal<bool>,

    // This is what I'd like, except it wants a ReadSignal
    #[props(default = true)] 
    value: ReadSignal<bool>,

    // Instead you have to do this:
    #[props(default = ReadSignal::new(Signal::new(true)))]
    value: ReadSignal<bool>,

    // Same for a regular signal:
    #[props(default = Signal::new(true))]
    value: Signal<bool>,
}
```

### No way to know a component or element's parent, siblings, or children.

Some stuff relies on knowing their surrounding elements for proper behavior. 

Take [radix-primitives' switch](https://github.com/radix-ui/primitives/blob/6e75e117977c9e6ffa939e6951a707f16ba0f95e/packages/react/switch/src/switch.tsx#L51) as an example. It detects when the switch is in a form and creates an input so that the switch's value bubbles with the form submit event.

This is also an issue with keybind navigation - we can give components ids to internally track them through a parent context, but how do we know which order they are in for navigation?

At a minimum, I need the ordering index. This could be a special prop similar to `children`. 

### Need Portals
Components should behave as if they are still a child of the parent of the "portaled" item. Same scope basically - context is still consumable as if it was a child.

Aka the component is still used in the same spot, and the portal only moves where that component is actually rendered. Portals can't have children or attributes:

```rust

#[component]
pub fn App() -> Element {
    let portal = use_portal();

    rsx! {
        div {
            // ... nested stuff
            PortalIn {
                portal,

                // Children of PortalIn becomes children of PortalOut.
                div {
                    h1 { "Alert Dialog!" }
                    p { "alert!!" }
                }
            }
        }

        div {
            // ... other nested stuff
            PortalOut { portal }
        }
    }
}

```

### `From<Signal<T>>` Is Not Implemented For `Option<ReadSignal<T>>`

### `From<T>` Is Not Implemented For `Option<ReadSignal<T>>`
`T` can already be converted to `Option<Signal<T>>` when provided thru props.
This however doesn't work for `Option<ReadSignal<T>>`.

### Number Props Don't Type Infer
Normally Rust would automatically determine that a number should be of type thru inference but for props it doesn't work when the prop is a signal.

`index: ReadSignal<usize>,` fails
`index: usize,` works

```rust
SomeComponent {
    index: 1,
}
```

### No `#[props(extends = MyPropsStruct)]`
https://github.com/DioxusLabs/dioxus/issues/3938

```rust
pub fn MyComp1(#[props(extends = MyComp2)] comp2_attr: Vec<Attribute>) -> Element {
    rsx! {
        MyComp2 {
            ..comp2_attr
        }
    }
}

```

### Unable To Insert Attributes On `Element`.

Ideally there would be a way to pass attributes through a top-level component. Radix has the `asChild` prop which replaces their element with the user provided one. E.g.
```rs
rsx! {

    if as_child {
        p {
            aria_something: "abc",
            {children}
        }
    } else {
        {children} // Can't add `aria_something`
    }
}
```
