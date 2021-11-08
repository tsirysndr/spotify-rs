use crate::album::ExternalUrls;
use crate::album::Image;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Artist {
  pub name: String,
  pub id: String,
  pub uri: String,
  pub href: String,
  pub external_urls: Option<ExternalUrls>,
  pub r#type: String,
  pub followers: Option<Followers>,
  pub genres: Option<Vec<String>>,
  pub images: Option<Vec<Image>>,
  pub popularity: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Artists {
  pub href: String,
  pub items: Vec<Artist>,
  pub limit: u32,
  pub next: Option<String>,
  pub offset: u32,
  pub previous: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Followers {
  pub href: Option<String>,
  pub total: u32,
}

pub struct ArtistService {
  client: Client,
}

impl ArtistService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub async fn get(&self, id: &str) -> Result<Artist, surf::Error> {
    let res = self
      .client
      .get(format!("artists/{}", id))
      .recv_json::<Artist>()
      .await?;
    Ok(res)
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
