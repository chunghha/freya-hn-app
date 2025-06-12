use crate::theme::{Theme, ThemeMode};
use freya::prelude::*;

#[component]
pub fn IconScore() -> Element {
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();
  rsx! { label { color: "{theme.color.text_alt}", font_size: "{theme.size.text_m}", "⭐" } }
}

#[component]
pub fn IconUser() -> Element {
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();
  rsx! { label { color: "{theme.color.text_alt}", font_size: "{theme.size.text_m}", "👤" } }
}

#[component]
pub fn IconTime() -> Element {
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();
  rsx! { label { color: "{theme.color.text_alt}", font_size: "{theme.size.text_m}", "🕒" } }
}

#[component]
pub fn IconComments() -> Element {
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();
  rsx! { label { color: "{theme.color.text_alt}", font_size: "{theme.size.text_m}", "💬" } }
}

#[component]
pub fn IconMoon() -> Element {
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();
  rsx! {
      label {
          font_size: "{theme.size.text_xl}",
          color: "{theme.color.accent_text}",
          "🌙"
      }
  }
}

#[component]
pub fn IconSun() -> Element {
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();
  rsx! {
      label {
          font_size: "{theme.size.text_xl}",
          color: "{theme.color.accent_text}",
          "☀️"
      }
  }
}

#[component]
pub fn IconThemeToggle() -> Element {
  let theme_mode = use_context::<Signal<ThemeMode>>();
  rsx! {
      if *theme_mode.read() == ThemeMode::Light {
          IconMoon {}
      } else {
          IconSun {}
      }
  }
}
