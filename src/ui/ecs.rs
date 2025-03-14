use bevy::{color::palettes::css::WHITE, prelude::*};
use solana_sdk::native_token::LAMPORTS_PER_SOL;

use crate::{
    enemies::WaveControl,
    solana::{WalletBalance, WALLET},
    tower_building::{Gold, Lifes},
};

pub struct UiPlugin;

#[derive(Component)]
pub enum TextType {
    GoldText,
    WaveCountText,
    LifesText,
    WalletBalanceText,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, update_ui_texts);
    }
}

fn spawn_game_ui(mut commands: Commands) {
    // think of this root_ui like a div in html that wraps all the other divs xd
    // it defines where the ui will be positioned, and from there, you spawn
    // the rest of the components as children. Pretty much like how you'd do it in html
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
                left: Val::Percent(44.0),
                top: Val::Percent(5.0),
                ..default()
            },
            Transform::from_translation(Vec3::new(-100.0, 0.0, 0.0)),
            Name::new("UI Root"),
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

    let _gold_text = commands.entity(root_ui).with_children(|parent| {
        parent.spawn((
            Text::new("Gold: 0"),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
            TextColor(WHITE.into()),
            TextType::GoldText,
        ));
    });

    add_top_padding(&mut commands, root_ui, 10.0);

    let _wave_count_text = commands.entity(root_ui).with_children(|parent| {
        parent.spawn((
            Text::new("Wave Count: 0"),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Right),
            TextColor(WHITE.into()),
            TextType::WaveCountText,
        ));
    });

    add_top_padding(&mut commands, root_ui, 10.0);

    let _lifes = commands.entity(root_ui).with_children(|parent| {
        parent.spawn((
            Text::new("Lifes: 30"),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Right),
            TextColor(WHITE.into()),
            TextType::LifesText,
        ));
    });

    add_top_padding(&mut commands, root_ui, 10.0);

    let wallet_str = WALLET.to_string();
    let shortened_wallet = format!(
        "{}...{}",
        &wallet_str[0..6],
        &wallet_str[wallet_str.len() - 6..]
    );

    let _wallet_address = commands.entity(root_ui).with_children(|parent| {
        parent.spawn((
            Text::new(format!("Wallet Address: {}", shortened_wallet)),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Right),
            TextColor(WHITE.into()),
        ));
    });

    add_top_padding(&mut commands, root_ui, 10.0);

    let _sol_balance = commands.entity(root_ui).with_children(|parent| {
        parent.spawn((
            Text::new("Sol Balance: 0.0"),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Right),
            TextColor(WHITE.into()),
            TextType::WalletBalanceText,
        ));
    });
}

pub fn update_ui_texts(
    mut texts: Query<(&mut Text, &TextType)>,
    resources: (Res<Gold>, Res<Lifes>, Res<WalletBalance>, Res<WaveControl>),
) {
    let (gold, lifes, wallet_balance, wave_control) = resources;
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
                    wallet_balance.0 as f32 / LAMPORTS_PER_SOL as f32
                )
            }
        }
    }
}
