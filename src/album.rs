use crate::artist::Artist;
use crate::track::Tracks;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Album {
  pub album_type: String,
  pub artists: Vec<Artist>,
  pub available_markets: Vec<String>,
  pub copyrights: Vec<Copyright>,
  pub external_ids: Option<ExternalIds>,
  pub external_urls: Option<ExternalUrls>,
  pub genres: Vec<String>,
  pub href: String,
  pub id: String,
  pub images: Vec<Image>,
  pub label: String,
  pub name: String,
  pub popularity: i32,
  pub release_date: String,
  pub release_date_precision: String,
  pub tracks: Option<Tracks>,
  pub r#type: String,
  pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Copyright {
  pub album_type: String,
  pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct ExternalIds {
  pub upc: String,
}

#[derive(Debug, Deserialize)]
pub struct ExternalUrls {
  pub spotify: String,
}

#[derive(Debug, Deserialize)]
pub struct Image {
  pub height: u32,
  pub url: String,
  pub width: u32,
}

#[derive(Debug, Deserialize)]
pub struct Albums {
  pub albums: Vec<Album>,
}

pub struct AlbumService {
  client: Client,
}

impl AlbumService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub async fn list(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get(&self, id: &str) -> Result<(), surf::Error> {
    let res = self
      .client
      .get(format!("albums/{}", id))
      .recv_string()
      .await?;
    println!("{}", res);
    Ok(())
  }

  pub async fn get_tracks(&self, id: &str, limit: u32, offset: u32) -> Result<(), surf::Error> {
    Ok(())
  }
}
