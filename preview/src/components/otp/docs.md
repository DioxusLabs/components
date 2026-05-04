The OneTimePasswordInput component is used to capture a short code (typically a 6-digit
authentication code) into a row of discrete slots. It is built on a single accessible
`<input>` element so paste, browser autofill (`autocomplete="one-time-code"`), IME
composition, and screen readers all continue to work.

## Component Structure

```rust
// The wrapper holds the hidden input and provides shared state to all slots.
OneTimePasswordInput {
    maxlength: 6,
    // A visual grouping of contiguous slots.
    OneTimePasswordGroup {
        // Each slot displays the character at its `index`.
        OneTimePasswordSlot { index: 0 }
        OneTimePasswordSlot { index: 1 }
        OneTimePasswordSlot { index: 2 }
    }
    // Decorative separator placed between groups.
    OneTimePasswordSeparator {}
    OneTimePasswordGroup {
        OneTimePasswordSlot { index: 3 }
        OneTimePasswordSlot { index: 4 }
        OneTimePasswordSlot { index: 5 }
    }
}
```
