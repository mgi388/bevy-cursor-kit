# `bevy_cursor_kit`

[![Crates.io](https://img.shields.io/crates/v/bevy_cursor_kit.svg)](https://crates.io/crates/bevy_cursor_kit)
[![Docs.rs](https://docs.rs/bevy_cursor_kit/badge.svg)](https://docs.rs/bevy_cursor_kit)
[![CI](https://github.com/mgi388/bevy-cursor-kit/workflows/CI/badge.svg)](https://github.com/mgi388/bevy-cursor-kit/actions)

## Summary

`bevy_cursor_kit` is a crate for Bevy apps that lets you load cursor assets in various formats and use them as custom `CursorIcon`s.

## Features

### `.CUR` and `.ANI` binary formats

Load the classic Microsoft Windows `.CUR` and `.ANI` cursor file formats.

- `.CUR` files can be used for static cursor icons like a grabbing hand.
- `.ANI` files can be used for animated cursor icons like an hourglass.

### `.cur.json` ,`.cur.ron`, `.cur.toml`, `.ani.json` ,`.ani.ron`, `.ani.toml` text formats

Text-based versions of the classic `.CUR` static cursor and `.ANI` animated cursor file formats.

Write your cursors in JSON, RON, or TOML and `bevy_cursor_kit` can load them for you.

#### Static cursor

```ron
(
    image: (
        path: "path/to/sprite-sheet.png",
    ),
    texture_atlas_layout: (
        tile_size: (32, 32),
        columns: 20,
        rows: 10,
    ),
    hotspots: (
        default: (0, 0),
        overrides: {
            11: (32, 32),
            95: (32, 8),
        },
    ),
)
```

Check out the [cur_ron_asset.rs example](example/cur_ron_asset.rs) for more details.

#### Animated cursor

```ron
(
    image: (
        path: "path/to/sprite-sheet.png",
    ),
    texture_atlas_layout: (
        tile_size: (32, 32),
        columns: 2,
        rows: 2,
    ),
    hotspots: (
        default: (0, 0),
    ),
    animation: (
        repeat: Loop,
        clips: [
            (
                atlas_indices: [3, 0, 1, 2],
                duration: PerFrame(75),
            ),
            (
                atlas_indices: [2],
                duration: PerFrame(5000),
            ),
        ],
    )
)
```

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

When the asset is ready, use its `image` when creating a custom `CursorIcon` component on your `Window`s:

```rust
let Some(cursor) = static_cursors.get(&handle) else {
  // ...
};

commands
  .entity(window)
  .insert(CursorIcon::Custom(
    CustomCursorImageBuilder::from_static_cursor(cursor, None).build(),
  ));
```

If you want to use the text-based formats, enable the `serde_json_asset`, `serde_ron_asset`, or `serde_toml_asset` feature in your `Cargo.toml` and load away:

```rust
let handle = asset_server.load("example.cur.ron");
```

Check out the [examples](examples) for more details.

## Version compatibility

> [!WARNING]
> `bevy_cursor_kit@0.3` is compatible with `bevy@0.15` and allows you to decode `.CUR` and `.ANI` files. Most of the benefits of this crate will come when `bevy@0.16` is released, so use `main` if you can.

| bevy | bevy_cursor_kit |
| ---- | --------------- |
| main | main            |
| 0.15 | 0.3             |

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
