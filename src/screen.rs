use crate::terrain::*;
use crate::ui::*;
use bevy::prelude::*;
use std::fmt::Debug;
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(Component, Clone, Default, Debug)]
#[require(MainMenuUI)]
pub struct MainMenu;


#[derive(Component, Clone, Default, Debug)]
#[require(
    Terrain,
    InGameUI
)]
pub struct InGame;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_plugins(ScreenPlugin { state: GameState::MainMenu, root: MainMenu })
            .add_plugins(ScreenPlugin { state: GameState::InGame, root: InGame })
        ;
    }
}
pub struct ScreenPlugin<C: Component + Clone + Default> {
    state: GameState,
    root: C,
}
impl<C: Component + Clone + Default + Debug> Plugin for ScreenPlugin<C> {
    fn build(&self, app: &mut App) {
        println!("Adding screen plugin for state: {:?} with root_component: {:?}", self.state, self.root);
        app
            .add_systems(OnEnter(self.state.clone()), |mut commands: Commands| {
                commands.spawn(C::default());
            })
            .add_systems(OnExit(self.state.clone()), |mut commands: Commands, query: Query<Entity, With<C>>| {
                for entity in query.iter() {
                    commands.entity(entity).despawn_recursive()
                }
            });
    }
}