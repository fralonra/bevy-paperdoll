mod loading;

use bevy::prelude::*;
use bevy_paperdoll::{PaperdollAsset, PaperdollId, PaperdollPlugin};
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
    paperdoll: PaperdollId,
}

mod in_game {
    use bevy::{ecs::spawn::SpawnWith, prelude::*};
    use bevy_paperdoll::PaperdollAsset;

    use super::{GameState, Resources};

    #[derive(Component)]
    enum ButtonAction {
        Next(u32),
        Prev(u32),
    }

    #[derive(Message)]
    struct PaperdollChangedEvent(u32);

    #[derive(Component)]
    struct PaperdollUiImage;

    #[derive(Component)]
    struct TextForSlotFragment(u32);

    pub struct InGamePlugin;

    impl Plugin for InGamePlugin {
        fn build(&self, app: &mut App) {
            app.add_message::<PaperdollChangedEvent>()
                .add_systems(OnEnter(GameState::InGame), setup_ui)
                .add_systems(
                    Update,
                    (button_action, paperdoll_update).run_if(in_state(GameState::InGame)),
                );
        }
    }

    // Handles user interactions.
    fn button_action(
        interaction_query: Query<
            (&Interaction, &ButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut ev_paperdoll: MessageWriter<PaperdollChangedEvent>,
        mut paperdolls: ResMut<Assets<PaperdollAsset>>,
        resources: Res<Resources>,
    ) {
        for (interaction, button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                if let Some(paperdoll_asset) = paperdolls.get_mut(&resources.asset) {
                    let paperdoll_id = resources.paperdoll;

                    if let Ok(slot_id) = match button_action {
                        ButtonAction::Next(slot_id) => paperdoll_asset
                            .slot_use_next(paperdoll_id, *slot_id)
                            .map(|_| slot_id),
                        ButtonAction::Prev(slot_id) => paperdoll_asset
                            .slot_use_prev(paperdoll_id, *slot_id)
                            .map(|_| slot_id),
                    } {
                        ev_paperdoll.write(PaperdollChangedEvent(*slot_id));
                    }
                }
            }
        }
    }

    // Updates texture and text.
    fn paperdoll_update(
        mut image_query: Query<&mut ImageNode, With<PaperdollUiImage>>,
        mut text_query: Query<(&mut Text, &TextForSlotFragment)>,
        mut ev_paperdoll: MessageReader<PaperdollChangedEvent>,
        mut images: ResMut<Assets<Image>>,
        mut paperdolls: ResMut<Assets<PaperdollAsset>>,
        resources: Res<Resources>,
    ) {
        if let Some(paperdoll_asset) = paperdolls.get_mut(&resources.asset) {
            for ev in ev_paperdoll.read() {
                if let Some(paperdoll_image) = paperdoll_asset.take_texture(resources.paperdoll) {
                    let texture = images.add(paperdoll_image);

                    for mut image in image_query.iter_mut() {
                        *image = ImageNode::new(texture.clone());
                    }

                    let slot_id = ev.0;

                    for (mut text, text_for_slot) in text_query.iter_mut() {
                        if text_for_slot.0 == slot_id {
                            let desc = paperdoll_asset
                                .get_slot_fragment(resources.paperdoll, slot_id)
                                .map(|fragment| fragment.desc.as_str())
                                .unwrap_or("-");

                            text.0 = desc.to_owned();

                            break;
                        }
                    }
                }
            }
        }
    }

    fn setup_ui(
        mut commands: Commands,
        mut images: ResMut<Assets<Image>>,
        mut paperdolls: ResMut<Assets<PaperdollAsset>>,
        mut resources: ResMut<Resources>,
    ) {
        let text_font = TextFont {
            font_size: 24.0,
            ..default()
        };

        commands.spawn(Camera2d::default());

        let Some(paperdoll_asset) = paperdolls.get_mut(&resources.asset) else {
            commands.spawn((Text::new("Failed to load assets"), text_font.clone()));
            return;
        };

        let paperdoll_id = paperdoll_asset.create_paperdoll(0);

        resources.paperdoll = paperdoll_id;

        let Some(paperdoll_image) = paperdoll_asset.take_texture(paperdoll_id) else {
            commands.spawn((Text::new("Failed to load textures"), text_font.clone()));
            return;
        };

        let texture_width = paperdoll_image.size().x;
        let texture = images.add(paperdoll_image);

        let slots = paperdoll_asset.get_slots(paperdoll_id);
        let slot_items = slots
            .iter()
            .map(|slot| {
                (
                    slot.id(),
                    slot.desc.clone(),
                    paperdoll_asset
                        .get_slot_fragment(paperdoll_id, slot.id())
                        .map(|fragment| fragment.desc.as_str())
                        .unwrap_or("-")
                        .to_owned(),
                )
            })
            .collect::<Vec<(u32, String, String)>>();

        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![
                // display image
                (
                    Node {
                        width: Val::Percent(50.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    children![(
                        ImageNode {
                            image: texture,
                            ..default()
                        },
                        Node {
                            width: Val::Percent(80.0),
                            max_width: Val::Px(texture_width as f32),
                            ..default()
                        },
                        PaperdollUiImage,
                    )]
                ),
                (
                    Node {
                        width: Val::Percent(30.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    // ui for each slot
                    Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                        for (id, desc, fragment_desc) in slot_items {
                            // slot description
                            parent.spawn((Text::new(desc), text_font.clone()));

                            parent.spawn((
                                Node {
                                    width: Val::Percent(80.0),
                                    min_width: Val::Px(200.0),
                                    max_width: Val::Px(300.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::SpaceBetween,
                                    margin: UiRect {
                                        top: Val::Px(5.0),
                                        bottom: Val::Px(40.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                children![
                                    // prev button
                                    (
                                        Button,
                                        Node {
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        BackgroundColor(Color::NONE),
                                        ButtonAction::Prev(id),
                                        children![(Text::new("<"), text_font.clone())]
                                    ),
                                    // fragment description
                                    //
                                    // Displays the description of fragment currently used for this slot.
                                    // If this slot is empty, shows '-' instead.
                                    (
                                        Text::new(fragment_desc),
                                        text_font.clone(),
                                        TextForSlotFragment(id),
                                    ),
                                    // next button
                                    (
                                        Button,
                                        Node {
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        BackgroundColor(Color::NONE),
                                        ButtonAction::Next(id),
                                        children![(Text::new(">"), text_font.clone())]
                                    )
                                ],
                            ));
                        }
                    }))
                )
            ],
        ));
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
