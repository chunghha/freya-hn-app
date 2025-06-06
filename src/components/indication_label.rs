use freya::prelude::*;
use std::borrow::Cow;

#[derive(Props, PartialEq, Clone)]
pub struct IndicationLabelProps {
    #[props(into)]
    pub text: String,

    #[props(default = Cow::Borrowed("rgb(60, 60, 60)"), into)]
    pub color: Cow<'static, str>,

    #[props(default = 16u32, into)]
    pub font_size: u32,

    #[props(default = 10u32, into)]
    pub padding: u32,
}

#[component]
pub fn IndicationLabel(props: IndicationLabelProps) -> Element {
    rsx! {
        rect {
            width: "100%",
            height: "auto",
            padding: "{props.padding}",
            main_align: "center",
            cross_align: "center",
            label {
                font_family: "Geist Mono",
                font_size: "{props.font_size}",
                color: "{props.color}",
                "{props.text}"
            }
        }
    }
}
