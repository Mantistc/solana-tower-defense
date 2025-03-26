use bevy::prelude::*;

use crate::tower_building::SelectedTowerType;

use super::*;

#[derive(Component, PartialEq, Eq)]
pub enum SelectedTowerTextTypes {
    TowerSelected,
    TowerCost,
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
                left: Val::Percent(38.5),
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

    let _selected_tower_text = commands.entity(root_ui).with_children(|p| {
        p.spawn((
            Text::new("Selected Tower to buy: Lich"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(BORDER_AND_TEXT_UI_COLOR),
            SelectedTowerTextTypes::TowerSelected,
        ));
    });

    let _padding = commands.entity(root_ui).with_children(|p| {
        p.spawn(Node {
            height: Val::Px(20.0),
            ..default()
        });
    });

    let _selected_tower_cost = commands.entity(root_ui).with_children(|p| {
        p.spawn((
            Text::new("Cost: 0.0 Gold"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(BORDER_AND_TEXT_UI_COLOR),
            SelectedTowerTextTypes::TowerCost,
        ));
    });
}

pub fn update_tower_selected_text(
    mut texts: Query<(&mut Text, &SelectedTowerTextTypes)>,
    selected_tower_type: Res<SelectedTowerType>,
) {
    for (mut text, text_type) in &mut texts {
        match text_type {
            SelectedTowerTextTypes::TowerSelected => {
                text.0 = format!("Selected Tower to buy: {:?}", selected_tower_type.0);
            }
            SelectedTowerTextTypes::TowerCost => {
                text.0 = format!("Cost: {:.1} Gold", selected_tower_type.to_cost(1));
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
