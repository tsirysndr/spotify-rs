
use surf::Client;

pub struct MeService {
  client: Client,
}

impl MeService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
