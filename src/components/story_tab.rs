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
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();
  let focus = use_focus();
  let is_focused = focus.is_focused();

  let background = if props.is_active {
    theme.color.tab_background_active
  } else if is_focused {
    theme.color.tab_background_hover
  } else {
    "transparent"
  };

  let color = if props.is_active { theme.color.tab_text_active } else { theme.color.tab_text_inactive };

  rsx! {
      rect {
          onclick: move |_| props.onclick.call(()),
          padding: "8 12",
          corner_radius: "6",
          background: "{background}",
          label {
              font_family: "{theme.font.serif}",
              font_size: "{theme.size.text_l}",
              font_weight: "{theme.font_weight.semibold}",
              color: "{color}",
              "{props.title}"
          }
      }
  }
}
