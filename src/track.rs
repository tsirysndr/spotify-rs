use crate::album::Album;
use crate::album::ExternalIds;
use crate::album::ExternalUrls;
use crate::artist::Artist;
use crate::audio_analysis::AudioAnalysis;
use crate::audio_features::{AudioFeatures, AudioFeaturesList};
use serde::{Deserialize, Serialize};
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
pub struct AlbumTracks {
  pub items: Vec<Track>,
  pub limit: u32,
  pub next: Option<String>,
  pub offset: u32,
  pub previous: Option<String>,
  pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct Tracks {
  pub tracks: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Restrictions {
  pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackParams {
  pub ids: String,
  pub market: Option<String>,
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

  pub async fn get(&self, id: &str) -> Result<Track, surf::Error> {
    let res = self
      .client
      .get(format!("tracks/{}", id))
      .recv_json::<Track>()
      .await?;
    Ok(res)
  }

  pub async fn list(&self, ids: &str, market: Option<String>) -> Result<Tracks, surf::Error> {
    let params = TrackParams {
      ids: ids.to_string(),
      market,
    };
    let res = self
      .client
      .get("tracks")
      .query(&params)?
      .recv_json::<Tracks>()
      .await?;
    Ok(res)
  }

  pub async fn audio_features(&self, ids: &str) -> Result<AudioFeaturesList, surf::Error> {
    let params = TrackParams {
      ids: ids.to_string(),
      market: None,
    };
    let res = self
      .client
      .get("audio-features")
      .query(&params)?
      .recv_json::<AudioFeaturesList>()
      .await?;
    Ok(res)
  }

  pub async fn audio_features_by_track_id(&self, id: &str) -> Result<AudioFeatures, surf::Error> {
    let res = self
      .client
      .get(format!("audio-features/{}", id))
      .recv_json::<AudioFeatures>()
      .await?;
    Ok(res)
  }

  pub async fn audio_analysis(&self, id: &str) -> Result<AudioAnalysis, surf::Error> {
    let res = self
      .client
      .get(format!("audio-analysis/{}", id))
      .recv_json::<AudioAnalysis>()
      .await?;
    Ok(res)
  }
}
