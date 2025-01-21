use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_enter_main_menu_ui);
    }
}

#[derive(Component, Clone, Default, Debug)]
#[require(Camera2d)]
pub struct MainMenuUI;

fn on_enter_main_menu_ui(query: Query<(&mut MainMenuUI), Added<MainMenuUI>>) {
    for menu in query.iter() {
        println!("Entered main menu UI");
    }
}