//! Contains the centralized ApiService for all Hacker News network requests.

use crate::models::{Comment, Story};
use reqwest::Client;
use strum_macros::Display;

// --- Public Enum for Story List Types ---
#[derive(Clone, Copy, PartialEq, Display)]
pub enum StoryListType {
  Best,
  Top,
  New,
  Ask,
  Show,
  Job,
}

// ... (The rest of the file remains exactly the same) ...
impl StoryListType {
  fn as_api_str(&self) -> &str {
    match self {
      Self::Best => "beststories",
      Self::Top => "topstories",
      Self::New => "newstories",
      Self::Ask => "askstories",
      Self::Show => "showstories",
      Self::Job => "jobstories",
    }
  }
}
const HN_API_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0/";
pub fn hn_item_url(id: u32) -> String {
  format!("{}item/{}.json", HN_API_BASE_URL, id)
}
pub fn get_story_list_url(list_type: StoryListType) -> String {
  format!("{}{}.json", HN_API_BASE_URL, list_type.as_api_str())
}
#[derive(Clone)]
pub struct ApiService {
  client: Client,
}
impl ApiService {
  pub fn new() -> Self {
    Self { client: Client::new() }
  }
  pub async fn fetch_story_ids(&self, list_type: StoryListType) -> Result<Vec<u32>, String> {
    self
      .client
      .get(get_story_list_url(list_type))
      .send()
      .await
      .map_err(|e| e.to_string())?
      .json::<Vec<u32>>()
      .await
      .map_err(|e| e.to_string())
  }
  pub async fn fetch_story_content(&self, id: u32) -> Result<Story, String> {
    self
      .client
      .get(hn_item_url(id))
      .send()
      .await
      .map_err(|e| e.to_string())?
      .json::<Story>()
      .await
      .map_err(|e| e.to_string())
  }
  pub async fn fetch_comment_content(&self, id: u32) -> Result<Comment, String> {
    let url = hn_item_url(id);
    let mut comment: Comment =
      self.client.get(url).send().await.map_err(|e| e.to_string())?.json().await.map_err(|e| e.to_string())?;
    comment.children = freya::prelude::Signal::new(vec![]);
    comment.is_expanded = freya::prelude::Signal::new(false);
    comment.fetch_state = freya::prelude::Signal::new(crate::models::FetchState::Idle);
    Ok(comment)
  }
}
