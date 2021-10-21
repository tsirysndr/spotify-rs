use surf::Client;

pub struct PlayerService {
  client: Client,
}

impl PlayerService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn get_currently_playing(&self) {}

  pub fn get_recently_played(&self) {}
}
