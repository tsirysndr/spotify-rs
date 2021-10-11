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
}
