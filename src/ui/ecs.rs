use bevy::{color::palettes::css::*, prelude::*};
use solana_sdk::{native_token::LAMPORTS_PER_SOL, signer::Signer};

use crate::{
    enemies::WaveControl,
    solana::{sign_message, Wallet, MESSAGE},
    tower_building::{GameState, Gold, Lifes},
};

pub struct UiPlugin;

#[derive(Component)]
pub enum TextType {
    GoldText,
    WaveCountText,
    LifesText,
    WalletBalanceText,
    WalletAddress,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sign_message_to_start)
            .add_systems(OnExit(GameState::Start), spawn_game_ui)
            .add_systems(Update, (sign_when_press_btn, update_ui_texts));
    }
}

// This part is the stats/values the player have after start the game
pub fn spawn_game_ui(mut commands: Commands, wallet: Res<Wallet>) {
    // think of this root_ui like a div in html that wraps all the other divs xd
    // it defines where the ui will be positioned, and from there, you spawn
    // the rest of the components as children. Pretty much like how you'd do it in html
    let border_and_text_color = Color::srgb(224.0 / 255.0, 162.0 / 255.0, 125.0 / 255.0);
    let root_ui = commands
        .spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Auto,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                position_type: PositionType::Absolute,
                right: Val::Percent(3.0),
                border: UiRect::all(Val::Px(5.0)),
                top: Val::Percent(60.0),
                ..default()
            },
            BorderColor(border_and_text_color),
            BorderRadius::all(Val::Px(15.0)),
            Name::new("UI Root"),
            BackgroundColor(Color::srgb(78.0 / 255.0, 43.0 / 255.0, 47.0 / 255.0)),
        ))
        .id();

    let add_top_padding = |commands: &mut Commands, px: f32| {
        commands.entity(root_ui).with_children(|p| {
            p.spawn(Node {
                height: Val::Px(px),
                ..default()
            });
        });
    };

    let create_text = |commands: &mut Commands, text: &str, text_type: TextType| {
        commands.entity(root_ui).with_children(|p| {
            p.spawn((
                Text::new(text),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                TextColor(border_and_text_color),
                text_type,
            ));
        });
    };

    let _gold_text = create_text(&mut commands, "Gold: 0", TextType::GoldText);

    add_top_padding(&mut commands, 10.0);
    let _wave_count_text = create_text(&mut commands, "Wave Count: 0", TextType::WaveCountText);

    add_top_padding(&mut commands, 10.0);
    let _lifes_text = create_text(&mut commands, "Lifes: 30", TextType::LifesText);

    add_top_padding(&mut commands, 35.0);
    let _sol_balance_text = create_text(
        &mut commands,
        "Sol Balance: 0.0",
        TextType::WalletBalanceText,
    );

    add_top_padding(&mut commands, 10.0);

    let wallet_str = wallet.keypair.pubkey().to_string();
    let shortened_wallet = format!(
        "{}...{}",
        &wallet_str[0..4],
        &wallet_str[wallet_str.len() - 4..]
    );

    let _wallet_address = create_text(
        &mut commands,
        &format!("Wallet Address: {}", shortened_wallet),
        TextType::WalletAddress,
    );
}

pub fn update_ui_texts(
    mut texts: Query<(&mut Text, &TextType)>,
    resources: (Res<Gold>, Res<Lifes>, Res<Wallet>, Res<WaveControl>),
) {
    let (gold, lifes, wallet, wave_control) = resources;
    for (mut text, text_type) in &mut texts {
        match text_type {
            TextType::GoldText => text.0 = format!("Gold: {:?}", gold.0),
            TextType::WaveCountText => {
                text.0 = format!("Wave count: {}", wave_control.wave_count + 1)
            }
            TextType::LifesText => text.0 = format!("Lifes: {:?}", lifes.0),
            TextType::WalletBalanceText => {
                text.0 = format!(
                    "Sol Balance: {:.2}",
                    wallet.balance as f32 / LAMPORTS_PER_SOL as f32
                )
            }
            TextType::WalletAddress => {}
        }
    }
}

// this UI is the **start ui** to sign the message with the keypair and change
// the `GameState` to start playing.

pub fn spawn_sign_message_to_start(mut commands: Commands, wallet: Res<Wallet>) {
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
            Name::new("start ui"),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        ))
        .id();

    let add_top_padding = |commands: &mut Commands, parent: Entity, px: f32| {
        commands.entity(parent).with_children(|p| {
            p.spawn(Node {
                height: Val::Px(px),
                ..default()
            });
        });
    };

    let _sign_message_header = commands.entity(root_ui).with_children(|parent| {
        parent.spawn((
            Text::new("Start"),
            TextFont {
                font_size: 35.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
            TextColor(WHITE.into()),
        ));
    });

    add_top_padding(&mut commands, root_ui, 25.0);

    let _message = commands.entity(root_ui).with_children(|parent| {
        parent.spawn((
            Text::new(MESSAGE),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
            TextColor(WHITE.into()),
        ));
    });

    add_top_padding(&mut commands, root_ui, 25.0);

    let _signer_address = commands.entity(root_ui).with_children(|parent| {
        parent.spawn((
            Text::new(format!(
                "Signer address: {}",
                wallet.keypair.pubkey().to_string()
            )),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
            TextColor(WHITE.into()),
        ));
    });

    add_top_padding(&mut commands, root_ui, 25.0);

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
                Text::new("Sign"),
                TextFont {
                    font_size: 23.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.0, 0.0)),
            ));
    });
}

pub fn sign_when_press_btn(
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
    wallet: Res<Wallet>,
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    entities: Query<(Entity, &Name), With<Node>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text_color = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                sign_message(&wallet);
                game_state.set(GameState::Building);
                if let Some((start_ui_entity, _)) = entities
                    .iter()
                    .find(|(_, name)| name.as_str() == "start ui")
                {
                    commands.entity(start_ui_entity).despawn_recursive();
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
