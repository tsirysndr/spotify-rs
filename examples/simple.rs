use spotify_rs::Spotify;

#[tokio::main]
async fn main() {
  const ACCESS_TOKEN: &str = "BQAcmV4WRbj2c8ZRR6P1jqfjO_JC7LcIgJKOnGhLMoxCQ9jj8xO136vz6YUPRFi3sWfcG5-IQX_YSnbAsTSDBaoYuKLv-Ew2NcFgJ1matE9Lp6Lo5mRUdqOJqSSrbezpGLomVafJIYop0EIrFjKukCi_KUdDkOQ";
  let client = Spotify::new(ACCESS_TOKEN);
  let album = client.album.get("382ObEPsp2rxGrnsizN5TX").await;
  println!("{:#?}\n", album.unwrap());
}
