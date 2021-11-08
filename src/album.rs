use crate::artist::Artist;
use crate::track::AlbumTracks;
use serde::{Deserialize, Serialize};
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Album {
  pub album_type: String,
  pub artists: Vec<Artist>,
  pub available_markets: Vec<String>,
  pub copyrights: Option<Vec<Copyright>>,
  pub external_ids: Option<ExternalIds>,
  pub external_urls: Option<ExternalUrls>,
  pub genres: Option<Vec<String>>,
  pub href: String,
  pub id: String,
  pub images: Vec<Image>,
  pub label: Option<String>,
  pub name: String,
  pub popularity: Option<i32>,
  pub release_date: String,
  pub release_date_precision: String,
  pub tracks: Option<AlbumTracks>,
  pub r#type: String,
  pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Copyright {
  pub text: String,
  pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct ExternalIds {
  pub upc: Option<String>,
  pub isrc: Option<String>,
  pub ean: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExternalUrls {
  pub spotify: String,
}

#[derive(Debug, Deserialize)]
pub struct Image {
  pub height: Option<u32>,
  pub url: String,
  pub width: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Albums {
  pub albums: Vec<Album>,
}

#[derive(Debug, Deserialize)]
pub struct AlbumsResult {
  pub items: Vec<Album>,
  pub limit: u32,
  pub next: Option<String>,
  pub offset: u32,
  pub previous: Option<String>,
  pub total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumParams {
  pub ids: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationParams {
  pub limit: u32,
  pub offset: u32,
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

  pub async fn list(&self, ids: &str) -> Result<Albums, surf::Error> {
    let params = AlbumParams {
      ids: ids.to_string(),
    };
    let res = self
      .client
      .get("albums")
      .query(&params)?
      .recv_json::<Albums>()
      .await?;
    Ok(res)
  }

  pub async fn get(&self, id: &str) -> Result<Album, surf::Error> {
    let res = self
      .client
      .get(format!("albums/{}", id))
      .recv_json::<Album>()
      .await?;
    Ok(res)
  }

  pub async fn get_tracks(
    &self,
    id: &str,
    limit: u32,
    offset: u32,
  ) -> Result<AlbumTracks, surf::Error> {
    let params = PaginationParams { limit, offset };
    let res = self
      .client
      .get(format!("albums/{}/tracks", id))
      .query(&params)?
      .recv_json::<AlbumTracks>()
      .await?;
    Ok(res)
  }
}
