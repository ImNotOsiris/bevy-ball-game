use bevy::app::AppExit;
use bevy::prelude::*;

use crate::AppState;
use crate::main_menu::components::*;
use crate::main_menu::styles::{ HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR };

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>)
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *bg_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *bg_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *bg_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>)
    >
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *bg_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *bg_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *bg_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
