use bevy::prelude::*;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, signer::Signer};

use crate::{
    enemies::WaveControl,
    solana::Wallet,
    tower_building::{GameState, Gold, Lifes},
};

use super::*;

pub struct UiPlugin;

#[derive(Component)]
pub enum TextType {
    GoldText,
    WaveCountText,
    LifesText,
    WalletBalanceText,
    TimeToBuild,
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

    let create_text = |commands: &mut Commands, text: &str, text_type: Option<TextType>| {
        commands.entity(root_ui).with_children(|p| {
            if let Some(text_type_value) = text_type {
                p.spawn((
                    Text::new(text),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextLayout::new_with_justify(JustifyText::Center),
                    TextColor(border_and_text_color),
                    text_type_value,
                ));
            }
        });
    };

    let _gold_text = create_text(&mut commands, "Gold: 0", Some(TextType::GoldText));

    add_top_padding(&mut commands, 10.0);
    let _wave_count_text = create_text(
        &mut commands,
        "Wave Count: 0",
        Some(TextType::WaveCountText),
    );

    add_top_padding(&mut commands, 10.0);
    let _lifes_text = create_text(&mut commands, "Lifes: 30", Some(TextType::LifesText));

    add_top_padding(&mut commands, 10.0);
    let _lifes_text = create_text(
        &mut commands,
        "Time to build: 15.0 secs",
        Some(TextType::TimeToBuild),
    );

    add_top_padding(&mut commands, 35.0);
    let _sol_balance_text = create_text(
        &mut commands,
        "Sol Balance: 0.0",
        Some(TextType::WalletBalanceText),
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
        None,
    );
}

// Update in real-time the UI texts with the resources states
pub fn update_ui_texts(
    mut texts: Query<(&mut Text, &TextType)>,
    resources: (Res<Gold>, Res<Lifes>, Res<Wallet>, ResMut<WaveControl>),
    time: Res<Time>,
) {
    let (gold, lifes, wallet, mut wave_control) = resources;
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
            TextType::TimeToBuild => {
                if !wave_control.time_between_waves.paused() {
                    wave_control.time_between_waves.tick(time.delta());
                    text.0 = format!(
                        "Time to Build: {:.2} secs",
                        wave_control.time_between_waves.remaining_secs()
                    );
                }
            }
        }
    }
}
