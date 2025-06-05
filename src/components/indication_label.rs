use freya::prelude::*;

#[derive(Props, PartialEq, Default, Clone)]
pub struct IndicationLabelProps {
    pub text: String,
    #[props(optional)]
    pub color: Option<String>,
    #[props(optional)]
    pub font_size: Option<u32>,
    #[props(optional)]
    pub padding: Option<u32>,
}

const DEFAULT_INDICATION_COLOR: &str = "rgb(60, 60, 60)";
const DEFAULT_INDICATION_FONT_SIZE: u32 = 16;
const DEFAULT_INDICATION_PADDING: u32 = 10;

#[component]
pub fn IndicationLabel(props: IndicationLabelProps) -> Element {
    let color = props
        .color
        .unwrap_or_else(|| DEFAULT_INDICATION_COLOR.to_string());
    let font_size = props.font_size.unwrap_or(DEFAULT_INDICATION_FONT_SIZE);
    let padding = props.padding.unwrap_or(DEFAULT_INDICATION_PADDING);

    rsx! {
        rect {
            width: "100%",
            height: "auto",
            padding: "{padding}",
            main_align: "center",
            cross_align: "center",
            label {
                font_family: "Geist Mono",
                font_size: "{font_size}",
                color: "{color}",
                "{props.text}"
            }
        }
    }
}
