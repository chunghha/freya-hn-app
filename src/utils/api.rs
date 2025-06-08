//! Contains the centralized ApiService for all Hacker News network requests.

use crate::models::{Comment, Story};
use reqwest::Client;

// --- Private Constants ---
const HN_BEST_STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/beststories.json";
const HN_ITEM_URL_BASE: &str = "https://hacker-news.firebaseio.com/v0/item/";

pub fn hn_item_url(id: u32) -> String {
  format!("{}{}.json", HN_ITEM_URL_BASE, id)
}

// --- The Service ---
#[derive(Clone)]
pub struct ApiService {
  client: Client,
}

impl ApiService {
  pub fn new() -> Self {
    Self { client: Client::new() }
  }

  pub async fn fetch_best_story_ids(&self) -> Result<Vec<u32>, String> {
    self
      .client
      .get(HN_BEST_STORIES_URL)
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
      self.client.get(&url).send().await.map_err(|e| e.to_string())?.json().await.map_err(|e| e.to_string())?;

    comment.children = freya::prelude::Signal::new(vec![]);
    comment.is_expanded = freya::prelude::Signal::new(false);
    comment.fetch_state = freya::prelude::Signal::new(crate::models::FetchState::Idle);
    Ok(comment)
  }
}
