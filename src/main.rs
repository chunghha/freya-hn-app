#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use jiff::Timestamp;
use serde::Deserialize;

mod utils;

use once_cell::sync::Lazy;
use std::env;

static DEBUG: Lazy<bool> = Lazy::new(|| {
    env::var("FREYA_DEBUG")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
});

macro_rules! debug_log {
    ($($arg:tt)*) => {
        if *DEBUG {
            println!($($arg)*);
        }
    };
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Story {
    pub id: u32,
    pub title: Option<String>,
    pub url: Option<String>,
    pub by: Option<String>,
    pub score: Option<u32>,
    #[serde(default, with = "jiff::fmt::serde::timestamp::second::optional")]
    pub time: Option<Timestamp>,
    pub descendants: Option<u32>,
    pub kids: Option<Vec<u32>>,
}

#[derive(Clone, PartialEq)]
enum CurrentView {
    List,
    Detail,
}

mod components;
use components::story_detail_view::StoryDetailView;
use components::story_list_view::StoryListView;

fn app() -> Element {
    let stories_signal: Signal<Vec<Story>> = use_signal(Vec::new);
    let error_signal: Signal<Option<String>> = use_signal(|| None);
    let mut current_view = use_signal(|| CurrentView::List);
    let selected_story_data: Signal<Option<Story>> = use_signal(|| None);
    let loaded_count: Signal<usize> = use_signal(|| 20);

    let scroll_controller = use_scroll_controller(ScrollConfig::default);

    const HN_BEST_STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/beststories.json";
    const HN_ITEM_URL_BASE: &str = "https://hacker-news.firebaseio.com/v0/item/";

    fn hn_item_url(id: u32) -> String {
        format!("{}{}.json", HN_ITEM_URL_BASE, id)
    }

    let best_story_ids_resource = use_resource(|| async move {
        let url = HN_BEST_STORIES_URL;
        println!("Fetching best story IDs...");
        match reqwest::get(url).await {
            Ok(response) => match response.json::<Vec<u32>>().await {
                Ok(ids) => {
                    println!("Fetched best story IDs: {:?}", ids);
                    Ok(ids)
                }
                Err(e) => {
                    println!("Failed to parse story IDs: {}", e);
                    Err(format!("Failed to parse story IDs: {}", e))
                }
            },
            Err(e) => {
                println!("Failed to fetch best story IDs: {}", e);
                Err(format!("Failed to fetch best story IDs: {}", e))
            }
        }
    });

    let is_loading_more: Signal<bool> = use_signal(|| false);

    let _ = use_resource({
        let mut stories_signal = stories_signal;
        let mut error_signal = error_signal;
        let mut is_loading_more = is_loading_more;

        move || {
            let current_best_ids = best_story_ids_resource.value().read().as_ref().cloned();
            let loaded_count_val = *loaded_count.read();
            let already_loaded = stories_signal.read().len();

            async move {
                if let Some(Ok(ids)) = current_best_ids {
                    if already_loaded < loaded_count_val && already_loaded < ids.len() {
                        is_loading_more.set(true);
                        debug_log!("Starting to fetch story details. loaded_count: {}, already_loaded: {}, ids.len(): {}", loaded_count_val, already_loaded, ids.len());
                        let mut new_stories = Vec::new();
                        for &id in ids
                            .iter()
                            .skip(already_loaded)
                            .take((loaded_count_val - already_loaded).min(20))
                        {
                            debug_log!("Fetching story with id: {}", id);
                            let story_url = hn_item_url(id);
                            match reqwest::get(&story_url).await {
                                Ok(response) => match response.json::<Story>().await {
                                    Ok(story) => {
                                        debug_log!("Fetched story: {:?}", story);
                                        new_stories.push(story)
                                    }
                                    Err(e) => {
                                        debug_log!("Failed to parse story {}: {}", id, e);
                                        error_signal.set(Some(format!(
                                            "Failed to parse story {}: {}",
                                            id, e
                                        )));
                                        break;
                                    }
                                },
                                Err(e) => {
                                    debug_log!("Failed to fetch story {}: {}", id, e);
                                    error_signal
                                        .set(Some(format!("Failed to fetch story {}: {}", id, e)));
                                    break;
                                }
                            }
                        }
                        let mut all_stories = stories_signal.read().clone();
                        all_stories.extend(new_stories);
                        debug_log!("Setting stories_signal with {} stories", all_stories.len());
                        stories_signal.set(all_stories);
                        error_signal.set(None);
                        is_loading_more.set(false);
                    }
                }
            }
        }
    });

    {
        let mut loaded_count = loaded_count;
        use_effect(move || {
            let y = scroll_controller.y();
            let layout = scroll_controller.layout();
            let y = y.read();
            let layout = layout.read();
            let end = layout.inner.height - layout.area.height();
            const MARGIN: i32 = 100;

            if layout.inner.height > layout.area.height() && -*y > end as i32 - MARGIN {
                if let Some(Ok(ids)) = best_story_ids_resource.value().read().as_ref() {
                    let current = *loaded_count.read();
                    let next = (current + 20).min(ids.len());
                    if next > current {
                        println!(
                            "Infinite scroll: loaded_count before: {}, after: {}, ids.len(): {}",
                            current,
                            next,
                            ids.len()
                        );
                        loaded_count.set(next);
                    }
                }
            }
        });
    }

    rsx! {
        rect {
            width: "100%",
            height: "100%",
            direction: "vertical",
            background: "rgb(246, 246, 239)",
            color: "black",
            padding: "10",

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

fn main() {
    launch_with_props(app, "Hackers News", (900.0, 900.0));
}
