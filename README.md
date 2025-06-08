# Freya Hacker News Client

A high-performance, modern desktop client for Hacker News built with [Freya](https://github.com/marc2332/freya), a Dioxus-based GUI framework for Rust.

This application showcases a robust, scalable architecture featuring on-demand data loading, a centralized API service, a component-based UI with a custom theme, and a highly responsive user experience.

---

## Features

-   **High-Performance Story & Comment Loading:**
    -   **Infinite Scroll:** The main story list loads in batches of 20 as you scroll, keeping initial load times fast.
    -   **On-Demand Comment Threads:** Comment threads are loaded lazily. Top-level comments appear instantly, and replies are fetched only when a user expands a thread, making even stories with thousands of comments feel snappy.
-   **Modern, Component-Based UI:**
    -   Clean separation of concerns with a rich set of reusable components (`StoryCard`, `CommentView`, `IconButton`, etc.).
    -   A fully scrollable detail view for stories and their nested comments.
-   **Centralized Theming:**
    -   A dedicated `theme.rs` module defines all fonts, sizes, and colors, ensuring a consistent look and feel.
    -   The theme is provided via context, making it easy to apply styles globally and paving the way for features like dark mode.
-   **Robust Data Fetching:**
    -   All network requests are handled by a centralized `ApiService`, making the code more maintainable, testable, and organized.
    -   Parallel fetching for story lists and asynchronous fetching for comments.
-   **Interactive UI:**
    -   **Pull to Refresh:** A refresh button in the header allows users to fetch the latest list of stories instantly.
    -   **Collapsible Threads:** Comment threads can be individually expanded and collapsed.
-   **Polished User Experience:**
    -   **Skeleton Loaders:** Smooth skeleton placeholders are shown while comments are loading, improving perceived performance.
    -   **Granular States:** The UI provides detailed feedback for loading and error states within comment threads, including a "Retry" button for failed fetches.
    -   **Icons & Typography:** A clear typographic hierarchy and icons improve scannability and visual appeal.
    -   **Version Display:** The app version from `Cargo.toml` is displayed in the header.

---

## Screenshots

*(**Note:** You should update this with a new screenshot of the polished UI!)*

![Screenshot of Freya Hacker News Client](screenshots/ss01.png)

---

## Getting Started

### Prerequisites

-   Rust (latest stable recommended)

### Install dependencies

```sh
cargo fetch
```

### Run the app

For the best performance, run in release mode:
```sh
cargo run --release
```

### Enable Logging

The application uses the `log` crate. You can control log verbosity with the `RUST_LOG` environment variable.

```sh
# Show info, warn, and error messages
RUST_LOG=info cargo run

# Show only errors
RUST_LOG=error cargo run
```

---

## Project Structure

The project is organized into a clean, modular structure:

```
freya-hn-app/
├── src/
│   ├── main.rs                # App entry point, routing, and top-level state
│   ├── models.rs              # Data structures (Story, Comment, etc.)
│   ├── theme.rs               # Centralized theme (colors, fonts, sizes)
│   ├── components/            # All UI components
│   │   ├── card_footer.rs
│   │   ├── comment_view.rs
│   │   ├── footer_label.rs
│   │   ├── icons.rs
│   │   ├── info_line.rs
│   │   ├── indication_label.rs
│   │   ├── no_story_selected_view.rs
│   │   ├── primitives.rs      # Basic, reusable UI primitives (Spacer, ErrorView)
│   │   ├── skeletons.rs       # Skeleton loader components
│   │   ├── story_card.rs
│   │   ├── story_detail_view.rs
│   │   ├── story_list_view.rs
│   │   └── mod.rs
│   └── utils/
│       ├── api.rs             # Centralized ApiService for all network requests
│       ├── datetime.rs        # Helper for formatting timestamps
│       └── mod.rs
├── Cargo.toml
└── README.md
```

---

## Architectural Highlights

-   **Centralized API Service:** All `reqwest` calls are encapsulated in `utils/api.rs`, providing a single, mockable interface for network interactions.
-   **Context API for Theming:** The `Theme` struct is provided to the entire component tree via `use_context_provider`, allowing any component to access shared styling rules without prop drilling.
-   **Reactive Data Fetching:** Freya's `use_resource` hook is used to manage the lifecycle of asynchronous data, automatically handling loading, success, and error states.
-   **On-Demand State:** The comment system is a prime example of efficient state management. Instead of fetching a massive, deeply nested structure, the app fetches data incrementally based on user interaction, making it highly performant and scalable.

---

## Key Dependencies

-   [freya](https://github.com/marc2332/freya) - GUI framework for Rust
-   [dioxus](https://dioxuslabs.com/) - The declarative UI library powering Freya
-   [tokio](https://tokio.rs/) - Asynchronous runtime
-   [reqwest](https://docs.rs/reqwest/) - Ergonomic HTTP client for fetching data
-   [serde](https://serde.rs/) - Framework for serializing and deserializing Rust data structures
-   [futures](https://docs.rs/futures/) - Utilities for working with asynchronous operations
-   [log](https://docs.rs/log/) & [env_logger](https://docs.rs/env_logger/) - Standard logging infrastructure
-   [jiff](https://docs.rs/jiff/) - Modern and correct date/time library
-   [html2text](https://docs.rs/html2text/) - For converting comment HTML into plain text

---

## Customization

-   **Change Theme:** All colors, fonts, and font sizes can be modified in one place: `src/theme.rs`.
-   **Adjust Fetching Behavior:** Constants like `BATCH_SIZE` can be tweaked in `main.rs`.
-   **Add New Views:** Create new components in `src/components/` and integrate them into the `CurrentView` enum and routing logic in `main.rs`.

---

## License

This project is licensed under the MIT License.
