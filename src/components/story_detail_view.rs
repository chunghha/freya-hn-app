use crate::components::comment_view::CommentView;
use crate::components::icons::*;
use crate::components::info_line::InfoLine;
use crate::components::no_story_selected_view::NoStorySelectedView;
use crate::components::primitives::Spacer;
use crate::components::skeletons::CommentSkeleton;
use crate::models::{Comment, FetchState, Story};
use crate::theme::Theme;
use crate::utils::api::ApiService;
use crate::utils::datetime::format_timestamp;
use freya::prelude::*;
use log::info;
use std::collections::HashMap;
use std::sync::Arc;

// --- Local Components ---
#[component]
fn RenderComments(
  comment_ids: Vec<u32>,
  all_comments: Signal<HashMap<u32, Comment>>,
  depth: u16,
  on_toggle_expand: EventHandler<u32>,
  on_retry_fetch: EventHandler<u32>,
) -> Element {
  let comments_map = all_comments.read();
  rsx! {
      {
          comment_ids.iter().map(|id| {
              if let Some(comment) = comments_map.get(id) {
                  let children_nodes = if *comment.is_expanded.read() && comment.fetch_state.read().clone() == FetchState::Idle {
                      let children_ids: Vec<u32> = comment.children.read().iter().map(|c| c.id).collect();
                      rsx! {
                          RenderComments {
                              comment_ids: children_ids,
                              all_comments: all_comments,
                              depth: depth + 1,
                              on_toggle_expand: on_toggle_expand,
                              on_retry_fetch: on_retry_fetch,
                          }
                      }
                  } else {
                      rsx! { Fragment {} }
                  };

                  rsx! {
                      Fragment {
                          key: "{comment.id}",
                          CommentView {
                              comment: comment.clone(),
                              depth: depth,
                              on_toggle_expand: on_toggle_expand,
                              on_retry_fetch: on_retry_fetch,
                          }
                          {children_nodes}
                      }
                  }
              } else {
                  rsx! { Fragment {} }
              }
          })
      }
  }
}

