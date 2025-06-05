# Freya Hacker News Best Stories

A modern, desktop-style Hacker News "Best Stories" reader built with [Freya](https://github.com/marc2332/freya) (a Dioxus-based GUI framework for Rust). This app demonstrates infinite scrolling, story detail views, and a modular component architecture.

---

## Features

- **Infinite Scroll:** Loads 20 stories at a time as you scroll, for a smooth, native-like experience.
- **Story Detail View:** Click any story to see its details, including a clickable URL that opens in your browser.
- **Error Handling:** Graceful error messages for network or parsing issues.
- **Component-based:** Clean separation of concerns with reusable components (`StoryCard`, `StoryDetailView`, `FooterLabel`, etc).
- **Debug Logging:** Enable detailed logs with an environment variable.

---

## Screenshots

![Screenshot of Freya Hacker News Best Stories](screenshots/ss01.png)

---

## Getting Started

### Prerequisites

- Rust (latest stable recommended)
- [Taskfile](https://github.com/go-task/task) (optional, for easier workflow)

### Install dependencies

```sh
cargo fetch
```

### Run the app

```sh
cargo run --release
```

### Enable debug logging

```sh
FREYA_DEBUG=1 cargo run
```

---

## Project Structure

```
freya-hn-app/
├── src/
│   ├── main.rs                # App entry point
│   ├── components/            # All UI components
│   │   ├── story_card.rs
│   │   ├── story_list_view.rs
│   │   ├── story_detail_view.rs
│   │   ├── indication_label.rs
│   │   ├── footer_label.rs
│   │   └── mod.rs
│   └── utils/
│       ├── datetime.rs        # Helper for formatting timestamps
│       └── mod.rs
├── Cargo.toml
└── README.md
```

---

## Key Dependencies

- [freya](https://github.com/marc2332/freya) - GUI framework for Rust
- [dioxus](https://dioxuslabs.com/) - Declarative UI library
- [reqwest](https://docs.rs/reqwest/) - HTTP client for fetching stories
- [serde](https://serde.rs/) - Serialization/deserialization
- [once_cell](https://docs.rs/once_cell/) - For runtime debug flag

---

## How It Works

- Fetches the list of "best stories" from the [Hacker News API](https://github.com/HackerNews/API).
- Loads story details in batches of 20 as the user scrolls.
- Clicking a story opens a detail view, with a clickable link to the original story.
- All status and error messages are handled with consistent UI components.

---

## Customization

- **Change batch size:** Edit the `loaded_count` logic in `main.rs`.
- **Add more views:** Create new components in `src/components/`.
- **Change styling:** Adjust inline styles in each component.

---

## License

MIT

---

## Credits

- [Freya](https://github.com/marc2332/freya) and [Dioxus](https://dioxuslabs.com/) teams
- [Hacker News API](https://github.com/HackerNews/API)
