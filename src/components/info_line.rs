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
  let theme = use_context::<Theme>();
  const ICON_TEXT_SPACING: &str = "8";

  rsx! {
      rect {
          direction: "horizontal",
          cross_align: "center",
          height: "auto",
          {props.icon}
          Spacer { width: ICON_TEXT_SPACING }
          label {
              // Use the sans font for all metadata.
              font_family: "{theme.font.sans}",
              font_size: "{theme.size.text_m}",
              color: "{theme.color.text}",
              "{props.text}"
          }
      }
  }
}
