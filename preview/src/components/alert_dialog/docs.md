The AlertDialog primitive provides an accessible, composable modal dialog for critical user confirmations (such as destructive actions). It is unstyled by default except for minimal centering/stacking, and can be fully themed by the consumer.

## Component Structure

```rust
// Usage example:
let open = use_signal(|| false);
rsx! {
    button {
        onclick: move |_| open.set(true),
        type: "button",
        "Show Alert Dialog"
    }
    AlertDialogRoot { open: Some(open), on_open_change: move |v| open.set(v),
        AlertDialogContent {
            // You may pass class/style for custom appearance
            AlertDialogTitle { "Title" }
            AlertDialogDescription { "Description" }
            AlertDialogActions {
                AlertDialogCancel { "Cancel" }
                AlertDialogAction { "Confirm" }
            }
        }
    }
}
```

### Components
- **AlertDialogRoot**: Provides context and manages open state.
- **AlertDialogContent**: The dialog container. Handles accessibility and focus trap. Applies only minimal inline style for centering/stacking if no style is provided.
- **AlertDialogTitle**: The dialog's heading.
- **AlertDialogDescription**: Additional description for the dialog.
- **AlertDialogActions**: Container for action buttons.
- **AlertDialogAction**: Main action button (e.g., confirm/delete). Closes dialog and calls optional `on_click`.
- **AlertDialogCancel**: Cancel/close button. Closes dialog and calls optional `on_click`.

### Notes
- By default, only minimal centering/positioning styles are applied to `AlertDialogContent` (position, top, left, transform, z-index). All appearance is controlled by your CSS.
- The dialog is accessible and closes on Escape, backdrop click, or Cancel/Action.
- You can pass custom `on_click`, `class`, and `style` props to all subcomponents for full control.
- Focus trap is not fully implemented; focus may escape the dialog in some cases.
