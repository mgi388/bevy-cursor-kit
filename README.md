# Bevy cursor kit

[![Crates.io](https://img.shields.io/crates/v/bevy_cursor_kit.svg)](https://crates.io/crates/bevy_cursor_kit)
[![Docs.rs](https://docs.rs/bevy_cursor_kit/badge.svg)](https://docs.rs/bevy_cursor_kit)
[![CI](https://github.com/mgi388/bevy-cursor-kit/workflows/CI/badge.svg)](https://github.com/mgi388/bevy-cursor-kit/actions)

## Summary

Allows you to load .CUR and .ANI cursor files in your Bevy app and use them in custom `CursorIcon`s.

- .CUR files can be used for static cursor icons like a grabbing hand.
- .ANI files can be used for animated cursor icons like an hourglass.

## Quick start

Add the asset plugin for asset loader support:

```rust
use bevy_cursor_kit::prelude::*;

app.add_plugins(CursorAssetPlugin);
```

Load a static cursor or an animated cursor:

```rust
let handle = asset_server.load("example.CUR");
```

When the asset is ready, use its image when creating a custom `CursorIcon` component on your `Window`s:

```rust
let Some(cursor) = static_cursors.get(&handle) else {
  // ...
};

commands
  .entity(window)
  .insert(CursorIcon::Custom(CustomCursor::Image {
      handle: cursor.image.clone(),
      // Most .CUR are expected to only have one frame so just use index 0.
      hotspot: cursor.hotspot_or_default(0),
  }));
```

Check out the [examples](examples) for more details.

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
