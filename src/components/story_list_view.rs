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
    const CONTAINER_HEIGHT_AUTO: &str = "auto";
    const SCROLL_HEIGHT: &str = "fill";
    const LOADING_TEXT: &str = "Loading best story IDs...";
    const FETCHING_TEXT: &str = "Fetching story details...";
    const LOADING_MORE_TEXT: &str = "Loading more stories...";
    const UNKNOWN_TEXT: &str = "Unknown state or finished loading with no stories.";

    rsx! {
        ScrollView {
            scroll_controller,
            width: CONTAINER_WIDTH,
            height: SCROLL_HEIGHT,
            show_scrollbar: true,
            {
                match (
                    best_story_ids_resource.value().read().as_ref().cloned(),
                    error_signal.read().as_ref().cloned()
                ) {
                    (_, Some(err)) => rsx! {
                        rect {
                            width: CONTAINER_WIDTH,
                            height: CONTAINER_HEIGHT_AUTO,
                            padding: ERROR_PADDING,
                            main_align: "center",
                            cross_align: "center",
                            label {
                                color: ERROR_COLOR,
                                font_size: ERROR_FONT_SIZE,
                                "Error: {err}"
                            }
                        }
                    },
                    (None, _) => rsx! {
                        rect {
                          IndicationLabel {
                              text: LOADING_TEXT.to_string()
                          }
                        }
                    },
                    (Some(Ok(_)), _) if stories_signal.read().is_empty() && error_signal.read().is_none() => rsx! {
                        rect {
                            IndicationLabel {
                                text: FETCHING_TEXT.to_string(),
                            }
                        }
                    },
                    (Some(Ok(_ids)), _) => rsx! {
                        rect {
                            width: CONTAINER_WIDTH,
                            height: CONTAINER_HEIGHT_AUTO,
                            for story_item in stories_signal.read().iter() {
                                StoryCard {
                                    story: story_item.clone(),
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
                    },
                    _ => rsx! {
                        rect {
                            width: CONTAINER_WIDTH,
                            height: CONTAINER_HEIGHT_AUTO,
                            IndicationLabel {
                                text: UNKNOWN_TEXT.to_string(),
                            }
                        }
                    }
                }
            }
        }
    }
}
