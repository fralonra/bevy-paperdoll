# bevy-paperdoll

[![Latest version](https://img.shields.io/crates/v/bevy-paperdoll.svg)](https://crates.io/crates/bevy-paperdoll)
[![Documentation](https://docs.rs/bevy-paperdoll/badge.svg)](https://docs.rs/bevy-paperdoll)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![CI](https://github.com/fralonra/bevy-paperdoll/actions/workflows/build.yml/badge.svg)](https://github.com/fralonra/bevy-paperdoll/actions)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

[Bevy](https://github.com/bevyengine/bevy) plugin for [paperdoll](https://github.com/fralonra/paperdoll), a 2D pixel-based stationary paper doll model.

<p align="center">
	<img alt="screenshot" src="https://raw.githubusercontent.com/fralonra/bevy-paperdoll/master/doc/screenshot.gif" width="600" />
</p>

## Usage

Add the plugin to your app first.

```rust
use bevy::prelude::*;
use bevy_paperdoll::{PaperdollAsset, PaperdollPlugin};

fn main() {
    App::new()
        .add_plugins(PaperdollPlugin);

    // Other logic
    // ...
}
```

Then load a paperdoll file and store the handle. You can use [ppd-editor](https://github.com/fralonra/ppd-editor) to create a valid paperdoll asset.

```rust
fn load_paperdoll(asset_server: Res<AssetServer>) {
    let handle = asset_server.load("your/paperdoll.ppd");

    // Store the handle.
    // ...
}
```

Create a paperdoll from the loaded asset and play with it.

```rust
fn create_paperdoll(
    mut paperdolls: ResMut<Assets<PaperdollAsset>>,
) {
    // Access the paperdoll asset using the handle stored previously.
    let paperdoll_asset = paperdolls.get_mut(&handle).unwrap();

    // Create a paperdoll based on doll 0.
    // The returned id will be used to refer to this paperdoll in the following process.
    let paperdoll_id = paperdoll_asset.create_paperdoll(0);

    // Do something with the paperdoll just created.

    // eg. Set slot 0 to fragment 1
    // paperdoll_asset.slot_use_fragment(paperdoll_id, 0, 1);

    // eg. Set slot 1 to empty
    // paperdoll_asset.slot_use_empty(paperdoll_id, 1);

    // Get the image to be drawn on the screen.
    let paperdoll_image = paperdoll_asset.take_texture(paperdoll_id).unwrap();
}
```

See [examples](examples/README.md) for more.

## Bevy Compatibility

| bevy | bevy-paperdoll |
| ---- | -------------- |
| 0.13 | 0.3            |
| 0.12 | 0.2            |
| 0.11 | 0.1            |
