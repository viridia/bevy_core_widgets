//! Demonstrations of scrolling and scrollbars.
//!
//! Note that this example should not be used as a basis for a real application. A real application
//! would likely use a more sophisticated UI framework or library that includes composable styles,
//! templates, reactive signals, and other techniques that improve the developer experience. This
//! example has been written in a very brute-force, low-level style so as to demonstrate the
//! functionality of the core widgets with minimal dependencies.

use bevy::{
    a11y::AccessibilityNode,
    ecs::{
        component::HookContext, relationship::RelatedSpawner, spawn::SpawnWith, system::SystemId,
        world::DeferredWorld,
    },
    input_focus::{
        tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin},
        InputDispatchPlugin, InputFocus, InputFocusVisible,
    },
    prelude::*,
    ui,
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};
use bevy_core_widgets::{
    hover::Hovering, ButtonClicked, ButtonPressed, CoreButton, CoreScrollArea, CoreScrollbar,
    CoreScrollbarThumb, CoreSlider, CoreWidgetsPlugin, InteractionDisabled, Orientation,
    ValueChange,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CoreWidgetsPlugin,
            InputDispatchPlugin,
            TabNavigationPlugin,
        ))
        .add_systems(Startup, setup_view_root)
        .add_systems(
            Update,
            (
                update_focus_rect,
                update_scrollbar_thumb,
                close_on_esc,
                update_button_bg_colors,
            ),
        )
        .run();
}

fn setup_view_root(mut commands: Commands) {
    let camera = commands.spawn((Camera::default(), Camera2d)).id();

    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            left: ui::Val::Px(0.),
            top: ui::Val::Px(0.),
            right: ui::Val::Px(0.),
            bottom: ui::Val::Px(0.),
            padding: ui::UiRect::all(Val::Px(3.)),
            row_gap: ui::Val::Px(6.),
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        UiTargetCamera(camera),
        TabGroup::default(),
        Children::spawn((Spawn(Text::new("Scrolling")), Spawn(scroll_area_demo()))),
    ));

    // Observer for sliders that don't have an on_change handler.
    commands.add_observer(
        |mut trigger: Trigger<ValueChange<f32>>, mut q_slider: Query<&mut CoreSlider>| {
            trigger.propagate(false);
            if let Ok(mut slider) = q_slider.get_mut(trigger.target()) {
                // Update slider state from event.
                slider.set_value(trigger.event().0);
                info!("New slider state: {:?}", slider.value());
            }
        },
    );

    // Observer for buttons that don't have an on_click handler.
    commands.add_observer(
        |mut trigger: Trigger<ButtonClicked>, q_button: Query<&CoreButton>| {
            // If the button doesn't exist or is not a CoreButton
            if q_button.get(trigger.target()).is_ok() {
                trigger.propagate(false);
                let button_id = trigger.target();
                info!("Got button click event: {:?}", button_id);
            }
        },
    );
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

/// The variant determines the button's color scheme
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ButtonVariant {
    /// The default button apperance.
    #[default]
    Default,

    /// A more prominent, "call to action", appearance.
    Primary,

    /// An appearance indicating a potentially dangerous action.
    Danger,

    /// A button that is in a "toggled" state.
    Selected,
}

// Places an outline around the currently focused widget. This is a generic implementation for demo
// purposes; in a real widget library the focus rectangle would be customized to the shape of
// the individual widgets.
#[allow(clippy::type_complexity)]
fn update_focus_rect(
    mut query: Query<(Entity, Has<Outline>), With<TabIndex>>,
    focus: Res<InputFocus>,
    focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    for (control, has_focus) in query.iter_mut() {
        let needs_focus = Some(control) == focus.0 && focus_visible.0;
        if needs_focus != has_focus {
            if needs_focus {
                commands.entity(control).insert(Outline {
                    color: colors::FOCUS.into(),
                    width: ui::Val::Px(2.0),
                    offset: ui::Val::Px(1.0),
                });
            } else {
                commands.entity(control).remove::<Outline>();
            }
        }
    }
}

