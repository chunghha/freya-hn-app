use crate::components::primitives::Spacer;
use crate::theme::Theme;
use freya::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct InfoLineProps {
  pub icon: Element,
  pub text: String,
}

#[component]
pub fn InfoLine(props: InfoLineProps) -> Element {
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();

  const ICON_TEXT_SPACING: &str = "8";

  rsx! {
      rect {
          direction: "horizontal",
          cross_align: "center",
          height: "auto",
          {props.icon}
          Spacer { width: ICON_TEXT_SPACING }
          label {
              font_family: "{theme.font.sans}",
              font_size: "{theme.size.text_m}",
              color: "{theme.color.text}",
              "{props.text}"
          }
      }
  }
}
