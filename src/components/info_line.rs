use crate::components::primitives::Spacer;
use freya::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct InfoLineProps {
  pub icon: Element,
  pub text: String,
}

#[component]
pub fn InfoLine(props: InfoLineProps) -> Element {
  const INFO_FONT_SIZE: &str = "16";
  const ICON_TEXT_SPACING: &str = "8";

  rsx! {
      rect {
          direction: "horizontal",
          cross_align: "center",
          height: "auto",
          {props.icon}
          Spacer { width: ICON_TEXT_SPACING }
          label {
              font_size: INFO_FONT_SIZE,
              "{props.text}"
          }
      }
  }
}
