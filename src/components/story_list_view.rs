use crate::CurrentView;
use crate::components::indication_label::IndicationLabel;
use crate::components::primitives::ErrorView;
use crate::components::story_card::StoryCard;
use crate::models::Story;
use freya::prelude::*;

#[component]
pub fn StoryListView(
  stories_signal: Signal<Vec<Story>>,
  error_signal: Signal<Option<String>>,
  best_story_ids_resource: Resource<Result<Vec<u32>, String>>,
  loaded_count: Signal<usize>,
  is_loading_more: Signal<bool>,
  current_view: Signal<CurrentView>,
  selected_story_data: Signal<Option<Story>>,
  scroll_controller: ScrollController,
) -> Element {
  // --- Constants ---
  const CONTAINER_WIDTH: &str = "100%";
  const SCROLL_HEIGHT: &str = "fill";
  const LOADING_TEXT: &str = "Loading best story IDs...";
  const FETCHING_TEXT: &str = "Fetching story details...";
  const LOADING_MORE_TEXT: &str = "Loading more stories...";
  const NO_STORIES_TEXT: &str = "No stories found.";

  // --- Render Logic ---
  // This block determines what content to show based on the current state
  // of data fetching, prioritizing errors first.
  let content = if let Some(err) = error_signal.read().as_ref() {
    // State 1: A top-level error occurred during story fetching.
    rsx! { ErrorView { message: err.clone() } }
  } else {
    match best_story_ids_resource.value().read().as_ref() {
      // State 2: The initial list of story IDs is still loading.
      None => rsx! { IndicationLabel { text: LOADING_TEXT.to_string() } },

      // State 3: Fetching the list of story IDs failed.
      Some(Err(err)) => rsx! { ErrorView { message: err.clone() } },

      // State 4: The API returned an empty list of stories.
      Some(Ok(ids)) if ids.is_empty() => {
        rsx! { IndicationLabel { text: NO_STORIES_TEXT.to_string() } }
      }

      // State 5: We have story IDs, now render the list or a loading state.
      Some(Ok(_)) => {
        let stories = stories_signal.read();
        if stories.is_empty() && !*is_loading_more.read() {
          // The story list is empty, but we expect stories to be loading.
          rsx! { IndicationLabel { text: FETCHING_TEXT.to_string() } }
        } else {
          // We have stories to display.
          rsx! {
              for story_item in stories.iter() {
                  StoryCard {
                      story: story_item.clone(),
                      // The on_select handler now receives the story's ID.
                      // This is a more flexible pattern than passing the whole story.
                      on_select: {
                          // Clone the story list to be used inside the closure.
                          let stories = stories_signal.read().clone();
                          let mut selected_story_data = selected_story_data;
                          let mut current_view = current_view;
                          move |id: u32| {
                              // Find the full story object using the received ID.
                              if let Some(story) = stories.iter().find(|s| s.id == id) {
                                  selected_story_data.set(Some(story.clone()));
                                  current_view.set(CurrentView::Detail);
                              }
                          }
                      }
                  }
              }
              // Show a loading indicator at the bottom during infinite scroll.
              if *is_loading_more.read() {
                  IndicationLabel {
                      text: LOADING_MORE_TEXT.to_string(),
                  }
              }
          }
        }
      }
    }
  };

  // The root element is a ScrollView that renders the determined content.
  rsx! {
      ScrollView {
          scroll_controller,
          width: CONTAINER_WIDTH,
          height: SCROLL_HEIGHT,
          show_scrollbar: true,
          {content}
      }
  }
}
