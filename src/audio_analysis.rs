use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct AudioAnalysis {
  pub meta: Meta,
  pub bars: Vec<Bar>,
  pub beats: Vec<Beat>,
  pub sections: Vec<Section>,
  pub segments: Vec<Segment>,
  pub tatums: Vec<Tatum>,
  pub track: AnalysisTrack,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
  pub analyzer_version: String,
}

#[derive(Debug, Deserialize)]
pub struct Bar {
  pub start: f32,
  pub duration: f32,
  pub confidence: f32,
}

#[derive(Debug, Deserialize)]
pub struct Beat {
  pub start: f32,
  pub duration: f32,
  pub confidence: f32,
}

#[derive(Debug, Deserialize)]
pub struct Section {
  pub start: f32,
  pub duration: f32,
  pub confidence: f32,
  pub loudness: f32,
  pub tempo: f32,
  pub tempo_confidence: f32,
  pub key: i32,
  pub key_confidence: f32,
  pub mode: i32,
  pub mode_confidence: f32,
  pub time_signature: i32,
  pub time_signature_confidence: f32,
}

#[derive(Debug, Deserialize)]
pub struct Segment {
  pub start: f32,
  pub duration: f32,
  pub confidence: f32,
  pub loudness_start: f32,
  pub loudness_max: f32,
  pub loudness_max_time: f32,
  pub loudness_end: f32,
  pub pitches: Vec<f32>,
  pub timbre: Vec<f32>,
}

#[derive(Debug, Deserialize)]
pub struct Tatum {
  pub start: f32,
  pub duration: f32,
  pub confidence: f32,
}

#[derive(Debug, Deserialize)]
pub struct AnalysisTrack {
  pub num_samples: u32,
  pub duration: f32,
  pub sample_md5: String,
  pub offset_seconds: f32,
  pub window_seconds: f32,
  pub analysis_sample_rate: u32,
  pub analysis_channels: u32,
  pub end_of_fade_in: f32,
  pub start_of_fade_out: f32,
  pub loudness: f32,
  pub tempo: f32,
  pub tempo_confidence: f32,
  pub time_signature: u32,
  pub time_signature_confidence: f32,
  pub key: u32,
  pub key_confidence: f32,
  pub mode: u32,
  pub mode_confidence: f32,
  pub codestring: String,
  pub code_version: f32,
  pub echoprintstring: String,
  pub echoprint_version: f32,
  pub synchstring: String,
  pub synch_version: f32,
  pub rhythmstring: String,
  pub rhythm_version: f32,
}
