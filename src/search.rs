use crate::album::AlbumsResult;
use crate::artist::Artists;
use crate::track::AlbumTracks;
use serde::{Deserialize, Serialize};
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct SearchResult {
  pub artists: Option<Artists>,
  pub tracks: Option<AlbumTracks>,
  pub albums: Option<AlbumsResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchParams {
  pub q: String,
  pub r#type: String,
  pub limit: u32,
  pub offset: u32,
}

pub struct SearchService {
  client: Client,
}

impl SearchService {
  pub fn new(client: &Client) -> SearchService {
    SearchService {
      client: client.clone(),
    }
  }

  pub async fn get(
    &self,
    query: &str,
    filter: &str,
    limit: u32,
    offset: u32,
  ) -> Result<SearchResult, surf::Error> {
    let params = SearchParams {
      q: query.to_string(),
      r#type: filter.to_string(),
      limit,
      offset,
    };
    let res = self
      .client
      .get("search")
      .query(&params)?
      .recv_json::<SearchResult>()
      .await?;
    Ok(res)
  }
}
