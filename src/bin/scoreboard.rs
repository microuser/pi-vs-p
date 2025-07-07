//! TinyText 3D Scoreboard Demo
//! Run with: cargo run --bin scoreboard

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::math::primitives::{Cuboid, Plane3d};
use std::f32::consts::PI;
use pi_vs_pi::tiny_text::TinyFont;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TinyText ASCII Demo".into(),
                resolution: WindowResolution::new(1200.0, 800.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(CharacterCycler::new())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (update_character_display, cleanup_old_text))
        .run();
}

#[derive(Resource)]
struct CharacterCycler {
    timer: Timer,
    current_char: u8,
    text_entities: Vec<Entity>,
}

impl CharacterCycler {
    fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            current_char: 32, // Start with space character
            text_entities: Vec::new(),
        }
    }
    fn get_current_char(&self) -> char {
        self.current_char as char
    }
    fn advance(&mut self) {
        self.current_char += 1;
        if self.current_char > 126 {
            self.current_char = 32;
        }
    }
    fn get_display_string(&self) -> String {
        let current = self.get_current_char();
        let ascii_code = self.current_char;
        format!("Char: '{}' ASCII: {} ({}/95)",
            if current.is_whitespace() { '·' } else { current },
            ascii_code,
            ascii_code - 31)
    }
}

#[derive(Component)]
struct TextCube;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 8.0, 20.0)
            .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 15.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            color: Color::rgb(1.0, 0.9, 0.8),
            range: 50.0,
            ..default()
        },
        transform: Transform::from_xyz(-10.0, 10.0, 5.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            color: Color::rgb(0.8, 0.9, 1.0),
            range: 30.0,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 5.0, 15.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::default())),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.2, 0.3),
            metallic: 0.1,
            perceptual_roughness: 0.9,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, -2.0, 0.0),
        ..default()
    });
    for i in 0..10 {
        let angle = (i as f32 / 10.0) * 2.0 * PI;
        let radius = 25.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let height = (i as f32 * 0.7).sin() * 3.0 + 5.0;
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid { half_size: Vec3::splat(0.75) })),
            material: materials.add(StandardMaterial {
                base_color: Color::hsl(i as f32 * 36.0, 0.7, 0.5),
                metallic: 0.3,
                perceptual_roughness: 0.4,
                ..default()
            }),
            transform: Transform::from_xyz(x, height, z)
                .with_rotation(Quat::from_rotation_y(angle + PI)),
            ..default()
        });
    }
}

fn update_character_display(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cycler: ResMut<CharacterCycler>,
    time: Res<Time>,
) {
    cycler.timer.tick(time.delta());
    if cycler.timer.just_finished() {
        for entity in cycler.text_entities.drain(..) {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
        }
        let current_char = cycler.get_current_char();
        let display_info = cycler.get_display_string();
        spawn_text_with_tracking(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut cycler.text_entities,
            &current_char.to_string(),
            Vec3::new(-2.0, 5.0, 0.0),
            Vec3::new(1.0, 1.0, 1.0),
            Quat::IDENTITY,
            Vec3::new(4.0, 0.0, 0.0),
            Color::rgb(1.0, 0.6, 0.2),
        );
        spawn_text_with_tracking(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut cycler.text_entities,
            &display_info,
            Vec3::new(-8.0, 1.0, 0.0),
            Vec3::new(0.3, 0.3, 0.3),
            Quat::IDENTITY,
            Vec3::new(1.2, 0.0, 0.0),
            Color::rgb(0.4, 0.8, 1.0),
        );
        let preview_text = (1..=5)
            .map(|offset| {
                let next_ascii = if cycler.current_char + offset > 126 {
                    32 + (cycler.current_char + offset - 127)
                } else {
                    cycler.current_char + offset
                };
                let next_char = next_ascii as char;
                if next_char.is_whitespace() { '·' } else { next_char }
            })
            .collect::<String>();
        spawn_text_with_tracking(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut cycler.text_entities,
            &format!("Next: {}", preview_text),
            Vec3::new(-6.0, -1.0, 0.0),
            Vec3::new(0.25, 0.25, 0.25),
            Quat::IDENTITY,
            Vec3::new(1.0, 0.0, 0.0),
            Color::rgb(0.6, 0.6, 0.6),
        );
        let rotation = Quat::from_rotation_y(time.elapsed_seconds() * 0.5);
        spawn_text_with_tracking(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut cycler.text_entities,
            &current_char.to_string(),
            Vec3::new(8.0, 3.0, 0.0),
            Vec3::new(0.7, 0.7, 0.7),
            rotation,
            Vec3::new(3.0, 0.0, 0.0),
            Color::rgb(0.8, 0.2, 0.8),
        );
        let progress = (cycler.current_char - 32) as f32 / 94.0;
        let progress_text = format!("Progress: {:.0}%", progress * 100.0);
        spawn_text_with_tracking(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut cycler.text_entities,
            &progress_text,
            Vec3::new(-8.0, -3.0, 0.0),
            Vec3::new(0.3, 0.3, 0.3),
            Quat::IDENTITY,
            Vec3::new(1.2, 0.0, 0.0),
            Color::rgb(0.2, 1.0, 0.4),
        );
        cycler.advance();
    }
}

fn spawn_text_with_tracking(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    entity_tracker: &mut Vec<Entity>,
    text: &str,
    origin: Vec3,
    scale: Vec3,
    orientation: Quat,
    offset: Vec3,
    color: Color,
) {
    let font = TinyFont::new();
    let mut cursor = origin;
    let cube_mesh = meshes.add(Mesh::from(Cuboid { half_size: Vec3::splat(0.5) }));
    let cube_material = materials.add(StandardMaterial {
        base_color: color,
        metallic: 0.2,
        perceptual_roughness: 0.3,
        ..default()
    });
    for c in text.chars() {
        if let Some(glyph) = font.get_glyph(c) {
            for cube_pos in glyph {
                let local_pos = Vec3::new(
                    cube_pos.x as f32 * scale.x,
                    cube_pos.y as f32 * scale.y,
                    0.0
                );
                let world_pos = cursor + orientation * local_pos;
                let entity = commands.spawn((
                    PbrBundle {
                        mesh: cube_mesh.clone(),
                        material: cube_material.clone(),
                        transform: Transform {
                            translation: world_pos,
                            rotation: orientation,
                            scale: scale * 0.8,
                        },
                        ..default()
                    },
                    TextCube,
                )).id();
                entity_tracker.push(entity);
            }
        }
        cursor += orientation * offset;
    }
}

fn cleanup_old_text(
    mut commands: Commands,
    query: Query<Entity, With<TextCube>>,
    cycler: Res<CharacterCycler>,
) {
    if cycler.timer.just_finished() {
        for entity in query.iter() {
            if !cycler.text_entities.contains(&entity) {
                if let Some(mut entity_commands) = commands.get_entity(entity) {
                    entity_commands.despawn();
                }
            }
        }
    }
}

