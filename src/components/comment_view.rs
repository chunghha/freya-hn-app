use crate::components::icons::{IconTime, IconUser};
use crate::components::primitives::Spacer;
use crate::models::Comment;
use crate::utils::datetime::format_timestamp;
use freya::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CommentViewProps {
    pub comment: Comment,
    pub depth: u16,
    pub on_toggle_expand: EventHandler<u32>,
}

#[component]
pub fn CommentView(props: CommentViewProps) -> Element {
    let comment = &props.comment;
    let has_kids = comment.kids.as_ref().is_some_and(|k| !k.is_empty());
    let comment_id = comment.id;

    const INDENTATION_SIZE: u16 = 20;
    const DELETED_TEXT: &str = "[deleted]";
    const DELETED_COLOR: &str = "rgb(150, 150, 150)";
    const META_FONT_SIZE: &str = "13";
    const META_COLOR: &str = "rgb(80, 80, 80)";
    const TEXT_FONT_SIZE: &str = "14";

    let padding_left = (props.depth * INDENTATION_SIZE).to_string();

    let display_text = if comment.deleted {
        DELETED_TEXT.to_string()
    } else {
        match &comment.text {
            Some(text) => match html2text::from_read(text.as_bytes(), 80) {
                Ok(plain_text) => plain_text,
                Err(_) => "[failed to parse comment]".to_string(),
            },
            None => "".to_string(),
        }
    };

    rsx! {
        rect {
            key: "{comment.id}",
            direction: "vertical",
            width: "100%",
            padding: "8 0 8 {padding_left}",
            border: "1 solid rgb(235, 235, 235)",

            rect {
                direction: "horizontal",
                cross_align: "center",
                if has_kids {
                    Button {
                        onclick: move |_| props.on_toggle_expand.call(comment_id),
                        label {
                            font_size: "12",
                            if *comment.is_expanded.read() { "[-]" } else { "[+]" }
                        }
                    }
                    Spacer { width: "8" }
                }
                if !comment.deleted {
                    rect { direction: "horizontal", cross_align: "center", IconUser {}, Spacer { width: "4" }, label { font_size: META_FONT_SIZE, color: META_COLOR, "{comment.by.as_deref().unwrap_or(\"N/A\")}" } }
                    Spacer { width: "12" }
                    if let Some(time) = &comment.time {
                        rect { direction: "horizontal", cross_align: "center", IconTime {}, Spacer { width: "4" }, label { font_size: META_FONT_SIZE, color: META_COLOR, "{format_timestamp(time)}" } }
                    }
                }
            }
            Spacer { height: "6" }

            label {
                font_size: TEXT_FONT_SIZE,
                color: if comment.deleted { DELETED_COLOR } else { "black" },
                "{display_text}"
            }
        }
    }
}
