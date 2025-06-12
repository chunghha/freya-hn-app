use crate::theme::Theme;
use freya::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct IndicationLabelProps {
  #[props(into)]
  pub text: String,
  #[props(default = "rgb(60, 60, 60)".to_string(), into)]
  pub color: String,
  #[props(default = 16u32, into)]
  pub font_size: u32,
  #[props(default = 10u32, into)]
  pub padding: u32,
}

#[component]
pub fn IndicationLabel(props: IndicationLabelProps) -> Element {
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();

  rsx! {
      rect {
          width: "100%",
          height: "auto",
          padding: "{props.padding}",
          main_align: "center",
          cross_align: "center",
          label {
              font_family: "{theme.font.sans}",
              font_size: "{props.font_size}",
              color: "{props.color}",
              "{props.text}"
          }
      }
  }
}
