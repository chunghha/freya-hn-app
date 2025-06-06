use freya::prelude::Signal;
use jiff::Timestamp;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Story {
    pub id: u32,
    pub title: Option<String>,
    pub url: Option<String>,
    pub by: Option<String>,
    pub score: Option<u32>,
    #[serde(default, with = "jiff::fmt::serde::timestamp::second::optional")]
    pub time: Option<Timestamp>,
    pub descendants: Option<u32>,
    pub kids: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Comment {
    pub id: u32,
    pub by: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default, with = "jiff::fmt::serde::timestamp::second::optional")]
    pub time: Option<Timestamp>,
    pub kids: Option<Vec<u32>>,
    #[serde(default)]
    pub deleted: bool,

    // --- NEW: UI State Fields ---
    // We will populate these fields manually, so we tell Serde to skip them.
    #[serde(skip)]
    pub children: Signal<Vec<Comment>>,
    #[serde(skip)]
    pub is_expanded: Signal<bool>,
}
