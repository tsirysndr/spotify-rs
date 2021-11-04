use crate::album::Album;
use crate::album::ExternalIds;
use crate::album::ExternalUrls;
use crate::artist::Artist;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Track {
  pub album: Option<Album>,
  pub artists: Vec<Artist>,
  pub available_markets: Vec<String>,
  pub disc_number: u32,
  pub duration_ms: u32,
  pub explicit: bool,
  pub external_ids: Option<ExternalIds>,
  pub external_urls: Option<ExternalUrls>,
  pub href: String,
  pub is_playable: Option<bool>,
  pub linked_from: Option<Box<Track>>,
  pub id: String,
  pub name: String,
  pub popularity: Option<u32>,
  pub restrictions: Option<Restrictions>,
  pub preview_url: Option<String>,
  pub track_number: u32,
  pub r#type: String,
  pub uri: String,
  pub is_local: bool,
}

#[derive(Debug, Deserialize)]
pub struct Tracks {
  pub href: String,
  pub items: Vec<Track>,
  pub limit: u32,
  pub next: Option<String>,
  pub offset: u32,
  pub previous: Option<String>,
  pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct Restrictions {
  pub reason: Option<String>,
}

pub struct TrackService {
  client: Client,
}

impl TrackService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
