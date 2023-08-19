use bevy::prelude::*;
use bevy_paperdoll::PaperdollAsset;

use super::{GameState, Resources};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                check_assets_loaded.run_if(in_state(GameState::Loading)),
            );
    }
}

// check if the asset has been loaded
fn check_assets_loaded(
    mut game_state: ResMut<NextState<GameState>>,
    paperdolls: Res<Assets<PaperdollAsset>>,
    resources: Res<Resources>,
) {
    if paperdolls.get(&resources.asset).is_some() {
        game_state.set(GameState::InGame);
    }
}

// load paperdoll assets
fn load_assets(asset_server: Res<AssetServer>, mut resources: ResMut<Resources>) {
    resources.asset = asset_server.load("basic.ppd");
}
