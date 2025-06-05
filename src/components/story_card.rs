use crate::components::footer_label::FooterLabel;
use crate::utils::datetime::format_timestamp;
use crate::Story;
use freya::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct StoryCardProps {
    pub story: Story,
    pub on_select: EventHandler<()>,
}

#[component]
pub fn StoryCard(props: StoryCardProps) -> Element {
    let story = &props.story;

    const CARD_PADDING: &str = "8 16";
    const CARD_MARGIN: &str = "0 0 6 0";
    const CARD_CORNER_RADIUS: &str = "8";
    const CARD_BACKGROUND: &str = "white";
    const CARD_SHADOW: &str = "0 2 8 0 rgb(0,0,0,0.1)";
    const TITLE_FONT_FAMILY: &str = "Inter";
    const TITLE_FONT_SIZE: &str = "18";
    const TITLE_FONT_WEIGHT: &str = "bold";
    const TITLE_COLOR: &str = "black";
    const URL_FONT_SIZE: &str = "14";
    const URL_COLOR: &str = "blue";
    const INFO_DIRECTION: &str = "horizontal";

    rsx! {
        rect {
            key: "{story.id}",
            width: "100%",
            height: "auto",
            direction: "vertical",
            padding: CARD_PADDING,
            margin: CARD_MARGIN,
            corner_radius: CARD_CORNER_RADIUS,
            background: CARD_BACKGROUND,
            shadow: CARD_SHADOW,
            onclick: move |_| props.on_select.call(()),
            label {
                font_family: TITLE_FONT_FAMILY,
                font_size: TITLE_FONT_SIZE,
                font_weight: TITLE_FONT_WEIGHT,
                color: TITLE_COLOR,
                "{story.title.as_deref().unwrap_or(\"[No Title]\")}"
            }
            if let Some(url) = &story.url {
                label {
                    font_size: URL_FONT_SIZE,
                    color: URL_COLOR,
                    "{url}"
                }
            }
            rect {
                width: "100%",
                height: "auto",
                direction: INFO_DIRECTION,
                main_align: "space-between",
                cross_align: "center",
                FooterLabel { text: format!("Score: {}", story.score.unwrap_or(0)) }
                FooterLabel { text: format!("By: {}", story.by.as_deref().unwrap_or("N/A")) }
                if let Some(time) = &story.time {
                    FooterLabel { text: format!("Time: {}", format_timestamp(time)) }
                }
                FooterLabel { text: format!("Comments: {}", story.descendants.unwrap_or(0)) }
            }
        }
    }
}
