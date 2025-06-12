use crate::components::card_footer::CardFooter;
use crate::models::Story;
use crate::theme::Theme;
use freya::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct StoryCardProps {
  pub story: Story,
  pub on_select: EventHandler<u32>,
}

#[component]
pub fn StoryCard(props: StoryCardProps) -> Element {
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();

  const CARD_MARGIN: &str = "0 0 8 0";
  const CARD_CORNER_RADIUS: &str = "8";
  const CARD_SHADOW: &str = "0 2 8 0 rgb(0,0,0,0.1)";
  const CARD_PADDING: &str = "12 16";

  let story_id = props.story.id;

  rsx! {
      rect {
          key: "{props.story.id}",
          width: "100%",
          height: "auto",
          direction: "vertical",
          padding: CARD_PADDING,
          margin: CARD_MARGIN,
          corner_radius: CARD_CORNER_RADIUS,
          background: "{theme.color.background_card}",
          shadow: CARD_SHADOW,
          onclick: move |_| props.on_select.call(story_id),

          label {
              font_family: "{theme.font.serif}",
              font_size: "{theme.size.text_xl}",
              font_weight: "{theme.font_weight.bold}",
              color: "{theme.color.base}",
              max_lines: "2",
              "{props.story.title.as_deref().unwrap_or(\"[No Title]\")}"
          }

          {
              props.story.url.as_ref().map(|url| rsx! {
                  label {
                      font_family: "{theme.font.mono}",
                      font_size: "{theme.size.text_l}",
                      color: "{theme.color.text_alt}",
                      max_lines: "1",
                      text_overflow: "ellipsis",
                      "{url}"
                  }
              })
          }

          CardFooter { story: props.story.clone() }
      }
  }
}
