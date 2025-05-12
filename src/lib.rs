use bevy::app::{App, Plugin, Update};
mod core_barrier;
mod core_button;
mod core_checkbox;
mod core_radio;
mod core_radio_group;
mod core_scrollbar;
mod core_slider;
mod cursor;
mod events;
pub mod hover;
mod interaction_states;

pub use core_barrier::{CoreBarrier, CoreBarrierPlugin};
pub use core_button::{CoreButton, CoreButtonPlugin};
pub use core_checkbox::{CoreCheckbox, CoreCheckboxPlugin};
pub use core_radio::{CoreRadio, CoreRadioPlugin};
pub use core_radio_group::{CoreRadioGroup, CoreRadioGroupPlugin};
pub use core_scrollbar::{
    CoreScrollArea, CoreScrollbar, CoreScrollbarPlugin, CoreScrollbarThumb, Orientation,
};
pub use core_slider::{CoreSlider, CoreSliderPlugin, SliderDragState};
pub use cursor::CursorIconPlugin;
pub use events::{ButtonClicked, ValueChange};
pub use interaction_states::{ButtonPressed, Checked, InteractionDisabled};

pub struct CoreWidgetsPlugin;

impl Plugin for CoreWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CoreBarrierPlugin,
            CoreButtonPlugin,
            CoreCheckboxPlugin,
            CoreRadioPlugin,
            CoreRadioGroupPlugin,
            CoreScrollbarPlugin,
            CoreSliderPlugin,
            CursorIconPlugin,
        ))
        .add_systems(Update, hover::update_hover_states);
    }
}
