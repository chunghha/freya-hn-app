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
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();

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
              font_family: "{theme.font.sans}",
              font_size: "{theme.size.text_s}",
              color: "{theme.color.text}",
              "{props.text}"
          }
      }
  }
}
