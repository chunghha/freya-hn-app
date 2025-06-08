use crate::theme::Theme;
use freya::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct StoryTabProps {
  pub title: String,
  pub is_active: bool,
  pub onclick: EventHandler<()>,
}

#[component]
pub fn StoryTab(props: StoryTabProps) -> Element {
  let theme = use_context::<Theme>();
  let focus = use_focus();
  let is_focused = focus.is_focused();

  let background = if props.is_active {
    "rgb(220, 235, 255)" // Light blue for active
  } else if is_focused {
    "rgb(235, 245, 255)" // Slightly lighter blue for hover
  } else {
    "transparent"
  };
  let color = if props.is_active { "rgb(0, 50, 100)" } else { "{theme.color.text}" };

  rsx! {
      rect {
          onclick: move |_| props.onclick.call(()),
          padding: "8 12",
          corner_radius: "6",
          background: background,
          label {
              font_family: "{theme.font.serif}",
              font_size: "{theme.size.text_l}",
              font_weight: "semibold",
              color: color,
              "{props.title}"
          }
      }
  }
}
