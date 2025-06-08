//! Centralized theme definition for the application.

// A struct to hold all font family names.
#[derive(Clone, PartialEq)]
pub struct FontTheme {
  pub sans: &'static str,
  pub serif: &'static str,
  pub mono: &'static str,
}

// A struct to hold all font sizes, providing a consistent scale.
#[derive(Clone, PartialEq)]
pub struct SizeTheme {
  pub text_xs: &'static str,     // 12px - extra small meta text
  pub text_s: &'static str,      // 13px - footer labels
  pub text_m: &'static str,      // 14px - comment text, body
  pub text_l: &'static str,      // 16px - URLs, indication labels
  pub text_xl: &'static str,     // 20px - story card titles
  pub text_xxl: &'static str,    // 22px - story detail titles
  pub text_header: &'static str, // 24px - main header
}

// A struct to hold all application colors.
#[derive(Clone, PartialEq)]
pub struct ColorTheme {
  pub base: &'static str,
  pub text: &'static str,
  pub text_alt: &'static str,
  pub accent: &'static str,
  pub accent_text: &'static str,
  pub link: &'static str,
  pub background_card: &'static str,
  pub background_page: &'static str,
}

// The main Theme struct that combines all the sub-themes.
#[derive(Clone, PartialEq)]
pub struct Theme {
  pub font: FontTheme,
  pub size: SizeTheme,
  pub color: ColorTheme,
}

impl Theme {
  /// Defines the default light theme for the application.
  pub fn light() -> Self {
    Self {
      font: FontTheme { sans: "IBM Plex Sans", serif: "IBM Plex Serif", mono: "IBM Plex Mono" },
      size: SizeTheme {
        text_xs: "12",
        text_s: "13",
        text_m: "14",
        text_l: "16",
        text_xl: "20",
        text_xxl: "22",
        text_header: "24",
      },
      color: ColorTheme {
        base: "black",
        text: "rgb(50, 50, 50)",
        text_alt: "rgb(80, 80, 80)",
        accent: "rgb(255, 102, 0)",
        accent_text: "white",
        link: "rgb(0, 0, 200)",
        background_card: "white",
        background_page: "rgb(246, 246, 239)",
      },
    }
  }
}
