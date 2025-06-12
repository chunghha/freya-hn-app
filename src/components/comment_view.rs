use crate::components::icons::{IconTime, IconUser};
use crate::components::indication_label::IndicationLabel;
use crate::components::primitives::Spacer;
use crate::models::{Comment, FetchState};
use crate::theme::Theme;
use crate::utils::datetime::format_timestamp;
use freya::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CommentViewProps {
  pub comment: Comment,
  pub depth: u16,
  pub on_toggle_expand: EventHandler<u32>,
  pub on_retry_fetch: EventHandler<u32>,
}

#[component]
pub fn CommentView(props: CommentViewProps) -> Element {
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();
  let comment = &props.comment;
  let has_kids = comment.kids.as_ref().is_some_and(|k| !k.is_empty());
  let comment_id = comment.id;
  let fetch_state = comment.fetch_state.read().clone();

  const INDENTATION_SIZE: u16 = 20;
  const DELETED_TEXT: &str = "[deleted]";

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
          border: "1 solid {theme.color.border}",

          rect {
              direction: "horizontal",
              cross_align: "center",
              if has_kids {
                  rect {
                      onclick: move |_| props.on_toggle_expand.call(comment_id),
                      background: "{theme.color.background_page}",
                      border: "1 solid {theme.color.border}",
                      padding: "2 6",
                      corner_radius: "4",
                      label {
                          font_family: "{theme.font.sans}",
                          font_size: "{theme.size.text_xs}",
                          color: "{theme.color.text_alt}",
                          if fetch_state == FetchState::Loading {
                              "⏳"
                          } else if *comment.is_expanded.read() {
                              "[-]"
                          } else {
                              "[+]"
                          }
                      }
                  }
                  Spacer { width: "8" }
              }
              if !comment.deleted {
                  rect {
                      direction: "horizontal",
                      cross_align: "center",
                      IconUser {},
                      Spacer { width: "4" },
                      label {
                          font_family: "{theme.font.sans}",
                          font_size: "{theme.size.text_s}",
                          color: "{theme.color.text_alt}",
                          "{comment.by.as_deref().unwrap_or(\"N/A\")}"
                      }
                  }
                  Spacer { width: "12" }
                  if let Some(time) = &comment.time {
                      rect {
                          direction: "horizontal",
                          cross_align: "center",
                          IconTime {},
                          Spacer { width: "4" },
                          label {
                              font_family: "{theme.font.sans}",
                              font_size: "{theme.size.text_s}",
                              color: "{theme.color.text_alt}",
                              "{format_timestamp(time)}"
                          }
                      }
                  }
              }
          }
          Spacer { height: "6" }

          label {
              font_family: "{theme.font.sans}",
              font_size: "{theme.size.text_l}",
              color: if comment.deleted { "{theme.color.text_alt}" } else { "{theme.color.text}" },
              "{display_text}"
          }

          if *comment.is_expanded.read() {
              match fetch_state {
                  FetchState::Loading => rsx!{
                      IndicationLabel { text: "Loading replies...".to_string() }
                  },
                  FetchState::Failed => rsx!{
                      rect {
                          direction: "horizontal",
                          cross_align: "center",
                          padding: "4 0",
                          label { font_family: "{theme.font.sans}", color: "red", "Failed to load replies." }
                          Spacer { width: "8" }
                          // This can remain a default Button as it's simple.
                          Button {
                              onclick: move |_| props.on_retry_fetch.call(comment_id),
                              label {
                                  font_family: "{theme.font.sans}",
                                  "Retry"
                              }
                          }
                      }
                  },
                  FetchState::Idle => rsx!{ Fragment {} }
              }
          }
      }
  }
}
