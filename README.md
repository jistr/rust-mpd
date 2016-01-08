# rust-mpd <a href="https://travis-ci.org/kstep/rust-mpd"><img src="https://img.shields.io/travis/kstep/rust-mpd.png?style=flat-square" /></a> <a href="https://crates.io/crates/mpd"><img src="https://img.shields.io/crates/d/mpd.png?style=flat-square" /></a> <a href="https://crates.io/crates/mpd"><img src="https://img.shields.io/crates/v/mpd.png?style=flat-square" /></a>

Pure Rust version of [libmpdclient](http://www.musicpd.org/libs/libmpdclient/).

[Full documentation](http://kstep.me/rust-mpd/mpd/index.html)

## Example

Add to `Cargo.toml`:

```toml
[dependencies]
mpd = "*"
```

Then just use:

```rust
extern crate mpd;

use mpd::Client;
use std::net::TcpStream;

let mut conn = Client::connect("127.0.0.1:6600").unwrap();
conn.volume(100).unwrap();
conn.load("My Lounge Playlist", ..).unwrap();
conn.play().unwrap();
println!("Status: {:?}", conn.status());
```
