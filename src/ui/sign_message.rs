use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};
use solana_sdk::signer::Signer;

use crate::solana::*;

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

    let create_text = |commands: &mut Commands, text: &str, font_size: f32| {
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
    };

    let add_top_padding = |commands: &mut Commands, parent: Entity, px: f32| {
        commands.entity(parent).with_children(|p| {
            p.spawn(Node {
                height: Val::Px(px),
                ..default()
            });
        });
    };

    let _sign_message_header = create_text(&mut commands, "Start", 35.0);
    add_top_padding(&mut commands, root_ui, 25.0);

    let _message = create_text(&mut commands, MESSAGE, 15.0);
    add_top_padding(&mut commands, root_ui, 25.0);

    let _signer_address = create_text(
        &mut commands,
        &format!("Signer address: {}", wallet.keypair.pubkey().to_string()),
        15.0,
    );
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
