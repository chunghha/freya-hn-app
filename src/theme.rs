//! Centralized theme definition for the application.

// An enum to represent the current theme mode.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ThemeMode {
  #[default]
  Light,
  Dark,
}

// A struct to hold all font family names.
#[derive(Clone, PartialEq)]
pub struct FontTheme {
  pub sans: &'static str,
  pub serif: &'static str,
  pub mono: &'static str,
}

// A struct to hold all font weights.
#[derive(Clone, PartialEq)]
pub struct FontWeightTheme {
  pub regular: &'static str,
  pub semibold: &'static str,
  pub bold: &'static str,
}

// A struct to hold all font sizes, providing a consistent scale.
#[derive(Clone, PartialEq)]
pub struct SizeTheme {
  pub text_xs: &'static str,
  pub text_s: &'static str,
  pub text_m: &'static str,
  pub text_l: &'static str,
  pub text_xl: &'static str,
  pub text_xxl: &'static str,
  pub text_header: &'static str,
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
  pub border: &'static str,
  pub tab_background_active: &'static str,
  pub tab_background_hover: &'static str,
  pub tab_text_active: &'static str,
  pub tab_text_inactive: &'static str,
}

// The main Theme struct that combines all the sub-themes.
#[derive(Clone, PartialEq)]
pub struct Theme {
  pub font: FontTheme,
  pub font_weight: FontWeightTheme,
  pub size: SizeTheme,
  pub color: ColorTheme,
}

impl Theme {
  /// Defines the default light theme for the application.
  pub fn light() -> Self {
    Self {
      font: FontTheme { sans: "IBM Plex Sans", serif: "IBM Plex Serif", mono: "IBM Plex Mono" },
      font_weight: FontWeightTheme { regular: "normal", semibold: "semibold", bold: "bold" },
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
        border: "rgb(230, 230, 230)",
        // Light mode active tab remains blueish.
        tab_background_active: "rgb(220, 235, 255)",
        tab_background_hover: "rgb(235, 245, 255)",
        tab_text_active: "rgb(0, 50, 100)",
        tab_text_inactive: "rgb(50, 50, 50)",
      },
    }
  }

  /// Defines the high-contrast dark theme for the application.
  pub fn dark() -> Self {
    Self {
      font: FontTheme { sans: "IBM Plex Sans", serif: "IBM Plex Serif", mono: "IBM Plex Mono" },
      font_weight: FontWeightTheme { regular: "normal", semibold: "semibold", bold: "bold" },
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
        base: "rgb(235, 235, 235)",
        text: "rgb(205, 205, 205)",
        text_alt: "rgb(150, 150, 150)",
        accent: "rgb(255, 102, 0)",
        accent_text: "white",
        link: "rgb(100, 150, 255)",
        background_card: "rgb(28, 28, 32)",
        background_page: "rgb(18, 18, 20)",
        border: "rgb(50, 50, 55)",
        tab_background_active: "rgb(255, 200, 80)",
        tab_background_hover: "rgb(40, 40, 45)",
        tab_text_active: "rgb(20, 20, 20)",
        tab_text_inactive: "rgb(150, 150, 150)",
      },
    }
  }
}
