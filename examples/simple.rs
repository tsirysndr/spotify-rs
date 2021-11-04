use spotify_rs::Spotify;

#[tokio::main]
async fn main() {
  const ACCESS_TOKEN: &str =
    "BQAP150-AwblglOf5wk5YYlzp_Njg4e5EQKwSdksfVxmXi9qczVLk2mVmRjaFQEgAM2u6lzWfBLt3jNYv200fxuBWz5IKJNxING96eSqJUM7bYx2clbTPvh13StT2pSSm7NzAdhr3PGK7BVh-s8jiz-WqW5_9JM";
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
  println!("{:#?}\n", album.unwrap());
  println!("{:#?}\n", albums.unwrap());
  println!("{:#?}\n", album_tracks.unwrap());
  println!("{:#?}\n", track.unwrap());
  println!("{:#?}\n", tracks.unwrap());
  println!("{:#?}\n", audio_features_list.unwrap());
  println!("{:#?}\n", audio_features.unwrap());
  println!("{:#?}\n", audio_analysis.unwrap());
}
