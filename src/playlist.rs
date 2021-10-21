use surf::Client;

pub struct PlaylistService {
  client: Client,
}

impl PlaylistService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn get(&self) {}

  pub fn get_cover_images(&self) {}

  pub fn get_tracks(&self) {}

  pub fn get_user_playlists(&self) {}
}
