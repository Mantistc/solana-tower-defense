use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    enemies::WaveControl,
    tower_building::{Gold, Lifes},
};

pub struct UiPlugin;

#[derive(Component)]
pub struct GoldText;

#[derive(Component)]
pub struct WaveCountText;

#[derive(Component)]
pub struct LifesText;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui).add_systems(
            Update,
            (update_gold_text, update_lifes_text, update_wave_count_text),
        );
    }
}

fn spawn_game_ui(mut commands: Commands) {
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
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
            GoldText,
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
            WaveCountText,
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
            LifesText,
        ));
    });
}

pub fn update_gold_text(mut text: Query<&mut Text, With<GoldText>>, gold: Res<Gold>) {
    if let Ok(gold_text) = &mut text.get_single_mut() {
        gold_text.0 = format!("Gold: {:?}", gold.0);
    }
}

pub fn update_lifes_text(mut text: Query<&mut Text, With<LifesText>>, lifes: Res<Lifes>) {
    if let Ok(lifes_text) = &mut text.get_single_mut() {
        lifes_text.0 = format!("Lifes: {:?}", lifes.0);
    }
}

pub fn update_wave_count_text(
    mut text: Query<&mut Text, With<WaveCountText>>,
    wave_count: Res<WaveControl>,
) {
    if let Ok(wave_count_text) = &mut text.get_single_mut() {
        wave_count_text.0 = format!("Wave count: {}", wave_count.wave_count + 1);
    }
}