/// Create a scrolling area.
///
/// The "scroll area" is a container that can be scrolled. It has a nested structure which is
/// three levels deep:
/// - The outermost node is a grid that contains the scroll area and the scrollbars.
/// - The scroll area is a flex container that contains the scrollable content. This
///   is the element that has the `overflow: scroll` property.
/// - The scrollable content consists of the elements actually displayed in the scrolling area.
fn scroll_area_demo() -> impl Bundle {
    (
        // Frame element which contains the scroll area and scrollbars.
        Node {
            display: ui::Display::Grid,
            width: ui::Val::Px(200.0),
            height: ui::Val::Px(150.0),
            grid_template_columns: vec![
                ui::RepeatedGridTrack::flex(1, 1.),
                ui::RepeatedGridTrack::auto(1),
            ],
            grid_template_rows: vec![
                ui::RepeatedGridTrack::flex(1, 1.),
                ui::RepeatedGridTrack::auto(1),
            ],
            row_gap: ui::Val::Px(2.0),
            column_gap: ui::Val::Px(2.0),
            ..default()
        },
        Children::spawn((SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
            // The actual scrolling area.
            // Note that we're using `SpawnWith` here because we need to get the entity id of the
            // scroll area in order to set the target of the scrollbars.
            let scroll_area_id = parent
                .spawn((
                    Node {
                        display: ui::Display::Flex,
                        flex_direction: ui::FlexDirection::Column,
                        padding: ui::UiRect::all(ui::Val::Px(4.0)),
                        overflow: ui::Overflow::scroll(),
                        ..default()
                    },
                    BackgroundColor(colors::U3.into()),
                    ScrollPosition {
                        offset_x: 0.0,
                        offset_y: 8.0,
                    },
                    CoreScrollArea,
                    Children::spawn((
                        // The actual content of the scrolling area
                        Spawn(text_row("Alpha Wolf")),
                        Spawn(text_row("Beta Blocker")),
                        Spawn(button("Save", ButtonVariant::Default, None)),
                        Spawn(button("Create", ButtonVariant::Primary, None)),
                        Spawn(text_row("Delta Sleep")),
                        Spawn(text_row("Gamma Ray")),
                        Spawn(text_row("Epsilon Eridani")),
                        Spawn(text_row("Zeta Function")),
                        Spawn(text_row("Lambda Calculus")),
                        Spawn(text_row("Nu Metal")),
                        Spawn(text_row("Pi Day")),
                        Spawn(text_row("Chi Pants")),
                        // Spawn(text_row("Psi Powers")),
                        // Spawn(text_row("Omega Fatty Acid")),
                    )),
                ))
                .id();

            // Vertical scrollbar
            parent.spawn((
                Node {
                    min_width: ui::Val::Px(8.0),
                    grid_row: GridPlacement::start(1),
                    grid_column: GridPlacement::start(2),
                    ..default()
                },
                Hovering(false),
                CoreScrollbar {
                    orientation: Orientation::Vertical,
                    target: scroll_area_id,
                    min_thumb_size: 8.0,
                },
                Children::spawn(Spawn((
                    Node {
                        position_type: ui::PositionType::Absolute,
                        ..default()
                    },
                    Hovering(false),
                    BackgroundColor(colors::U4.into()),
                    BorderRadius::all(ui::Val::Px(4.0)),
                    CoreScrollbarThumb,
                ))),
            ));

            // Horizontal scrollbar
            parent.spawn((
                Node {
                    min_height: ui::Val::Px(8.0),
                    grid_row: GridPlacement::start(2),
                    grid_column: GridPlacement::start(1),
                    ..default()
                },
                Hovering(false),
                CoreScrollbar {
                    orientation: Orientation::Horizontal,
                    target: scroll_area_id,
                    min_thumb_size: 8.0,
                },
                Children::spawn(Spawn((
                    Node {
                        position_type: ui::PositionType::Absolute,
                        ..default()
                    },
                    Hovering(false),
                    BackgroundColor(colors::U4.into()),
                    BorderRadius::all(ui::Val::Px(4.0)),
                    CoreScrollbarThumb,
                ))),
            ));
        }),)),
    )
}

/// Create a list row
fn text_row(caption: &str) -> impl Bundle {
    (
        Text::new(caption),
        TextFont {
            font_size: 14.0,
            ..default()
        },
    )
}

