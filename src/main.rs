#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use freya::prelude::*;
use futures::stream::{self, StreamExt};
use log::{error, info};
use std::sync::Arc;

// --- Module Declarations ---
mod components;
mod models;
mod theme;
mod utils;

// --- Imports ---
use components::{
  StoryDetailView, StoryListView, StoryTab,
  icons::{IconMoon, IconSun},
  primitives::{IconButton, Spacer},
};
use freya::prelude::{ScrollDirection, ScrollPosition};
use models::Story;
use theme::{Theme, ThemeMode};
use utils::api::{ApiService, StoryListType};

// --- Application Constants ---
const BATCH_SIZE: usize = 20;
const SCROLL_END_MARGIN: i32 = 150;
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

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
  let mut current_list_type = use_signal(|| StoryListType::Best);
  let mut theme_mode = use_signal(|| ThemeMode::Light);

  // --- Service and Theme Instantiation and Context ---
  let api_service = Arc::new(ApiService::new());
  use_context_provider(|| api_service.clone());

  let mut theme_signal = use_signal(Theme::light);
  use_context_provider(|| theme_signal);

  // This makes the `theme_mode` signal available to all children.
  use_context_provider(|| theme_mode);

  // This effect will update the theme_signal whenever theme_mode changes.
  use_effect(move || {
    let new_theme = if *theme_mode.read() == ThemeMode::Light { Theme::light() } else { Theme::dark() };
    theme_signal.set(new_theme);
  });

  // --- Hooks ---
  let mut scroll_controller = use_scroll_controller(ScrollConfig::default);

  use_effect(move || {
    current_list_type.read();
    info!("List type changed, resetting state.");
    stories_signal.set(vec![]);
    loaded_count.set(BATCH_SIZE);
    error_signal.set(None);
    scroll_controller.scroll_to(ScrollPosition::Start, ScrollDirection::Vertical);
  });

  let mut story_ids_resource = {
    let api_service = api_service.clone();
    use_resource(move || {
      let list_type = *current_list_type.read();
      let service = api_service.clone();
      async move {
        info!("Fetching story IDs for: {list_type}");
        let result = service.fetch_story_ids(list_type).await;
        if result.is_ok() {
          info!("Successfully fetched story IDs for: {list_type}");
        }
        result
      }
    })
  };

  let _ = {
    let api_service = api_service.clone();
    use_resource(move || {
      let current_best_ids = story_ids_resource.value().read().as_ref().cloned();
      let loaded_count_val = *loaded_count.read();
      let already_loaded = stories_signal.read().len();
      let api_service = api_service.clone();
      async move {
        if let Some(Ok(ids)) = current_best_ids {
          if already_loaded < loaded_count_val && already_loaded < ids.len() {
            is_loading_more.set(true);
            let ids_to_fetch =
              ids.iter().skip(already_loaded).take(loaded_count_val - already_loaded).cloned().collect::<Vec<_>>();
            info!("Fetching {} story details in parallel...", ids_to_fetch.len());
            let stories_futures = ids_to_fetch.into_iter().map(|id| {
              let api_service = api_service.clone();
              async move { api_service.fetch_story_content(id).await }
            });
            let results = stream::iter(stories_futures).buffer_unordered(10).collect::<Vec<_>>().await;
            let mut new_stories = Vec::new();
            for result in results {
              match result {
                Ok(story) => new_stories.push(story),
                Err(e) => {
                  let err_msg = format!("Failed to fetch/parse story: {}", e);
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
    })
  };

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
      if let Some(Ok(ids)) = story_ids_resource.value().read().as_ref() {
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
  let theme = theme_signal.read();
  rsx! {
      rect {
          width: "100%",
          height: "100%",
          direction: "vertical",
          background: "{theme.color.background_page}",
          color: "{theme.color.base}",
          padding: "10",

          // Header
          rect {
              width: "100%",
              height: "50",
              background: "{theme.color.accent}",
              direction: "horizontal",
              main_align: "space-between",
              cross_align: "center",
              padding: "10",
              rect {
                  direction: "horizontal",
                  cross_align: "center",
                  label {
                      font_family: "{theme.font.mono}",
                      font_size: "{theme.size.text_header}",
                      font_weight: "{theme.font_weight.bold}",
                      color: "{theme.color.accent_text}",
                      "Hacker News"
                  }
                  Spacer { width: "12" }
                  label {
                      font_family: "{theme.font.mono}",
                      font_size: "{theme.size.text_s}",
                      color: "{theme.color.accent_text}",
                      "v{APP_VERSION}"
                  }
              }
              rect {
                  direction: "horizontal",
                  cross_align: "center",
                  IconButton {
                      onclick: move |_| {
                          let new_mode = if *theme_mode.read() == ThemeMode::Light {
                              ThemeMode::Dark
                          } else {
                              ThemeMode::Light
                          };
                          theme_mode.set(new_mode);
                      },
                      icon: rsx! {
                          if *theme_mode.read() == ThemeMode::Light {
                              IconMoon {}
                          } else {
                              IconSun {}
                          }
                      }
                  }
                  Spacer { width: "8" }
                  if story_ids_resource.value().read().is_none() {
                      label { font_size: "{theme.size.text_xl}", "⏳" }
                  } else {
                      IconButton {
                          onclick: move |_| {
                              info!("Refreshing story list...");
                              story_ids_resource.restart();
                          },
                          icon: rsx! { label { font_size: "{theme.size.text_xl}", color: "{theme.color.accent_text}", "🔄" } }
                      }
                  }
              }
          }

          // Viewport
          if *current_view.read() == CurrentView::List {
              rect {
                  width: "100%",
                  height: "100%",
                  direction: "vertical",

                  // Tabs for selecting the story list type
                  rect {
                      width: "100%",
                      height: "auto",
                      direction: "horizontal",
                      padding: "6",
                      background: "{theme.color.background_card}",
                      main_align: "space-around",
                      border: "1 solid {theme.color.border}",
                      corner_radius: "6",

                      for list_type in [StoryListType::Best, StoryListType::Top, StoryListType::New, StoryListType::Ask, StoryListType::Show, StoryListType::Job] {
                          StoryTab {
                              title: list_type.to_string(),
                              is_active: *current_list_type.read() == list_type,
                              onclick: move |_| {
                                  if *current_list_type.read() != list_type {
                                      current_list_type.set(list_type);
                                  }
                              }
                          }
                      }
                  }

                  // The actual list of stories
                  StoryListView {
                      stories_signal,
                      error_signal,
                      best_story_ids_resource: story_ids_resource,
                      loaded_count,
                      is_loading_more,
                      current_view,
                      selected_story_data,
                      scroll_controller,
                  }
              }
          } else {
              // The detail view does not include the tabs.
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
  env_logger::init();
  launch_with_props(app, "Hacker News", (900.0, 900.0));
}
