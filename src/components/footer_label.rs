use crate::components::primitives::Spacer;
use freya::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FooterLabelProps {
    pub text: String,
    #[props(optional)]
    pub icon: Option<Element>,
}

#[component]
pub fn FooterLabel(props: FooterLabelProps) -> Element {
    const LABEL_COLOR: &str = "rgb(50, 50, 50)";
    const LABEL_FONT_SIZE: &str = "13";
    const ICON_TEXT_SPACING: &str = "4";

    rsx! {
        rect {
            direction: "horizontal",
            cross_align: "center",
            if let Some(icon) = props.icon {
                Fragment {
                    {icon}
                    Spacer { width: ICON_TEXT_SPACING }
                }
            }
            label {
                font_size: LABEL_FONT_SIZE,
                color: LABEL_COLOR,
                "{props.text}"
            }
        }
    }
}
