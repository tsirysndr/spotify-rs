use std::convert::TryInto;
use std::time::Duration;
use surf::{Client, Config, Url};

pub mod album;
pub mod artist;
pub mod audio_analysis;
pub mod audio_features;
pub mod category;
pub mod library;
pub mod me;
pub mod player;
pub mod playlist;
pub mod search;

pub struct Spotify {
    pub album: album::AlbumService,
    pub artist: artist::ArtistService,
    pub audio_analysis: audio_analysis::AudioAnalysisService,
    pub audio_features: audio_features::AudioFeaturesService,
    pub category: category::CategoryService,
    pub library: library::LibraryService,
    pub me: me::MeService,
    pub player: player::PlayerService,
    pub playlist: playlist::PlaylistService,
    pub search: search::SearchService,
}

impl Spotify {
    pub fn new() -> Self {
        let client: Client = Config::new()
            .set_base_url(Url::parse("https://api.spotify.com/v1/").unwrap())
            .set_timeout(Some(Duration::from_secs(5)))
            .try_into()
            .unwrap();
        Self {
            album: album::AlbumService::new(&client),
            artist: artist::ArtistService::new(&client),
            audio_analysis: audio_analysis::AudioAnalysisService::new(&client),
            audio_features: audio_features::AudioFeaturesService::new(&client),
            category: category::CategoryService::new(&client),
            library: library::LibraryService::new(&client),
            me: me::MeService::new(&client),
            player: player::PlayerService::new(&client),
            playlist: playlist::PlaylistService::new(&client),
            search: search::SearchService::new(&client),
        }
    }
}
