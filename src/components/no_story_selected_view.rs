use crate::components::primitives::Spacer;
use freya::prelude::*;

/// A view to display when no story is selected.
#[derive(Props, PartialEq, Clone, Copy)]
pub struct NoStorySelectedViewProps {
  pub on_back: EventHandler<()>,
}

#[component]
pub fn NoStorySelectedView(props: NoStorySelectedViewProps) -> Element {
  rsx! {
      rect {
          width: "100%",
          height: "fill",
          direction: "vertical",
          main_align: "center",
          cross_align: "center",

          label {
              "No story selected or data is missing."
          }
          Spacer { height: "15" }
          Button {
              onclick: move |_| props.on_back.call(()),
              label { "← Back to List" }
          }
      }
  }
}
