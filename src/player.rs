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
}
