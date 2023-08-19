//! [Bevy](https://github.com/bevyengine/bevy) plugin for [paperdoll](https://github.com/fralonra/paperdoll), a 2D pixel-based stationary paper doll model.
//!
//! ## Usage
//!
//! Add the plugin to your app first.
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_paperdoll::{PaperdollAsset, PaperdollPlugin};
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(PaperdollPlugin);
//!
//!     // Other logic
//!     // ...
//! }
//! ```
//!
//! Then load a paperdoll file and store the handle. You can use [ppd-editor](https://github.com/fralonra/ppd-editor) to create a valid paperdoll asset.
//!
//! ```no_run
//! # use bevy::prelude::*;
//! # use bevy_paperdoll::PaperdollAsset;
//! #
//! fn load_paperdoll(asset_server: Res<AssetServer>) {
//!     let handle: Handle<PaperdollAsset> = asset_server.load("your/paperdoll.ppd");
//!
//!     // Store the handle.
//!     // ...
//! }
//! ```
//!
//! Create a paperdoll from the loaded asset and play with it.
//!
//! ```no_run
//! # use bevy::prelude::*;
//! # use bevy_paperdoll::PaperdollAsset;
//! #
//! # #[derive(Default, Resource)]
//! # struct Resources(Handle<PaperdollAsset>);
//! #
//! fn create_paperdoll(
//!     mut paperdolls: ResMut<Assets<PaperdollAsset>>,
//! #   resources: Res<Resources>,
//! ) {
//! #   let handle = resources.0.clone();
//! #
//!     // Access the paperdoll asset using the handle stored previously.
//!     let paperdoll_asset = paperdolls.get_mut(&handle).unwrap();
//!
//!     // Create a paperdoll based on doll 0.
//!     // The returned id will be used to refer to this paperdoll in the following process.
//!     let paperdoll_id = paperdoll_asset.create_paperdoll(0);
//!
//!     // Do something with the paperdoll just created.
//!     
//!     // eg. Set slot 0 to fragment 1
//!     // paperdoll_asset.slot_use_fragment(paperdoll_id, 0, 1);
//!     
//!     // eg. Set slot 1 to empty
//!     // paperdoll_asset.slot_use_empty(paperdoll_id, 1);
//!
//!     // Get the image to be drawn on the screen.
//!     let paperdoll_image = paperdoll_asset.take_texture(paperdoll_id).unwrap();
//! }
//! ```
//!
//! See [examples](https://github.com/fralonra/bevy-paperdoll/blob/master/examples/README.md) for more.

mod asset;
mod loader;
mod plugin;

pub use asset::{PaperdollAsset, PaperdollId};
pub use plugin::PaperdollPlugin;
