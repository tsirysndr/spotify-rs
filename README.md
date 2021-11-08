<h1 align="left">spotify-rs</h1>
<p>
  <a href="https://github.com/tsirysndr/spotify-rs/blob/master/LICENSE"" target="_blank">
    <img alt="License: BSD" src="https://img.shields.io/badge/License-BSD-green.svg" />
  </a>
  <a href="https://github.com/tsirysndr/spotify-rs/commits/master">
    <img src="https://img.shields.io/github/last-commit/tsirysndr/spotify-rs.svg" target="_blank" />
  </a>
  <a href="https://twitter.com/tsiry_sndr" target="_blank">
    <img alt="Twitter: tsiry_sndr" src="https://img.shields.io/twitter/follow/tsiry_sndr.svg?style=social" />
  </a>
</p>

> spotify-rs is a Rust client library for accessing the [Spotify API](https://developer.spotify.com/web-api/)

## Install

```toml
[dependencies]
spotify-rs = { git = "https://github.com/tsirysndr/spotify-rs" }
```

## Usage

Construct a new Spotify client, then use the various services on the client to access different parts of the Spotify API. For example:

```rust
use spotify_rs::Spotify;

#[tokio::main]
async fn main() {
  const ACCESS_TOKEN: &str = "<YOUR ACCESS TOKEN>";
  let client = Spotify::new(ACCESS_TOKEN);
  let album = client.album.get("382ObEPsp2rxGrnsizN5TX").await;
  let results = client.search.get("Muse", "track,artist", 50, 0).await;
  println!("{:#?}\n", album);
  println!("{:#?}\n", results);
}


```

## Author

👤 **Tsiry Sandratraina**

* Website: https://tsiry-sandratraina.netlify.com
* Twitter: [@tsiry\_sndr](https://twitter.com/tsiry\_sndr)
* Github: [@tsirysndr](https://github.com/tsirysndr)

## Show your support

Give a ⭐️ if this project helped you!
