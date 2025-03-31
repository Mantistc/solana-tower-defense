use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};
use solana_sdk::signer::Signer;

use crate::{solana::*, tower_building::GameState};

pub fn spawn_how_to_play_ui(mut commands: Commands) {
    let root_ui = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            Name::new("how to play ui"),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        ))
        .id();

    let create_text = |commands: &mut Commands, text: &str, font_size: f32, bottom_padding: f32| {
        commands.entity(root_ui).with_children(|p| {
            p.spawn((
                Text::new(text),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(WHITE.into()),
            ));
        });
        commands.entity(root_ui).with_children(|p| {
            p.spawn(Node {
                height: Val::Px(bottom_padding),
                ..default()
            });
        });
    };

    let _how_to_play_header = create_text(&mut commands, "How to Play", 35.0, 25.0);

    let _explaining = create_text(
        &mut commands,
        "Click on the highlighted zones to place towers and stop the enemies.",
        15.0,
        10.0,
    );

    let _explaining = create_text(
        &mut commands,
        "You can build three types of towers: Lich, Zigurat, and Electric. Each has its own strengths.",
        15.0,
        25.0,
    );

    let _explaining = create_text(
        &mut commands,
        "Use these keys to pick your tower:",
        15.0,
        35.0,
    );

    let _explaining = create_text(
        &mut commands,
        "Key Q - Lich Tower: Cheap and reliable.",
        15.0,
        10.0,
    );

    let _explaining = create_text(
        &mut commands,
        "Key W - Zigurat Tower: Hits fast, keeps up the pressure.",
        15.0,
        10.0,
    );

    let _explaining = create_text(
        &mut commands,
        "Key E - Necro Tower: Slow but deals heavy damage.",
        15.0,
        35.0,
    );

    let _explaining = create_text(
        &mut commands,
        "Green highlight means you can build, red means you don't have enough gold.",
        15.0,
        25.0,
    );

    let _explaining = create_text(
        &mut commands,
        "You've got 15 seconds between waves to build and upgrade your defenses.",
        15.0,
        10.0,
    );

    let _explaining = create_text(
        &mut commands,
        "Towers attack automatically, always aiming for the enemies closest to the goal.",
        15.0,
        10.0,
    );

    let _explaining = create_text(
        &mut commands,
        "Defeat enemies to earn gold and spend it on new towers or upgrades.",
        15.0,
        10.0,
    );

    let _button = commands.entity(root_ui).with_children(|parent| {
        parent
            .spawn((
                Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(BLACK.into()),
                BorderRadius::MAX,
                BackgroundColor(Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 0.5))),
            ))
            .with_child((
                Text::new("Start"),
                TextFont {
                    font_size: 23.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.0, 0.0)),
            ));
    });
}

pub fn handle_btn_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut TextColor>,
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    entities: Query<(Entity, &Name), With<Node>>,
    wallet: ResMut<Wallet>,
    mut tasks: ResMut<Tasks>,
    client: Res<SolClient>,
    mut player_info: ResMut<PlayerInfo>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text_color = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                let mut entity_to_despawn = None;

                if let Some((entity, _)) = entities
                    .iter()
                    .find(|(_, name)| name.as_str() == "game over")
                {
                    game_state.set(GameState::Building);
                    entity_to_despawn = Some(entity);
                }

                if let Some((entity, _)) = entities
                    .iter()
                    .find(|(_, name)| name.as_str() == "start ui")
                {
                    sign_message(&wallet);
                    game_state.set(GameState::HowToPlay);
                    entity_to_despawn = Some(entity);
                }

                if let Some((entity, _)) = entities
                    .iter()
                    .find(|(_, name)| name.as_str() == "how to play ui")
                {
                    let signer = wallet.keypair.clone();
                    let signer_pubkey = signer.pubkey();
                    let (player, bump) = player_info.set_address(&signer_pubkey);
                    tasks.add_task(initialize_player(signer, client.clone(), player, bump));
                    game_state.set(GameState::Building);
                    entity_to_despawn = Some(entity);
                }

                if let Some(entity) = entity_to_despawn {
                    commands.entity(entity).despawn_recursive();
                }
            }
            Interaction::Hovered => {
                *color = BLACK.into();
                border_color.0 = WHITE.into();
                text_color.0 = WHITE.into();
            }
            Interaction::None => {
                *color = WHITE.into();
                border_color.0 = BLACK.into();
                text_color.0 = BLACK.into();
            }
        }
    }
}
