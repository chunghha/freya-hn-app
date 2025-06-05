use crate::components::indication_label::IndicationLabel;
use crate::components::story_card::StoryCard;
use crate::{CurrentView, Story};
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
    const ERROR_COLOR: &str = "red";
    const ERROR_FONT_SIZE: &str = "16";
    const ERROR_PADDING: &str = "10";
    const CONTAINER_WIDTH: &str = "100%";
    const SCROLL_HEIGHT: &str = "fill";
    const LOADING_TEXT: &str = "Loading best story IDs...";
    const FETCHING_TEXT: &str = "Fetching story details...";
    const LOADING_MORE_TEXT: &str = "Loading more stories...";

    let content = if let Some(err) = error_signal.read().as_ref() {
        rsx! {
            rect {
                width: CONTAINER_WIDTH,
                padding: ERROR_PADDING,
                main_align: "center",
                label {
                    color: ERROR_COLOR,
                    font_size: ERROR_FONT_SIZE,
                    "Error: {err}"
                }
            }
        }
    } else {
        match best_story_ids_resource.value().read().as_ref() {
            None => rsx! {
                IndicationLabel { text: LOADING_TEXT.to_string() }
            },
            Some(Err(err)) => rsx! {
                rect {
                    width: CONTAINER_WIDTH,
                    padding: ERROR_PADDING,
                    main_align: "center",
                    label {
                        color: ERROR_COLOR,
                        font_size: ERROR_FONT_SIZE,
                        "Error: {err}"
                    }
                }
            },
            Some(Ok(_)) => {
                let stories = stories_signal.read();
                if stories.is_empty() && !*is_loading_more.read() {
                    rsx! {
                        IndicationLabel { text: FETCHING_TEXT.to_string() }
                    }
                } else {
                    rsx! {
                        for story_item in stories.iter() {
                            StoryCard {
                                story: story_item.clone(),
                                // CORRECTED: Clone the story *inside* the `move` closure.
                                // This ensures the closure is `FnMut` because it doesn't
                                // consume its environment.
                                on_select: {
                                    let mut selected_story_data = selected_story_data;
                                    let mut current_view = current_view;
                                    let story = story_item.clone();
                                    move |_| {
                                        selected_story_data.set(Some(story.clone()));
                                        current_view.set(CurrentView::Detail);
                                    }
                                }
                            }
                        }
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
