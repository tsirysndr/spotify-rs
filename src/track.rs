use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Track {}

#[derive(Debug, Deserialize)]
pub struct Tracks {}

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
