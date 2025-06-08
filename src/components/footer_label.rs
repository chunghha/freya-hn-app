use crate::components::primitives::Spacer;
use crate::theme::Theme;
use freya::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FooterLabelProps {
  pub text: String,
  #[props(optional)]
  pub icon: Option<Element>,
}

#[component]
pub fn FooterLabel(props: FooterLabelProps) -> Element {
  let theme = use_context::<Theme>();
  const ICON_TEXT_SPACING: &str = "4";

  rsx! {
      rect {
          direction: "horizontal",
          cross_align: "center",
          if let Some(icon) = props.icon {
              Fragment {
                  {icon}
                  Spacer { width: ICON_TEXT_SPACING }
              }
          }
          label {
              // Use the sans font for all metadata.
              font_family: "{theme.font.sans}",
              font_size: "{theme.size.text_s}",
              color: "{theme.color.text}",
              "{props.text}"
          }
      }
  }
}
