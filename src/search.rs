use surf::Client;

pub struct SearchService {
  client: Client,
}

impl SearchService {
  pub fn new(client: &Client) -> SearchService {
    SearchService {
      client: client.clone(),
    }
  }
}
