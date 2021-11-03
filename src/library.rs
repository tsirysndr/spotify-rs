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

  pub async fn get_current_user_saved_tracks(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_current_user_saved_albums(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn check_current_user_saved_tracks(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn check_current_user_saved_albums(&self) -> Result<(), surf::Error> {
    Ok(())
  }
}
