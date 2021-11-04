use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct AudioFeatures {
  pub acousticness: f32,
  pub analysis_url: String,
  pub danceability: f32,
  pub duration_ms: i32,
  pub energy: f32,
  pub id: String,
  pub instrumentalness: f32,
  pub key: i32,
  pub liveness: f32,
  pub loudness: f32,
  pub mode: i32,
  pub speechiness: f32,
  pub tempo: f32,
  pub time_signature: i32,
  pub track_href: String,
  pub uri: String,
  pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct AudioFeaturesList {
  pub audio_features: Vec<AudioFeatures>,
}
