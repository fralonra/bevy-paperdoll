mod loading;

use bevy::prelude::*;
use bevy_paperdoll::{PaperdollAsset, PaperdollPlugin};
use in_game::InGamePlugin;
use loading::LoadingPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Loading,
    InGame,
}

#[derive(Default, Resource)]
struct Resources {
    asset: Handle<PaperdollAsset>,
}

mod in_game {
    use bevy::prelude::*;
    use bevy_paperdoll::PaperdollAsset;

    use super::{GameState, Resources};

    pub struct InGamePlugin;

    impl Plugin for InGamePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::InGame), setup_ui);
        }
    }

    fn setup_ui(
        mut commands: Commands,
        mut images: ResMut<Assets<Image>>,
        mut paperdolls: ResMut<Assets<PaperdollAsset>>,
        resources: Res<Resources>,
    ) {
        commands.spawn(Camera2d::default());

        let paperdoll_asset = paperdolls.get_mut(&resources.asset).unwrap();

        let paperdoll_id = paperdoll_asset.create_paperdoll(0);

        let paperdoll_image = paperdoll_asset.take_texture(paperdoll_id).unwrap();

        commands
            .spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((ImageNode {
                    image: images.add(paperdoll_image),
                    ..default()
                },));
            });
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PaperdollPlugin))
        .init_state::<GameState>()
        .init_resource::<Resources>()
        .add_plugins((LoadingPlugin, InGamePlugin))
        .run();
}
