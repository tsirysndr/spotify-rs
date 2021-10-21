use surf::Client;

pub struct ArtistService {
  client: Client,
}

impl ArtistService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn get(&self, id: &str) {}

  pub fn list(&self) {}

  pub fn get_top_tracks(&self, id: &str, country: &str) {}

  pub fn get_albums(&self, id: &str, limit: u32, offset: u32) {}

  pub fn related_artists(&self, id: &str) {}
}
