use surf::Client;

pub struct LibraryService {
  client: Client,
}

impl LibraryService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn get_current_user_saved_tracks(&self) {}

  pub fn get_current_user_saved_albums(&self) {}

  pub fn check_current_user_saved_tracks(&self) {}

  pub fn check_current_user_saved_albums(&self) {}
}
