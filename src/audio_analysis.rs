use surf::Client;

pub struct AudioAnalysisService {
  client: Client,
}

impl AudioAnalysisService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