// --- Main Component ---
#[component]
pub fn StoryDetailView(story_data: Signal<Option<Story>>, on_back: EventHandler<()>) -> Element {
  let theme_signal = use_context::<Signal<Theme>>();
  let theme = theme_signal.read();
  let api_service = use_context::<Arc<ApiService>>();

  const SKELETON_COUNT: usize = 5;
  const DETAIL_PADDING: &str = "15";
  const VERTICAL_SPACER_HEIGHT: &str = "12";
  const COMMENTS_SECTION_SPACER: &str = "20";
  const TITLE_PLACEHOLDER: &str = "[No Title]";

  let mut all_comments: Signal<HashMap<u32, Comment>> = use_signal(HashMap::new);

  let comments_resource = use_resource({
    let api_service = api_service.clone();
    move || {
      let story = story_data.read().clone();
      let mut all_comments = all_comments;
      let api_service = api_service.clone();
      async move {
        if let Some(story) = story {
          if let Some(kids) = story.kids {
            info!("Fetching {} top-level comments...", kids.len());
            for kid_id in kids {
              if let Ok(comment) = api_service.fetch_comment_content(kid_id).await {
                all_comments.write().insert(kid_id, comment);
              }
            }
          }
        }
      }
    }
  });

  if let Some(story) = story_data.read().as_ref() {
    rsx! {
        ScrollView {
            width: "100%",
            height: "fill",
            show_scrollbar: true,
            rect {
                width: "100%",
                height: "auto",
                padding: DETAIL_PADDING,
                direction: "vertical",
                background: "{theme.color.background_card}",

                // Story Header
                rect {
                    onclick: move |_| on_back.call(()),
                    background: "{theme.color.background_page}",
                    border: "1 solid {theme.color.border}",
                    padding: "6 10",
                    corner_radius: "6",
                    label {
                        font_family: "{theme.font.sans}",
                        font_size: "{theme.size.text_m}",
                        color: "{theme.color.text}",
                        "← Back to List"
                    }
                }
                Spacer { height: VERTICAL_SPACER_HEIGHT }
                label {
                    font_family: "{theme.font.serif}",
                    font_size: "{theme.size.text_xxl}",
                    font_weight: "{theme.font_weight.bold}",
                    "{story.title.as_deref().unwrap_or(TITLE_PLACEHOLDER)}"
                }
                Spacer { height: VERTICAL_SPACER_HEIGHT }
                if let Some(url) = &story.url {
                    Link {
                        to: url.clone(),
                        label {
                            font_family: "{theme.font.mono}",
                            font_size: "{theme.size.text_l}",
                            color: "{theme.color.link}",
                            "URL: {url}"
                        }
                    }
                    Spacer { height: VERTICAL_SPACER_HEIGHT }
                }

                // Story Metadata
                InfoLine { icon: rsx!{ IconScore {} }, text: format!("Score: {}", story.score.unwrap_or(0)) }
                Spacer { height: VERTICAL_SPACER_HEIGHT }
                InfoLine { icon: rsx!{ IconUser {} }, text: format!("By: {}", story.by.as_deref().unwrap_or("N/A")) }
                Spacer { height: VERTICAL_SPACER_HEIGHT }
                if let Some(time) = &story.time {
                    InfoLine { icon: rsx!{ IconTime {} }, text: format!("Time: {}", format_timestamp(time)) }
                    Spacer { height: VERTICAL_SPACER_HEIGHT }
                }
                InfoLine { icon: rsx!{ IconComments {} }, text: format!("Comments: {}", story.descendants.unwrap_or(0)) }
                Spacer { height: COMMENTS_SECTION_SPACER }

                // Comments Section
                label {
                    font_size: "{theme.size.text_l}",
                    font_weight: "{theme.font_weight.bold}",
                    "Comments:"
                }
                Spacer { height: "4" }

                if comments_resource.value().read().is_none() {
                    Fragment {
                        {
                            (0..SKELETON_COUNT).map(|_| rsx!{ CommentSkeleton {} })
                        }
                    }
                } else if let Some(kids) = &story.kids {
                    RenderComments {
                        comment_ids: kids.clone(),
                        all_comments: all_comments,
                        depth: 0,
                        on_toggle_expand: {
                            let api_service = api_service.clone();
                            move |comment_id: u32| {
                                let mut comments_map = all_comments.write();
                                if let Some(comment_to_toggle) = comments_map.get_mut(&comment_id) {
                                    let is_currently_expanded = *comment_to_toggle.is_expanded.read();
                                    if !is_currently_expanded {
                                        comment_to_toggle.is_expanded.set(true);
                                        if comment_to_toggle.children.read().is_empty() {
                                            if let Some(kids) = comment_to_toggle.kids.clone() {
                                                let mut children_signal = comment_to_toggle.children;
                                                let mut fetch_state_signal = comment_to_toggle.fetch_state;
                                                let api_service = api_service.clone();
                                                spawn(async move {
                                                    fetch_state_signal.set(FetchState::Loading);
                                                    let mut fetched_children = Vec::new();
                                                    let mut all_successful = true;
                                                    for kid_id in kids {
                                                        if let Ok(child) = api_service.fetch_comment_content(kid_id).await {
                                                            fetched_children.push(child);
                                                        } else {
                                                            all_successful = false;
                                                            break;
                                                        }
                                                    }
                                                    if all_successful {
                                                        children_signal.set(fetched_children);
                                                        fetch_state_signal.set(FetchState::Idle);
                                                    } else {
                                                        fetch_state_signal.set(FetchState::Failed);
                                                    }
                                                });
                                            }
                                        }
                                    } else {
                                        comment_to_toggle.is_expanded.set(false);
                                    }
                                }
                            }
                        },
                        on_retry_fetch: {
                            let api_service = api_service.clone();
                            move |comment_id: u32| {
                                let mut comments_map = all_comments.write();
                                if let Some(comment_to_toggle) = comments_map.get_mut(&comment_id) {
                                    if *comment_to_toggle.fetch_state.read() == FetchState::Failed {
                                        if let Some(kids) = comment_to_toggle.kids.clone() {
                                            let mut children_signal = comment_to_toggle.children;
                                            let mut fetch_state_signal = comment_to_toggle.fetch_state;
                                            let api_service = api_service.clone();
                                            spawn(async move {
                                                fetch_state_signal.set(FetchState::Loading);
                                                let mut fetched_children = Vec::new();
                                                let mut all_successful = true;
                                                for kid_id in kids {
                                                    if let Ok(child) = api_service.fetch_comment_content(kid_id).await {
                                                        fetched_children.push(child);
                                                    } else {
                                                        all_successful = false;
                                                        break;
                                                    }
                                                }
                                                if all_successful {
                                                    children_signal.set(fetched_children);
                                                    fetch_state_signal.set(FetchState::Idle);
                                                } else {
                                                    fetch_state_signal.set(FetchState::Failed);
                                                }
                                            });
                                        }
                                    }
                                }
                            }
                        },
                    }
                } else {
                    label {
                        color: "{theme.color.text_alt}",
                        "No comments to display."
                    }
                }
            }
        }
    }
  } else {
    rsx! {
        NoStorySelectedView {
            on_back: on_back
        }
    }
  }
}
