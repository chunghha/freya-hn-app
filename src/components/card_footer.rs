use crate::components::footer_label::FooterLabel;
use crate::components::icons::*;
use crate::components::primitives::Spacer;
use crate::utils::datetime::format_timestamp;
use crate::Story;
use freya::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CardFooterProps {
    pub story: Story,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    let story = &props.story;

    const FOOTER_PADDING_TOP: &str = "10";
    const SPACER_WIDTH: &str = "16";

    let mut footer_items: Vec<(Element, String)> = vec![
        (
            rsx! { IconScore {} },
            format!("{}", story.score.unwrap_or(0)),
        ),
        (
            rsx! { IconUser {} },
            story.by.as_deref().unwrap_or("N/A").to_string(),
        ),
    ];

    if let Some(time) = &story.time {
        footer_items.push((rsx! { IconTime {} }, format_timestamp(time)));
    }

    footer_items.push((
        rsx! { IconComments {} },
        format!("{}", story.descendants.unwrap_or(0)),
    ));

    rsx! {
        ScrollView {
            direction: "horizontal",
            width: "100%",
            height: "auto",
            show_scrollbar: true,
            padding: "{FOOTER_PADDING_TOP} 0 0 0",

            rect {
                direction: "horizontal",
                cross_align: "center",
                height: "auto",

                {
                    footer_items.into_iter().enumerate().map(|(i, (icon, text))| {
                        rsx! {
                            Fragment {
                                if i > 0 {
                                    Spacer { width: SPACER_WIDTH }
                                }
                                FooterLabel {
                                    icon: Some(icon),
                                    text: text
                                }
                            }
                        }
                    })
                }
            }
        }
    }
}
