use freya::prelude::*;

// --- Spacer Component ---
#[derive(Props, PartialEq, Clone)]
pub struct SpacerProps {
  #[props(default = "0", into)]
  pub width: &'static str,
  #[props(default = "0", into)]
  pub height: &'static str,
}

#[component]
pub fn Spacer(props: SpacerProps) -> Element {
  rsx! {
      rect {
          width: props.width,
          height: props.height,
      }
  }
}

// --- ErrorView Component ---
#[derive(Props, PartialEq, Clone)]
pub struct ErrorViewProps {
  #[props(into)]
  pub message: String,
}

#[component]
pub fn ErrorView(props: ErrorViewProps) -> Element {
  const ERROR_COLOR: &str = "red";
  const ERROR_FONT_SIZE: &str = "16";
  const ERROR_PADDING: &str = "10";
  const CONTAINER_WIDTH: &str = "100%";

  rsx! {
      rect {
          width: CONTAINER_WIDTH,
          padding: ERROR_PADDING,
          main_align: "center",
          label {
              color: ERROR_COLOR,
              font_size: ERROR_FONT_SIZE,
              "Error: {props.message}"
          }
      }
  }
}
