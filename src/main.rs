use bevy::{prelude::*, input::{keyboard::KeyboardInput, ButtonState}};
use rand::Rng;

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Resource)]
struct SymbolQueue(Vec<char>);

fn main() {
    App::new()
        .insert_resource(SpawnTimer(Timer::from_seconds(0.4, TimerMode::Repeating)))
        .insert_resource(SymbolQueue(Vec::new()))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (text_update_system, text_display, print_keyboard_event_system))
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
}

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText(KeyCode);

#[derive(Component)]
struct TextTimer(Timer);

struct Position(f32, f32);

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

fn text_update_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut squeue: ResMut<SymbolQueue>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let random_char = rng.gen_range(97..=122) as u8 as char;
        let range = 5.0..=95.0;
        let position = Position(rng.gen_range(range.clone()), rng.gen_range(range));
        squeue.0.push(random_char);
        commands.spawn((
            get_text_bundle(random_char, position, &asset_server),
            ColorText(get_char_keycode(random_char)),
            TextTimer(Timer::from_seconds(1.2, TimerMode::Repeating))
        ));
    }
}

fn print_keyboard_event_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut query: Query<(Entity, &ColorText), With<Text>>,
    mut commands: Commands,
) {
    if let Some((entity, text)) = query.iter_mut().next() {
        if let Some(KeyboardInput {
            key_code: Some(key),
            state: ButtonState::Pressed,
            window: _,
            scan_code: _,
        }) = keyboard_input_events.iter().next() {
            if text.0 == *key {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn text_display(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut TextTimer, Entity), With<Text>>,
) {
    for (mut timer, entity) in query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn get_text_bundle(text: char, position: Position, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            text,
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(position.0),
            right: Val::Percent(position.1),
            ..default()
        })
}

fn get_char_keycode(c: char) -> KeyCode {
    match c {
        'a' => KeyCode::A,
        'b' => KeyCode::B,
        'c' => KeyCode::C,
        'd' => KeyCode::D,
        'e' => KeyCode::E,
        'f' => KeyCode::F,
        'g' => KeyCode::G,
        'h' => KeyCode::H,
        'i' => KeyCode::I,
        'j' => KeyCode::J,
        'k' => KeyCode::K,
        'l' => KeyCode::L,
        'm' => KeyCode::M,
        'n' => KeyCode::N,
        'o' => KeyCode::O,
        'p' => KeyCode::P,
        'q' => KeyCode::Q,
        'r' => KeyCode::R,
        's' => KeyCode::S,
        't' => KeyCode::T,
        'u' => KeyCode::U,
        'v' => KeyCode::V,
        'w' => KeyCode::W,
        'x' => KeyCode::X,
        'y' => KeyCode::Y,
        'z' => KeyCode::Z,
        _ => KeyCode::Yen,
    }
}