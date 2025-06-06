//! Contains constants and helpers for interacting with the Hacker News API.

/// The base URL for fetching the list of best story IDs.
pub const HN_BEST_STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/beststories.json";

/// The base URL for fetching any item (story or comment).
pub const HN_ITEM_URL_BASE: &str = "https://hacker-news.firebaseio.com/v0/item/";

/// Generates the full URL for a specific Hacker News item using its ID.
/// This is the single function used by the rest of the app to get an item URL.
pub fn hn_item_url(id: u32) -> String {
  // This implementation now clearly uses the base constant.
  format!("{}{}.json", HN_ITEM_URL_BASE, id)
}
