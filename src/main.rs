mod parser;

use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{
    core_pipeline::{
        bloom::{Bloom, BloomCompositeMode},
        tonemapping::Tonemapping,
    },
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, set_additive_bloom).chain())
        .add_systems(Update, (wiggle_text, close_on_esc, fps_text_update_system, draw_board))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .run();
}

#[derive(Component)]
struct Wiggle;

fn setup(mut commands: Commands) {
    let text_font = TextFont {
        font_size: 100.0,
        ..default()
    };

    // 2d camera
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true, // 1. HDR is required for bloom
            ..default()
        },
        Tonemapping::TonyMcMapface, // 2. Using a tone mapper that desaturates to white is recommended
        Bloom::default(),           // 3. Enable bloom for the camera
    ));

    let color = Color::srgb(4.0, 1.0, 2.4);

    commands.spawn((
        Text2d::new("Weeeeee"),
        text_font.clone(),
        TextColor(color),
        Transform::from_xyz(0.0, 500.0, 1.0),
        Wiggle,
    ));

    // commands.spawn((Text2d::new("0?"), text_font, TextColor(color), FpsText));
}

fn draw_board(mut commands: Commands, mut gizmos: Gizmos,) {
    let radius = 20.0;
    for i in 0..10 {
        for j in 0..10 {
            gizmos.circle_2d(Isometry2d::from_xy(i as f32 * (radius * 2.4) - radius * 10.0, j as f32 * (radius * 2.4) - radius * 10.0), radius, Color::srgb(4.0, 1.0, 1.5));
        }
    }
}

fn wiggle_text(time: Res<Time>, mut query: Query<&mut Transform, (With<Text2d>, With<Wiggle>)>) {
    let time_factor = 2.0;
    let movement_factor = 5.0;
    let prev_t = time.elapsed_secs() - time.delta_secs();
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(
            ops::cos(time.elapsed_secs() * time_factor) * movement_factor / 150.0,
        );
        let (delta_x, delta_y) = wiggle_delta(prev_t, time.elapsed_secs(), time_factor);
        transform.translation.x += movement_factor * delta_x;
        transform.translation.y += movement_factor * delta_y;
    }
}

fn wiggle_delta(t_start: f32, t_end: f32, time_factor: f32) -> (f32, f32) {
    (
        ops::sin(t_start * time_factor * 2.1) - ops::sin(t_end * time_factor * 2.1),
        ops::cos(t_start * time_factor * 2.1) - ops::cos(t_end * time_factor * 2.1),
    )
}

fn set_additive_bloom(camera: Single<(Entity, Option<&mut Bloom>), With<Camera>>) {
    let bloom = camera.into_inner();
    match bloom {
        (_entity, Some(mut bloom)) => {
            bloom.composite_mode = BloomCompositeMode::EnergyConserving;
            bloom.low_frequency_boost = 0.8;
        }
        (_entity, None) => {}
    }
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

#[derive(Component)]
struct FpsText;

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<(&mut Text2d, &mut Transform), With<FpsText>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    for (mut text, mut transform) in &mut query {
        let (camera, camera_transform) = *camera_query;

        let Ok(point) = camera.viewport_to_world_2d(camera_transform, (20.0, 40.0).into()) else {
            println!("no point :(");
            return;
        };

        transform.translation.x = point.x;
        transform.translation.y = point.y;
        
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            text.0 = format!("{value:>4.0}");
        } else {
            // display "N/A" if we can't get an FPS measurement
            // add an extra space to preserve alignment
            text.0 = " N/A".into();
        }
    }
}
