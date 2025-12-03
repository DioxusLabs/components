The sheet component is a panel that slides in from the edge of the screen. It can be used to display additional content, forms, or navigation menus without leaving the current page.

## Component Structure

```rust
// The sheet component must wrap all sheet elements.
Sheet {
    // The open prop determines if the sheet is currently open or closed.
    open: open(),
    // SheetContent wraps the content and defines the side from which the sheet slides in.
    // Available sides: Top, Right (default), Bottom, Left.
    SheetContent {
        side: SheetSide::Right,
        // SheetHeader groups the title and description at the top.
        SheetHeader {
            // The sheet title defines the heading of the sheet.
            SheetTitle {
                "Edit Profile"
            }
            // The sheet description provides additional information about the sheet.
            SheetDescription {
                "Make changes to your profile here."
            }
        }
        // Add your main content here.
        // SheetFooter groups actions at the bottom.
        SheetFooter {
            // SheetClose can be used to close the sheet.
            SheetClose {
                "Close"
            }
        }
    }
}
```

## SheetClose with `as` prop

The `as` prop allows you to render a custom element while preserving the close behavior, similar to shadcn/ui's `asChild` pattern.

```rust
// Default: renders as <button>
SheetClose { "Close" }

// Custom element: attributes include the preset onclick handler
SheetClose {
    r#as: |attributes| rsx! {
        a { href: "#", ..attributes, "Go back" }
    }
}
```
