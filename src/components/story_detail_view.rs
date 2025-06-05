use crate::utils::datetime::format_timestamp;
use crate::Story;
use freya::prelude::*;

#[component]
pub fn StoryDetailView(
    story_data: ReadOnlySignal<Option<Story>>,
    on_back: EventHandler<()>,
) -> Element {
    const DETAIL_PADDING: &str = "15";
    const DETAIL_BG: &str = "rgb(250, 250, 250)";
    const TITLE_FONT_SIZE: &str = "22";
    const TITLE_FONT_WEIGHT: &str = "bold";
    const TITLE_PLACEHOLDER: &str = "[No Title]";
    const URL_FONT_SIZE: &str = "14";
    const URL_COLOR: &str = "rgb(0, 0, 200)";
    const INFO_FONT_SIZE: &str = "14";
    const INFO_LABEL_HEIGHT: &str = "5";
    const COMMENTS_LABEL_HEIGHT: &str = "20";
    const COMMENTS_TITLE_FONT_SIZE: &str = "16";
    const COMMENTS_TITLE_FONT_WEIGHT: &str = "bold";
    const COMMENTS_PLACEHOLDER_COLOR: &str = "rgb(100,100,100)";

    if let Some(story) = story_data.read().as_ref() {
        rsx! {
            rect {
                width: "100%",
                height: "fill",
                padding: DETAIL_PADDING,
                direction: "vertical",
                background: DETAIL_BG,

                Button {
                    onclick: move |_| on_back.call(()),
                    label { "← Back to List" }
                }
                rect { height: INFO_LABEL_HEIGHT }

                if let Some(title) = &story.title {
                    label { font_size: TITLE_FONT_SIZE, font_weight: TITLE_FONT_WEIGHT, "{title}" }
                } else {
                    label { font_size: TITLE_FONT_SIZE, font_weight: TITLE_FONT_WEIGHT, "{TITLE_PLACEHOLDER}" }
                }
                rect { height: INFO_LABEL_HEIGHT }

                if let Some(url) = &story.url {
                    Link {
                        to: url.clone(),
                        label {
                            font_size: URL_FONT_SIZE,
                            color: URL_COLOR,
                            "URL: {url}"
                        }
                    }
                    rect { height: INFO_LABEL_HEIGHT }
                }

                label { font_size: INFO_FONT_SIZE, "Score: {story.score.unwrap_or(0)}" }
                rect { height: INFO_LABEL_HEIGHT }
                label { font_size: INFO_FONT_SIZE, {format!("By: {}", story.by.as_deref().unwrap_or("N/A"))} }
                rect { height: INFO_LABEL_HEIGHT }

                if let Some(time) = &story.time {
                    label { font_size: INFO_FONT_SIZE, "Time: {format_timestamp(time)}" }
                    rect { height: INFO_LABEL_HEIGHT }
                }
                label { font_size: INFO_FONT_SIZE, "Comments: {story.descendants.unwrap_or(0)}" }

                rect { height: COMMENTS_LABEL_HEIGHT }
                label { font_size: COMMENTS_TITLE_FONT_SIZE, font_weight: COMMENTS_TITLE_FONT_WEIGHT, "Comments:"}
                label { font_size: INFO_FONT_SIZE, color: COMMENTS_PLACEHOLDER_COLOR, "(Comment fetching not yet implemented)"}
            }
        }
    } else {
        rsx! {
            rect {
                width: "100%",
                height: "fill",
                main_align: "center",
                cross_align: "center",
                label { "No story selected or data is missing." }
                Button {
                    onclick: move |_| on_back.call(()),
                    label { "Back to List" }
                }
            }
        }
    }
}
