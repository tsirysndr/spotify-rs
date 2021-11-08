use crate::album::ExternalUrls;
use crate::album::Image;
use crate::album::PaginationParams;
use crate::artist::Followers;
use crate::track::Track;
use crate::user::Profile;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Playlist {
  pub collaborative: bool,
  pub description: String,
  pub external_urls: Option<ExternalUrls>,
  pub followers: Option<Followers>,
  pub href: String,
  pub id: String,
  pub images: Vec<Image>,
  pub name: String,
  pub owner: Option<Profile>,
  pub primary_color: Option<String>,
  pub public: bool,
  pub snapshot_id: String,
  pub tracks: Option<PlaylistTracks>,
}

#[derive(Debug, Deserialize)]
pub struct Playlists {
  pub href: Option<String>,
  pub items: Vec<Playlist>,
  pub limit: Option<u32>,
  pub next: Option<String>,
  pub offset: Option<u32>,
  pub previous: Option<String>,
  pub total: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistTracks {
  pub href: Option<String>,
  pub items: Option<Vec<PlaylistTrack>>,
  pub limit: Option<u32>,
  pub next: Option<String>,
  pub offset: Option<u32>,
  pub previous: Option<String>,
  pub total: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistTrack {
  pub added_at: String,
  pub added_by: Option<Profile>,
  pub is_local: bool,
  pub primary_color: Option<String>,
  pub track: Track,
  pub video_thumbnail: Option<VideoThumbnail>,
}

#[derive(Debug, Deserialize)]
pub struct VideoThumbnail {
  pub url: Option<String>,
}

pub struct PlaylistService {
  client: Client,
}

impl PlaylistService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub async fn get(&self, id: &str) -> Result<Playlist, surf::Error> {
    let res = self
      .client
      .get(format!("playlists/{}", id))
      .recv_json::<Playlist>()
      .await?;
    Ok(res)
  }

  pub async fn get_cover_images(&self, id: &str) -> Result<Vec<Image>, surf::Error> {
    let res = self
      .client
      .get(format!("playlists/{}/images", id))
      .recv_json::<Vec<Image>>()
      .await?;
    Ok(res)
  }

  pub async fn get_tracks(
    &self,
    id: &str,
    limit: u32,
    offset: u32,
  ) -> Result<PlaylistTracks, surf::Error> {
    let params = PaginationParams { limit, offset };
    let res = self
      .client
      .get(format!("playlists/{}/tracks", id))
      .query(&params)?
      .recv_json::<PlaylistTracks>()
      .await?;
    Ok(res)
  }

  pub async fn get_user_playlists(
    &self,
    user_id: &str,
    limit: u32,
    offset: u32,
  ) -> Result<Playlists, surf::Error> {
    let params = PaginationParams { limit, offset };
    let res = self
      .client
      .get(format!("users/{}/playlists", user_id))
      .query(&params)?
      .recv_json::<Playlists>()
      .await?;
    Ok(res)
  }
}
