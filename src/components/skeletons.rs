use crate::components::primitives::Spacer;
use freya::prelude::*;

/// A placeholder component that mimics the layout of a comment while it's loading.
#[component]
pub fn CommentSkeleton() -> Element {
  const SKELETON_BG: &str = "rgb(240, 240, 240)";
  const PLACEHOLDER_BG: &str = "rgb(220, 220, 220)";

  rsx! {
      rect {
          width: "100%",
          height: "auto",
          direction: "vertical",
          padding: "12",
          margin: "0 0 8 0",
          background: SKELETON_BG,
          corner_radius: "6",

          // Placeholder for the user/meta info line
          rect {
              width: "40%",
              height: "14",
              background: PLACEHOLDER_BG,
              corner_radius: "4",
          }
          Spacer { height: "10" }

          // Placeholders for the comment text
          rect {
              width: "90%",
              height: "14",
              background: PLACEHOLDER_BG,
              corner_radius: "4",
          }
          Spacer { height: "6" }
          rect {
              width: "70%",
              height: "14",
              background: PLACEHOLDER_BG,
              corner_radius: "4",
          }
      }
  }
}
