use crate::components::card_footer::CardFooter;
use crate::Story;
use freya::prelude::*;

// --- Props Definition ---
#[derive(Props, PartialEq, Clone)]
pub struct StoryCardProps {
    pub story: Story,
    pub on_select: EventHandler<u32>,
}

// --- Main Component ---
#[component]
pub fn StoryCard(props: StoryCardProps) -> Element {
    const CARD_PADDING: &str = "12 16";
    const CARD_MARGIN: &str = "0 0 8 0";
    const CARD_CORNER_RADIUS: &str = "8";
    const CARD_BACKGROUND: &str = "white";
    const CARD_SHADOW: &str = "0 2 8 0 rgb(0,0,0,0.1)";
    const TITLE_FONT_FAMILY: &str = "Inter";
    const TITLE_FONT_SIZE: &str = "18";
    const TITLE_FONT_WEIGHT: &str = "bold";
    const TITLE_COLOR: &str = "black";
    const URL_FONT_SIZE: &str = "14";
    const URL_COLOR: &str = "rgb(80, 80, 80)";

    let story_id = props.story.id;

    rsx! {
        rect {
            key: "{props.story.id}",
            width: "100%",
            height: "auto",
            direction: "vertical",
            padding: CARD_PADDING,
            margin: CARD_MARGIN,
            corner_radius: CARD_CORNER_RADIUS,
            background: CARD_BACKGROUND,
            shadow: CARD_SHADOW,
            onclick: move |_| props.on_select.call(story_id),

            label {
                font_family: TITLE_FONT_FAMILY,
                font_size: TITLE_FONT_SIZE,
                font_weight: TITLE_FONT_WEIGHT,
                color: TITLE_COLOR,
                max_lines: "2",
                "{props.story.title.as_deref().unwrap_or(\"[No Title]\")}"
            }

            {
                props.story.url.as_ref().map(|url| rsx! {
                    label {
                        font_size: URL_FONT_SIZE,
                        color: URL_COLOR,
                        max_lines: "1",
                        text_overflow: "ellipsis",
                        "{url}"
                    }
                })
            }

            CardFooter { story: props.story.clone() }
        }
    }
}
