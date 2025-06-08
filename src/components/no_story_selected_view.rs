use crate::components::primitives::Spacer;
use crate::theme::Theme;
use freya::prelude::*;

#[derive(Props, PartialEq, Clone, Copy)]
pub struct NoStorySelectedViewProps {
  pub on_back: EventHandler<()>,
}

#[component]
pub fn NoStorySelectedView(props: NoStorySelectedViewProps) -> Element {
  let theme = use_context::<Theme>();

  rsx! {
      rect {
          width: "100%",
          height: "fill",
          direction: "vertical",
          main_align: "center",
          cross_align: "center",

          label {
              font_family: "{theme.font.sans}",
              "No story selected or data is missing."
          }
          Spacer { height: "15" }
          Button {
              onclick: move |_| props.on_back.call(()),
              label {
                  // Use sans font for button text.
                  font_family: "{theme.font.sans}",
                  "← Back to List"
              }
          }
      }
  }
}
