use bevy::prelude::*;

use crate::{enemies::WaveControl, tower_building::SelectedTowerType};

use super::*;

#[derive(Component, PartialEq, Eq)]
pub enum SelectedTowerTextTypes {
    TowerSelected,
    TowerCost,
    TimeToBuild,
}

// display a text to indicate the selected tower to buy/build
pub fn spawn_tower_selected_text(mut commands: Commands) {
    let root_ui = commands
        .spawn((
            Node {
                width: Val::Auto,
                height: Val::Auto,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                position_type: PositionType::Absolute,
                left: Val::Percent(40.5),
                border: UiRect::all(Val::Px(5.0)),
                top: Val::Percent(5.0),
                ..default()
            },
            BorderColor(BORDER_AND_TEXT_UI_COLOR),
            BorderRadius::all(Val::Px(15.0)),
            Name::new("tower_selected_root_node"),
            BackgroundColor(BACKGROUND_COLOR),
        ))
        .id();

    let create_text = |commands: &mut Commands,
                       text: &str,
                       font_size: f32,
                       bottom_padding: f32,
                       text_type: SelectedTowerTextTypes| {
        commands.entity(root_ui).with_children(|p| {
            p.spawn((
                Text::new(text),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(BORDER_AND_TEXT_UI_COLOR),
                text_type,
            ));
        });

        commands.entity(root_ui).with_children(|p| {
            p.spawn(Node {
                height: Val::Px(bottom_padding),
                ..default()
            });
        });
    };

    let _selected_tower_text = create_text(
        &mut commands,
        "Selected Tower to buy: Lich",
        15.0,
        20.0,
        SelectedTowerTextTypes::TowerSelected,
    );

    let _selected_tower_cost = create_text(
        &mut commands,
        "Cost: 0.0 Gold",
        15.0,
        20.0,
        SelectedTowerTextTypes::TowerCost,
    );

    let _time_to_build = create_text(
        &mut commands,
        "Time to build: 15.0 secs",
        15.0,
        20.0,
        SelectedTowerTextTypes::TimeToBuild,
    );
}

pub fn update_tower_selected_text(
    mut texts: Query<(&mut Text, &SelectedTowerTextTypes)>,
    selected_tower_type: Res<SelectedTowerType>,
    wave_control: Res<WaveControl>,
) {
    for (mut text, text_type) in &mut texts {
        match text_type {
            SelectedTowerTextTypes::TowerSelected => {
                text.0 = format!("Selected Tower to buy: {:?}", selected_tower_type.0);
            }
            SelectedTowerTextTypes::TowerCost => {
                text.0 = format!("Cost: {:.1} Gold", selected_tower_type.to_cost(1));
            }
            SelectedTowerTextTypes::TimeToBuild => {
                if !wave_control.time_between_waves.paused() {
                    text.0 = format!(
                        "Time to Build: {:.1} secs",
                        wave_control.time_between_waves.remaining_secs()
                    );
                }
            }
        }
    }
}

pub fn despawn_selected_tower_ui(
    entities: Query<(Entity, &Name), With<Node>>,
    mut commands: Commands,
) {
    for (selected_text_node_entity, name) in &entities {
        if name.as_str() == "tower_selected_root_node" {
            commands
                .entity(selected_text_node_entity)
                .despawn_recursive();
        }
    }
}