// Update the button's background color.
#[allow(clippy::type_complexity)]
fn update_scrollbar_thumb(
    mut q_thumb: Query<(&CoreScrollbarThumb, &mut BackgroundColor, &Hovering), Changed<Hovering>>,
) {
    for (_thumb, mut thumb_bg, Hovering(is_hovering)) in q_thumb.iter_mut() {
        let color: Color = if *is_hovering {
            // If hovering, use a lighter color
            colors::U5
        } else {
            // Default color for the slider
            colors::U4
        }
        .into();

        if thumb_bg.0 != color {
            // Update the color of the thumb
            thumb_bg.0 = color;
        }
    }
}

#[derive(Component, Default)]
#[component(immutable, on_add = on_set_label, on_replace = on_set_label)]
struct AccessibleName(String);

// Hook to set the a11y "checked" state when the checkbox is added.
fn on_set_label(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    let name = entt.get::<AccessibleName>().unwrap().0.clone();
    if let Some(mut accessibility) = entt.get_mut::<AccessibilityNode>() {
        accessibility.set_label(name.as_str());
    }
}

mod colors {
    use bevy::color::Srgba;

    pub const U3: Srgba = Srgba::new(0.224, 0.224, 0.243, 1.0);
    pub const U4: Srgba = Srgba::new(0.486, 0.486, 0.529, 1.0);
    pub const U5: Srgba = Srgba::new(1.0, 1.0, 1.0, 1.0);
    pub const PRIMARY: Srgba = Srgba::new(0.341, 0.435, 0.525, 1.0);
    pub const DESTRUCTIVE: Srgba = Srgba::new(0.525, 0.341, 0.404, 1.0);
    pub const FOCUS: Srgba = Srgba::new(0.055, 0.647, 0.914, 0.15);
}

#[derive(Component, Default)]
struct DemoButton {
    variant: ButtonVariant,
}

fn button(caption: &str, variant: ButtonVariant, on_click: Option<SystemId>) -> impl Bundle {
    (
        Node {
            display: ui::Display::Flex,
            flex_direction: ui::FlexDirection::Row,
            justify_content: ui::JustifyContent::Center,
            align_items: ui::AlignItems::Center,
            align_content: ui::AlignContent::Center,
            padding: ui::UiRect::axes(ui::Val::Px(12.0), ui::Val::Px(0.0)),
            border: ui::UiRect::all(ui::Val::Px(0.0)),
            min_height: ui::Val::Px(24.0),
            ..default()
        },
        BorderRadius::all(ui::Val::Px(4.0)),
        Name::new("Button"),
        Hovering::default(),
        CursorIcon::System(SystemCursorIcon::Pointer),
        DemoButton { variant },
        CoreButton { on_click },
        AccessibleName(caption.to_string()),
        TabIndex(0),
        children![(
            Text::new(caption),
            TextFont {
                font_size: 14.0,
                ..default()
            }
        )],
    )
}

// Update the button's background color.
#[allow(clippy::type_complexity)]
fn update_button_bg_colors(
    mut query: Query<
        (
            &DemoButton,
            &mut BackgroundColor,
            &Hovering,
            &ButtonPressed,
            Has<InteractionDisabled>,
        ),
        Or<(Added<DemoButton>, Changed<Hovering>, Changed<ButtonPressed>)>,
    >,
) {
    for (button, mut bg_color, Hovering(is_hovering), ButtonPressed(is_pressed), is_disabled) in
        query.iter_mut()
    {
        // Update the background color based on the button's state
        let base_color = match button.variant {
            ButtonVariant::Default => colors::U3,
            ButtonVariant::Primary => colors::PRIMARY,
            ButtonVariant::Danger => colors::DESTRUCTIVE,
            ButtonVariant::Selected => colors::U4,
        };

        let new_color = match (is_disabled, is_pressed, is_hovering) {
            (true, _, _) => base_color.with_alpha(0.2),
            (_, true, true) => base_color.lighter(0.07),
            (_, false, true) => base_color.lighter(0.03),
            _ => base_color,
        };

        bg_color.0 = new_color.into();
    }
}
