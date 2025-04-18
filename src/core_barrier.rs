use bevy::{
    ecs::system::SystemId,
    input::{keyboard::KeyboardInput, ButtonState},
    input_focus::{FocusedInput, InputFocus, InputFocusVisible},
    prelude::*,
};

/// A "barrier" is a backdrop element, one that covers the entire screen, blocks click events
/// from reaching elements behind it, and can be used to close a dialog or menu.
///
/// The `on_close` field is a system that will be run when the barrier gets a mouse down event.
#[derive(Component, Debug)]
pub struct CoreBarrier {
    pub on_close: Option<SystemId>,
}

pub(crate) fn barrier_on_key_input(
    mut trigger: Trigger<FocusedInput<KeyboardInput>>,
    q_state: Query<&CoreBarrier>,
    mut commands: Commands,
) {
    if let Ok(bstate) = q_state.get(trigger.target()) {
        let event = &trigger.event().input;
        if event.state == ButtonState::Pressed
            && !event.repeat
            && (event.key_code == KeyCode::Escape)
        {
            if let Some(on_close) = bstate.on_close {
                trigger.propagate(false);
                commands.run_system(on_close);
            }
        }
    }
}

pub(crate) fn barrier_on_pointer_down(
    mut trigger: Trigger<Pointer<Pressed>>,
    q_state: Query<&CoreBarrier>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    let entity_id = trigger.target();
    if let Ok(bstate) = q_state.get(entity_id) {
        focus.0 = Some(entity_id);
        focus_visible.0 = false;
        trigger.propagate(false);
        if let Some(on_close) = bstate.on_close {
            commands.run_system(on_close);
        }
    }
}

pub struct CoreBarrierPlugin;

impl Plugin for CoreBarrierPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(barrier_on_key_input)
            .add_observer(barrier_on_pointer_down);
    }
}
