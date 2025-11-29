The card component is a flexible container for grouping related content and actions. It provides a structured layout with optional header, content, and footer sections.

## Component Structure

```rust
// The Card component must wrap all card elements.
Card {
    // CardHeader contains the title, description, and optional action.
    CardHeader {
        // CardTitle displays the main heading.
        CardTitle { "Card Title" }
        // CardDescription provides supporting text.
        CardDescription { "Card description goes here." }
        // CardAction positions action elements (e.g., buttons) in the header.
        CardAction {
            Button { "Action" }
        }
    }
    // CardContent holds the main body content.
    CardContent {
        p { "Main content of the card." }
    }
    // CardFooter contains footer actions or information.
    CardFooter {
        Button { "Submit" }
    }
}
```

## Layout Notes

- When `CardAction` is present inside `CardHeader`, the header automatically switches to a two-column grid layout.
