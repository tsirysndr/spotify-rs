use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Playlist {}

pub struct PlaylistService {
  client: Client,
}

impl PlaylistService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub async fn get(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_cover_images(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_tracks(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_user_playlists(&self) -> Result<(), surf::Error> {
    Ok(())
  }
}
