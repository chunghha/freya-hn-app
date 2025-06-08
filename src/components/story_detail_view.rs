use crate::components::comment_view::CommentView;
use crate::components::icons::*;
use crate::components::info_line::InfoLine;
use crate::components::no_story_selected_view::NoStorySelectedView;
use crate::components::primitives::Spacer;
use crate::components::skeletons::CommentSkeleton;
use crate::models::{Comment, FetchState, Story};
use crate::utils::api::hn_item_url;
use crate::utils::datetime::format_timestamp;
use freya::prelude::*;
use log::info;
use std::collections::HashMap;

// --- Async Helper Functions ---
async fn fetch_comment_content(id: u32) -> Result<Comment, String> {
  let url = hn_item_url(id);
  let mut comment: Comment =
    reqwest::get(&url).await.map_err(|e| e.to_string())?.json().await.map_err(|e| e.to_string())?;

  comment.children = Signal::new(vec![]);
  comment.is_expanded = Signal::new(false);
  comment.fetch_state = Signal::new(FetchState::Idle);
  Ok(comment)
}

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
  // --- Constants ---
  const SKELETON_COUNT: usize = 5;
  const DETAIL_PADDING: &str = "15";
  const DETAIL_BG: &str = "rgb(250, 250, 250)";
  const TITLE_FONT_SIZE: &str = "22";
  const TITLE_FONT_WEIGHT: &str = "bold";
  const TITLE_PLACEHOLDER: &str = "[No Title]";
  const URL_FONT_SIZE: &str = "14";
  const URL_COLOR: &str = "rgb(0, 0, 200)";
  const VERTICAL_SPACER_HEIGHT: &str = "12";
  const COMMENTS_SECTION_SPACER: &str = "20";
  const COMMENTS_TITLE_FONT_SIZE: &str = "16";
  const COMMENTS_TITLE_FONT_WEIGHT: &str = "bold";
  const COMMENTS_PLACEHOLDER_COLOR: &str = "rgb(100,100,100)";

  // --- State and Hooks ---
  let mut all_comments: Signal<HashMap<u32, Comment>> = use_signal(HashMap::new);

  let comments_resource = use_resource(move || {
    let story = story_data.read().clone();
    let mut all_comments = all_comments;
    async move {
      if let Some(story) = story {
        if let Some(kids) = story.kids {
          info!("Fetching {} top-level comments...", kids.len());
          for kid_id in kids {
            if let Ok(comment) = fetch_comment_content(kid_id).await {
              all_comments.write().insert(kid_id, comment);
            }
          }
        }
      }
    }
  });

  let on_toggle_or_retry = move |comment_id: u32| {
    let mut comments_map = all_comments.write();
    if let Some(comment_to_toggle) = comments_map.get_mut(&comment_id) {
      let is_currently_expanded = *comment_to_toggle.is_expanded.read();

      if !is_currently_expanded || *comment_to_toggle.fetch_state.read() == FetchState::Failed {
        comment_to_toggle.is_expanded.set(true);

        if comment_to_toggle.children.read().is_empty() {
          if let Some(kids) = comment_to_toggle.kids.clone() {
            let mut children_signal = comment_to_toggle.children;
            let mut fetch_state_signal = comment_to_toggle.fetch_state;

            spawn(async move {
              info!("Fetching {} children for comment {}", kids.len(), comment_id);
              fetch_state_signal.set(FetchState::Loading);
              let mut fetched_children = Vec::new();
              let mut all_successful = true;

              for kid_id in kids {
                match fetch_comment_content(kid_id).await {
                  Ok(child_comment) => fetched_children.push(child_comment),
                  Err(_) => {
                    all_successful = false;
                    break;
                  }
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
  };

  // --- Render Logic ---
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
                background: DETAIL_BG,

                // Story Header
                Button { onclick: move |_| on_back.call(()), label { "← Back to List" } }
                Spacer { height: VERTICAL_SPACER_HEIGHT }
                label { font_size: TITLE_FONT_SIZE, font_weight: TITLE_FONT_WEIGHT, "{story.title.as_deref().unwrap_or(TITLE_PLACEHOLDER)}" }
                Spacer { height: VERTICAL_SPACER_HEIGHT }
                if let Some(url) = &story.url {
                    Link { to: url.clone(), label { font_size: URL_FONT_SIZE, color: URL_COLOR, "URL: {url}" } }
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
                label { font_size: COMMENTS_TITLE_FONT_SIZE, font_weight: COMMENTS_TITLE_FONT_WEIGHT, "Comments:" }
                Spacer { height: "4" }

                // Conditional rendering for the comment list.
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
                        on_toggle_expand: on_toggle_or_retry,
                        on_retry_fetch: on_toggle_or_retry,
                    }
                } else {
                    label { color: COMMENTS_PLACEHOLDER_COLOR, "No comments to display." }
                }
            }
        }
    }
  } else {
    // Fallback view if no story is selected.
    rsx! {
        NoStorySelectedView {
            on_back: on_back
        }
    }
  }
}
