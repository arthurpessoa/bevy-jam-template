use crate::screen::states::ScreenState;
use crate::screen::title::components::TitleScreen;
use crate::settings::Action;
use bevy::prelude::*;
use i_cant_believe_its_not_bsn::WithChild;
use leafwing_input_manager::prelude::ActionState;

pub(super) fn setup(mut commands: Commands) {
    commands.spawn((
        TitleScreen,
        Camera2d,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        WithChild(
            (
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Percent(25.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(25.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                WithChild(
                    Text::new("Press <Space> to start!")
                )
            )
        )
    ));
}

pub(super) fn enter_world(query: Query<&ActionState<Action>>, mut next_screen: ResMut<NextState<ScreenState>>) {
    let action_state = query.single();
    if action_state.just_pressed(&Action::Accept) {
        next_screen.set(ScreenState::InWorld);
    }
}


pub(super) fn cleanup(mut commands: Commands, query: Query<Entity, With<TitleScreen>>) {
    for entity in query.iter() {
        println!("Despawning title screen entity: {:?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}