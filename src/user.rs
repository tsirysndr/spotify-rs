use crate::album::ExternalUrls;
use crate::album::Image;
use crate::artist::Followers;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Profile {
  pub display_name: Option<String>,
  pub external_urls: Option<ExternalUrls>,
  pub href: Option<String>,
  pub id: String,
  pub followers: Option<Followers>,
  pub images: Option<Vec<Image>>,
  pub r#type: String,
  pub uri: Option<String>,
}
