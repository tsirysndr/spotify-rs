use surf::Client;

pub struct CategoryService {
  client: Client,
}

impl CategoryService {
  pub fn new(client: &Client) -> CategoryService {
    CategoryService {
      client: client.clone(),
    }
  }
}
