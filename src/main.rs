#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use futures::stream::{self, StreamExt};
use log::{error, info};

// --- Module Declarations ---
// Declare the modules that make up the application.
mod components;
mod models;
mod utils;

// --- Imports ---
// Import the necessary components and models from our modules.
use components::{StoryDetailView, StoryListView};
use models::Story;
use utils::api::{hn_item_url, HN_BEST_STORIES_URL};

// --- Application Constants ---
const BATCH_SIZE: usize = 20;
const SCROLL_END_MARGIN: i32 = 150;

// --- Top-level Application State ---

#[derive(Clone, PartialEq)]
enum CurrentView {
    List,
    Detail,
}

// --- Main App Component ---

fn app() -> Element {
    // --- State Signals ---
    let mut stories_signal: Signal<Vec<Story>> = use_signal(Vec::new);
    let mut error_signal: Signal<Option<String>> = use_signal(|| None);
    let mut current_view = use_signal(|| CurrentView::List);
    let selected_story_data: Signal<Option<Story>> = use_signal(|| None);
    let mut loaded_count: Signal<usize> = use_signal(|| BATCH_SIZE);
    let mut is_loading_more: Signal<bool> = use_signal(|| false);

    // --- Hooks ---
    let scroll_controller = use_scroll_controller(ScrollConfig::default);

    // Resource to fetch the initial list of best story IDs.
    let best_story_ids_resource = use_resource(|| async move {
        async fn fetch() -> Result<Vec<u32>, String> {
            info!("Fetching best story IDs...");
            let response = reqwest::get(HN_BEST_STORIES_URL)
                .await
                .map_err(|e| format!("Failed to fetch best story IDs: {}", e))?;
            let ids = response
                .json::<Vec<u32>>()
                .await
                .map_err(|e| format!("Failed to parse story IDs: {}", e))?;
            info!("Successfully fetched {} story IDs", ids.len());
            Ok(ids)
        }
        fetch().await
    });

    // Resource to fetch story details in batches.
    let _ = use_resource({
        move || {
            let current_best_ids = best_story_ids_resource.value().read().as_ref().cloned();
            let loaded_count_val = *loaded_count.read();
            let already_loaded = stories_signal.read().len();

            async move {
                if let Some(Ok(ids)) = current_best_ids {
                    if already_loaded < loaded_count_val && already_loaded < ids.len() {
                        is_loading_more.set(true);

                        let ids_to_fetch = ids
                            .iter()
                            .skip(already_loaded)
                            .take(loaded_count_val - already_loaded)
                            .cloned()
                            .collect::<Vec<_>>();

                        info!(
                            "Fetching {} story details in parallel...",
                            ids_to_fetch.len()
                        );

                        let stories_futures = ids_to_fetch.into_iter().map(|id| async move {
                            let story_url = hn_item_url(id);
                            match reqwest::get(&story_url).await {
                                Ok(response) => response
                                    .json::<Story>()
                                    .await
                                    .map_err(|e| (id, e.to_string())),
                                Err(e) => Err((id, e.to_string())),
                            }
                        });

                        let results = stream::iter(stories_futures)
                            .buffer_unordered(10)
                            .collect::<Vec<_>>()
                            .await;

                        let mut new_stories = Vec::new();
                        for result in results {
                            match result {
                                Ok(story) => new_stories.push(story),
                                Err((id, e)) => {
                                    let err_msg =
                                        format!("Failed to fetch/parse story {}: {}", id, e);
                                    error!("{}", err_msg);
                                    error_signal.set(Some(err_msg));
                                }
                            }
                        }

                        if !new_stories.is_empty() {
                            stories_signal.write().extend(new_stories);
                        }

                        is_loading_more.set(false);
                    }
                }
            }
        }
    });

    // Effect for infinite scrolling.
    use_effect(move || {
        let y = scroll_controller.y();
        let layout = scroll_controller.layout();
        let y_val = *y.read();
        let layout_val = layout.read();
        let end = layout_val.inner.height - layout_val.area.height();

        if !*is_loading_more.read()
            && layout_val.inner.height > layout_val.area.height()
            && -y_val > end as i32 - SCROLL_END_MARGIN
        {
            if let Some(Ok(ids)) = best_story_ids_resource.value().read().as_ref() {
                let current = *loaded_count.read();
                let next = (current + BATCH_SIZE).min(ids.len());
                if next > current {
                    info!("Infinite scroll triggered: loading up to {} stories.", next);
                    loaded_count.set(next);
                }
            }
        }
    });

    // --- Render ---
    rsx! {
        rect {
            width: "100%",
            height: "100%",
            direction: "vertical",
            background: "rgb(246, 246, 239)",
            color: "black",
            padding: "10",

            // Header
            rect {
                width: "100%",
                height: "50",
                background: "rgb(255, 102, 0)",
                direction: "horizontal",
                main_align: "center",
                cross_align: "center",
                padding: "10",
                label {
                    font_size: "24",
                    font_weight: "bold",
                    color: "white",
                    "Hacker News Best Stories"
                }
            }

            // Viewport: Switches between the list and detail views.
            if *current_view.read() == CurrentView::List {
                StoryListView {
                    stories_signal,
                    error_signal,
                    best_story_ids_resource,
                    loaded_count,
                    is_loading_more,
                    current_view,
                    selected_story_data,
                    scroll_controller,
                }
            } else {
                StoryDetailView {
                    story_data: selected_story_data,
                    on_back: move |_| {
                        current_view.set(CurrentView::List);
                    }
                }
            }
        }
    }
}

// --- Application Entry Point ---

fn main() {
    // Initialize the logger. Run with `RUST_LOG=info cargo run` to see logs.
    env_logger::init();
    launch_with_props(app, "Hacker News", (900.0, 900.0));
}
