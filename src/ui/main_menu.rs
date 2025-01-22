use crate::screen::GameState;
use crate::ui::UIAction;
use bevy::prelude::*;
use bevy::prelude::{Added, Commands, Component, Entity, KeyCode, NextState, Query, ResMut, With};
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::InputManagerBundle;

#[derive(Component, Clone, Default, Debug)]
#[require(Camera2d)]
pub struct MainMenuUI;
pub fn on_enter_main_menu_ui(
    mut commands: Commands,
    query: Query<Entity, Added<MainMenuUI>>,
    asset_server: Res<AssetServer>) {
    for menu in query.iter() {
        let input_map = InputMap::new([(UIAction::Accept, KeyCode::Space)]);
        commands.entity(menu).insert((
            InputManagerBundle::with_map(input_map),
            Text::new("press space to continue"),
            TextLayout::new_with_justify(JustifyText::Center),
        ));
    }
}

pub fn jump(query: Query<&ActionState<UIAction>, With<MainMenuUI>>, mut next_state: ResMut<NextState<GameState>>) {
    for action_state in query.iter() {
        if action_state.just_pressed(&UIAction::Accept) {
            next_state.set(GameState::InGame);
        }
    }
}