
use surf::Client;

pub struct AudioFeaturesService {
  client: Client,
}

impl AudioFeaturesService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
