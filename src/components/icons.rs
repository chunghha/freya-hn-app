use freya::prelude::*;

const ICON_COLOR: &str = "rgb(80, 80, 80)";
const ICON_SIZE: &str = "16";

#[component]
pub fn IconScore() -> Element {
  rsx! { label { color: ICON_COLOR, font_size: ICON_SIZE, "⭐" } }
}

#[component]
pub fn IconUser() -> Element {
  rsx! { label { color: ICON_COLOR, font_size: ICON_SIZE, "👤" } }
}

#[component]
pub fn IconTime() -> Element {
  rsx! { label { color: ICON_COLOR, font_size: ICON_SIZE, "🕒" } }
}

#[component]
pub fn IconComments() -> Element {
  rsx! { label { color: ICON_COLOR, font_size: ICON_SIZE, "💬" } }
}
