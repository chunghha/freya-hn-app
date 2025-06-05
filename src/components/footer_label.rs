use freya::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FooterLabelProps {
    pub text: String,
    #[props(optional)]
    pub color: Option<String>,
    #[props(optional)]
    pub font_size: Option<u32>,
}

#[component]
pub fn FooterLabel(props: FooterLabelProps) -> Element {
    const DEFAULT_FOOTER_COLOR: &str = "rgb(40, 60, 80)";
    const DEFAULT_FOOTER_FONT_SIZE: u32 = 14;

    let color = props
        .color
        .clone()
        .unwrap_or_else(|| DEFAULT_FOOTER_COLOR.to_string());
    let font_size = props.font_size.unwrap_or(DEFAULT_FOOTER_FONT_SIZE);

    rsx! {
        label {
            font_family: "Lato",
            font_size: "{font_size}",
            font_weight: "semibold",
            color: "{color}",
            "{props.text}"
        }
    }
}
