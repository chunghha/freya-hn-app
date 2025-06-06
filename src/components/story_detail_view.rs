use crate::components::icons::*;
use crate::components::info_line::InfoLine;
use crate::components::primitives::Spacer;
use crate::utils::datetime::format_timestamp;
use crate::Story;
use freya::prelude::*;

#[component]
pub fn StoryDetailView(story_data: Signal<Option<Story>>, on_back: EventHandler<()>) -> Element {
    const DETAIL_PADDING: &str = "15";
    const DETAIL_BG: &str = "rgb(250, 250, 250)";
    const TITLE_FONT_SIZE: &str = "22";
    const TITLE_FONT_WEIGHT: &str = "bold";
    const TITLE_PLACEHOLDER: &str = "[No Title]";
    const URL_FONT_SIZE: &str = "14";
    const URL_COLOR: &str = "rgb(0, 0, 200)";
    const INFO_FONT_SIZE: &str = "14";
    const VERTICAL_SPACER_HEIGHT: &str = "12";
    const COMMENTS_SECTION_SPACER: &str = "20";
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
                Spacer { height: VERTICAL_SPACER_HEIGHT }

                label {
                    font_size: TITLE_FONT_SIZE,
                    font_weight: TITLE_FONT_WEIGHT,
                    "{story.title.as_deref().unwrap_or(TITLE_PLACEHOLDER)}"
                }
                Spacer { height: VERTICAL_SPACER_HEIGHT }

                if let Some(url) = &story.url {
                    Link {
                        to: url.clone(),
                        label {
                            font_size: URL_FONT_SIZE,
                            color: URL_COLOR,
                            "URL: {url}"
                        }
                    }
                    Spacer { height: VERTICAL_SPACER_HEIGHT }
                }

                InfoLine {
                    icon: rsx!{ IconScore {} },
                    text: format!("Score: {}", story.score.unwrap_or(0))
                }
                Spacer { height: VERTICAL_SPACER_HEIGHT }

                InfoLine {
                    icon: rsx!{ IconUser {} },
                    text: format!("By: {}", story.by.as_deref().unwrap_or("N/A"))
                }
                Spacer { height: VERTICAL_SPACER_HEIGHT }

                if let Some(time) = &story.time {
                    InfoLine {
                        icon: rsx!{ IconTime {} },
                        text: format!("Time: {}", format_timestamp(time))
                    }
                    Spacer { height: VERTICAL_SPACER_HEIGHT }
                }

                InfoLine {
                    icon: rsx!{ IconComments {} },
                    text: format!("Comments: {}", story.descendants.unwrap_or(0))
                }

                Spacer { height: COMMENTS_SECTION_SPACER }
                label {
                    font_size: COMMENTS_TITLE_FONT_SIZE,
                    font_weight: COMMENTS_TITLE_FONT_WEIGHT,
                    "Comments:"
                }
                label {
                    font_size: INFO_FONT_SIZE,
                    color: COMMENTS_PLACEHOLDER_COLOR,
                    "(Comment fetching not yet implemented)"
                }
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
                Spacer { height: "15" }
                Button {
                    onclick: move |_| on_back.call(()),
                    label { "← Back to List" }
                }
            }
        }
    }
}
