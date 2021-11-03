use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Artist {}

pub struct ArtistService {
  client: Client,
}

impl ArtistService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub async fn get(&self, id: &str) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn list(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_top_tracks(&self, id: &str, country: &str) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_albums(&self, id: &str, limit: u32, offset: u32) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn related_artists(&self, id: &str) -> Result<(), surf::Error> {
    Ok(())
  }
}
