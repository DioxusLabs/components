# Avatar

The Avatar component is used to display a user's profile picture or an icon representing the user. It handles the loading state of the image and can display a fallback icon if the image fails to load.

## Component Structure

```rust
// All avatar contents must be wrapped in the Avatar component.
Avatar {
    on_state_change: |state: AvatarState| {
        // This callback is triggered when the avatar's state changes. The state can be used to determine if the image is loading, loaded, or failed to load.
    },
    // The avatar image component is used to display the user's profile picture.
    AvatarImage {
        // The source URL of the image to be displayed.
        src: "",
        // The alt text for the image, used for accessibility.
        alt: "",
    }
    // The avatar fallback component is used to display an icon or text when the image fails to load.
    AvatarFallback {
        // The content to display when the image fails to load.
        // This can be an icon or text representing the user.
        {children}
    }
}
```
