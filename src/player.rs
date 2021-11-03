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

  pub async fn get_currently_playing(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_recently_played(&self) -> Result<(), surf::Error> {
    Ok(())
  }
}
