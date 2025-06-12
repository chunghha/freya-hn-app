use crate::components::primitives::Spacer;
use crate::theme::Theme;
use freya::prelude::*;

/// A view to display when no story is selected.
#[derive(Props, PartialEq, Clone, Copy)]
pub struct NoStorySelectedViewProps {
  pub on_back: EventHandler<()>,
}

#[component]
pub fn NoStorySelectedView(props: NoStorySelectedViewProps) -> Element {
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();
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
          rect {
              onclick: move |_| props.on_back.call(()),
              background: "{theme.color.background_page}",
              border: "1 solid {theme.color.border}",
              padding: "6 10",
              corner_radius: "6",
              label {
                  font_family: "{theme.font.sans}",
                  font_size: "{theme.size.text_m}",
                  color: "{theme.color.text}",
                  "← Back to List"
              }
          }
      }
  }
}
