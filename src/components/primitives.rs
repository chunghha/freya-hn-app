use freya::prelude::*;

#[component]
pub fn ErrorView(message: String) -> Element {
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
                "Error: {message}"
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct SpacerProps {
    /// The width of the spacer. Defaults to "0".
    #[props(default = "0", into)]
    pub width: &'static str,
    /// The height of the spacer. Defaults to "0".
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
