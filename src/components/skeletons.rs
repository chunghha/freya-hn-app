use crate::components::primitives::Spacer;
use crate::theme::Theme; // Import the Theme
use freya::prelude::*;

#[component]
pub fn CommentSkeleton() -> Element {
  // Consume the theme from the context.
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();

  // Define skeleton colors based on the theme's card background.
  // This creates a subtle, shimmering effect.
  let base_color = &theme.color.background_card;
  let highlight_color = if *use_context::<Signal<crate::theme::ThemeMode>>().read() == crate::theme::ThemeMode::Light {
    "rgb(235, 235, 235)"
  } else {
    "rgb(45, 45, 50)"
  };

  rsx! {
      rect {
          width: "100%",
          padding: "12 8",
          margin: "0 0 8 0",
          background: "{base_color}",
          border: "1 solid {theme.color.border}",

          rect {
              direction: "horizontal",
              cross_align: "center",
              rect { width: "40%", height: "14", background: "{highlight_color}", corner_radius: "4" }
          }
          Spacer { height: "10" }
          rect { width: "90%", height: "12", background: "{highlight_color}", corner_radius: "4" }
          Spacer { height: "6" }
          rect { width: "70%", height: "12", background: "{highlight_color}", corner_radius: "4" }
      }
  }
}
