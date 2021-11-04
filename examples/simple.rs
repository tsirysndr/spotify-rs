use spotify_rs::Spotify;

#[tokio::main]
async fn main() {
  const ACCESS_TOKEN: &str =
    "BQDFQyS-_R-spZt5etVENQ1iBK1cmIY-WgofunNBZJ048d0HpMdBcV3bGEob1smf8g3lTwA2uEa-tZBWJt6bIcudaCOdPN989dfx4fU_0e2ravM7P24GFA6u0M0IRTLxPXEcugt-FaPFOp9LGbjtp4LJObwpIwo";
  let ids: &str = "382ObEPsp2rxGrnsizN5TX,1A2GTWGtFfWp7KSQTwWOyo,2noRn2Aes5aoNVsU6iWThc";
  let client = Spotify::new(ACCESS_TOKEN);
  let album = client.album.get("382ObEPsp2rxGrnsizN5TX").await;
  let albums = client.album.list(ids).await;
  let album_tracks = client
    .album
    .get_tracks("382ObEPsp2rxGrnsizN5TX", 50, 0)
    .await;
  println!("{:#?}\n", album.unwrap());
  println!("{:#?}\n", albums.unwrap());
  println!("{:#?}\n", album_tracks.unwrap());
}
