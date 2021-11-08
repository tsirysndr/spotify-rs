use spotify_rs::Spotify;

#[tokio::main]
async fn main() {
  const ACCESS_TOKEN: &str = "BQBIJbmSq7rJzfK1vqxYIG3UM0KxbdIDtKRaUVo6DgzbSJjXGpx7SFX5ht04R7-3uGX4KDrykUh5Vsl4cdf46NWNzHY3qW1QtbNLtjk1NFBRDZpSMW0rahAHx-RXb0Vzubgmcd_LLVHuE2PnQFwnOVxHLGZU0sQ";
  let ids: &str = "382ObEPsp2rxGrnsizN5TX,1A2GTWGtFfWp7KSQTwWOyo,2noRn2Aes5aoNVsU6iWThc";
  let client = Spotify::new(ACCESS_TOKEN);
  let album = client.album.get("382ObEPsp2rxGrnsizN5TX").await;
  let albums = client.album.list(ids).await;
  let album_tracks = client
    .album
    .get_tracks("382ObEPsp2rxGrnsizN5TX", 50, 0)
    .await;
  let track = client.track.get("1Nv1h7ANN9E4rAjLP4OfgA").await;
  let tracks = client.track.list("1Nv1h7ANN9E4rAjLP4OfgA", None).await;
  let audio_features_list = client.track.audio_features("1Nv1h7ANN9E4rAjLP4OfgA").await;
  let audio_features = client
    .track
    .audio_features_by_track_id("1Nv1h7ANN9E4rAjLP4OfgA")
    .await;
  let audio_analysis = client.track.audio_analysis("1Nv1h7ANN9E4rAjLP4OfgA").await;
  let artist = client.artist.get("4tZwfgrHOc3mvqYlEYSvVi").await;
  let playlist = client.playlist.get("37i9dQZF1DXcBWIGoYBM5M").await;
  let cover_images = client
    .playlist
    .get_cover_images("37i9dQZF1DXcBWIGoYBM5M")
    .await;
  let playlist_tracks = client
    .playlist
    .get_tracks("37i9dQZF1DXcBWIGoYBM5M", 50, 0)
    .await;
  let user_playlists = client.playlist.get_user_playlists("smedja", 50, 0).await;
  println!("{:#?}\n", album.unwrap());
  println!("{:#?}\n", albums.unwrap());
  println!("{:#?}\n", album_tracks.unwrap());
  println!("{:#?}\n", track.unwrap());
  println!("{:#?}\n", tracks.unwrap());
  println!("{:#?}\n", audio_features_list.unwrap());
  println!("{:#?}\n", audio_features.unwrap());
  println!("{:#?}\n", audio_analysis.unwrap());
  println!("{:#?}\n", artist.unwrap());
  println!("{:#?}\n", playlist.unwrap());
  println!("{:#?}\n", cover_images.unwrap());
  println!("{:#?}\n", playlist_tracks.unwrap());
  println!("{:#?}\n", user_playlists.unwrap());
}
