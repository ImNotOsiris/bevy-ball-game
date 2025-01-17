use bevy::prelude::*;

use crate::game::SimulationState;

pub fn pause_simulation(mut sim_state_next_state: ResMut<NextState<SimulationState>>) {
    sim_state_next_state.set(SimulationState::Paused)
}

pub fn resume_simulation(mut sim_state_next_state: ResMut<NextState<SimulationState>>) {
    sim_state_next_state.set(SimulationState::Running)
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.0 == SimulationState::Running {
            // commands.insert_resource(NextState(Some(SimulationState::Paused)));
            next_sim_state.set(SimulationState::Paused);
            println!("Simulation Paused.");
        }

        if simulation_state.0 == SimulationState::Paused {
            next_sim_state.set(SimulationState::Running);
            println!("Simulation Running.")
        }
    }
}
