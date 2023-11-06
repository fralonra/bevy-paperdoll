use bevy::prelude::*;

use crate::{asset::PaperdollAsset, loader::PaperdollLoader};

/// Bevy plugin for paperdoll.
#[derive(Default)]
pub struct PaperdollPlugin;

impl Plugin for PaperdollPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<PaperdollAsset>()
            .init_asset_loader::<PaperdollLoader>();
    }
}
