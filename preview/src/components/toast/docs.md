# Toast

The Toast component is used to display brief messages to the user, typically for notifications or alerts.

## Component Structure

```rust
// The Toast provider provides the toast context to its children and handler rendering any toasts that are sent.
ToastProvider {
    // Any child component can consume the toast context and send a toast to be rendered.
    button {
        onclick: |event: MouseEvent| {
            // Consume the toast context to send a toast.
            let toast_api = consume_toast();
            toast_api
                .error(
                    "Critical Error".to_string(),
                    Some(ToastOptions {
                        permanent: true,
                        ..Default::default()
                    }),
                );
        },
        "Show Toast"
    }
}
```
