use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput; // For explicit ButtonInput usage
// Optionally, for Input alias:
// use bevy::input::ButtonInput as Input;
use bevy::window::WindowResolution;
use bevy::render::render_asset::RenderAssetUsages;
use std::f32::consts::PI;
use serde::{Deserialize, Serialize};

// Data structures
#[derive(Serialize, Deserialize, Clone, Debug)]
struct CompetitionData {
    name: String,
    kobold: f32,
    troglodyte: f32,
}

#[derive(Resource)]
struct GameData {
    data: Vec<CompetitionData>,
    colors: Vec<Color>,
    selected_category: usize,
    left_angle: f32,
    right_angle: f32,
    target_right_angle: f32,
}

// Component markers
#[derive(Component)]
struct LeftChart;

#[derive(Component)]
struct RightChart;

#[derive(Component)]
struct PieSlice {
    category_index: usize,
    start_angle: f32,
    end_angle: f32,
    is_left: bool,
}

#[derive(Component)]
struct ScoreBoard;

#[derive(Component)]
struct CategorySelector;

// Input state
#[derive(Resource, Default)]
struct InputState {
    dragging: bool,
    drag_start_y: f32,
    drag_start_angle: f32,
}

pub fn run_pi_vs_pi_app() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pi vs Pi - Rust Edition".into(),
                resolution: WindowResolution::new(1200.0, 800.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(create_game_data())
        .insert_resource(InputState::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            update_charts,
            update_ui,
            handle_mouse_drag,
        ))
        .run();
}

fn create_game_data() -> GameData {
    let data = vec![
        CompetitionData { name: "Strength".to_string(), kobold: 4.0, troglodyte: 7.0 },
        CompetitionData { name: "Cunning".to_string(), kobold: 8.0, troglodyte: 5.0 },
        CompetitionData { name: "Aggression".to_string(), kobold: 6.0, troglodyte: 8.0 },
        CompetitionData { name: "Sociality".to_string(), kobold: 7.0, troglodyte: 3.0 },
        CompetitionData { name: "Habitat".to_string(), kobold: 6.0, troglodyte: 9.0 },
        CompetitionData { name: "Intelligence".to_string(), kobold: 7.0, troglodyte: 4.0 },
        CompetitionData { name: "Stealth".to_string(), kobold: 8.0, troglodyte: 6.0 },
    ];

    let colors = vec![
        Color::rgb(1.0, 0.27, 0.27),  // Red
        Color::rgb(0.27, 1.0, 0.27),  // Green
        Color::rgb(0.27, 0.27, 1.0),  // Blue
        Color::rgb(1.0, 1.0, 0.27),   // Yellow
        Color::rgb(1.0, 0.27, 1.0),   // Magenta
        Color::rgb(0.27, 1.0, 1.0),   // Cyan
        Color::rgb(1.0, 0.53, 0.27),  // Orange
        Color::rgb(0.53, 1.0, 0.27),  // Light Green
    ];

    GameData {
        data,
        colors,
        selected_category: 0,
        left_angle: -PI / 2.0,
        right_angle: 0.0,
        target_right_angle: 0.0,
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_data: Res<GameData>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 25.0)
            .looking_at(Vec3::new(0.0, 4.0, 0.0), Vec3::Y),
        ..default()
    });

    // Lighting
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 20.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 15.0),
        ..default()
    });

    // Create initial charts
    create_pie_charts(&mut commands, &mut meshes, &mut materials, &game_data);
    create_scoreboards(&mut commands, &game_data);
}

fn create_pie_charts(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    game_data: &GameData,
) {
    // Left chart (Kobolds)
    let left_values: Vec<f32> = game_data.data.iter().map(|d| d.kobold).collect();
    create_chart(commands, meshes, materials, &left_values, &game_data.colors, 
                Vec3::new(-6.25, 7.2, 0.0), true, &game_data.data);

    // Right chart (Troglodytes)
    let right_values: Vec<f32> = game_data.data.iter().map(|d| d.troglodyte).collect();
    create_chart(commands, meshes, materials, &right_values, &game_data.colors, 
                Vec3::new(6.25, 7.2, 0.0), false, &game_data.data);
}

fn create_chart(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    values: &[f32],
    colors: &[Color],
    position: Vec3,
    is_left: bool,
    data: &[CompetitionData],
) {
    let total: f32 = values.iter().sum();
    let mut angle = 0.0;

    for (i, &value) in values.iter().enumerate() {
        let slice_angle = (value / total) * 2.0 * PI;
        let color = colors[i % colors.len()];
        
        // Calculate variable radius based on competitive proportion
        let other_value = if is_left { data[i].troglodyte } else { data[i].kobold };
        let category_total = value + other_value;
        let base_radius = 9.0;
        let slice_radius = base_radius * (value / category_total);

        // Create slice mesh
        let slice_mesh = create_pie_slice_mesh(angle, angle + slice_angle, slice_radius, 1.5);
        let material = materials.add(StandardMaterial {
            base_color: color,
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        let mut entity = commands.spawn(PbrBundle {
            mesh: meshes.add(slice_mesh),
            material,
            transform: Transform::from_translation(position),
            ..default()
        });

        entity.insert(PieSlice {
            category_index: i,
            start_angle: angle,
            end_angle: angle + slice_angle,
            is_left,
        });

        if is_left {
            entity.insert(LeftChart);
        } else {
            entity.insert(RightChart);
        }

        angle += slice_angle;
    }
}

fn create_pie_slice_mesh(start_angle: f32, end_angle: f32, radius: f32, height: f32) -> Mesh {
    let segments = 32;
    let angle_step = (end_angle - start_angle) / segments as f32;
    
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

    // Center points for top and bottom
    positions.push([0.0, height / 2.0, 0.0]); // Top center
    positions.push([0.0, -height / 2.0, 0.0]); // Bottom center
    normals.push([0.0, 1.0, 0.0]);
    normals.push([0.0, -1.0, 0.0]);

    // Edge vertices
    for i in 0..=segments {
        let angle = start_angle + i as f32 * angle_step;
        let x = radius * angle.cos();
        let z = radius * angle.sin();
        
        // Top edge
        positions.push([x, height / 2.0, z]);
        normals.push([0.0, 1.0, 0.0]);
        
        // Bottom edge
        positions.push([x, -height / 2.0, z]);
        normals.push([0.0, -1.0, 0.0]);
    }

    // Top face triangles
    for i in 0..segments {
        let v1 = 2 + i * 2;
        let v2 = 2 + (i + 1) * 2;
        indices.extend_from_slice(&[0, v1 as u32, v2 as u32]);
    }

    // Bottom face triangles
    for i in 0..segments {
        let v1 = 3 + i * 2;
        let v2 = 3 + (i + 1) * 2;
        indices.extend_from_slice(&[1, v2 as u32, v1 as u32]);
    }

    // Side faces
    for i in 0..segments {
        let top1 = 2 + i * 2;
        let bottom1 = 3 + i * 2;
        let top2 = 2 + (i + 1) * 2;
        let bottom2 = 3 + (i + 1) * 2;
        
        // Two triangles per segment
        indices.extend_from_slice(&[
            top1 as u32, bottom1 as u32, top2 as u32,
            bottom1 as u32, bottom2 as u32, top2 as u32,
        ]);
    }

    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    mesh
}

fn create_scoreboards(commands: &mut Commands, game_data: &GameData) {
    let selected = &game_data.data[game_data.selected_category];
    
    // Left scoreboard (Kobolds)
    create_scoreboard_text(commands, "KOBOLDS", selected.kobold as i32, 
                          Vec3::new(-6.25, 16.0, 0.0), Color::rgb(1.0, 0.42, 0.42));
    
    // Center scoreboard (Category name and difference)
    let diff = (selected.kobold - selected.troglodyte).abs() as i32;
    create_scoreboard_text(commands, &selected.name, diff, 
                          Vec3::new(0.0, 16.0, 0.0), Color::rgb(1.0, 1.0, 0.33));
    
    // Right scoreboard (Troglodytes)
    create_scoreboard_text(commands, "TROGLODYTES", selected.troglodyte as i32, 
                          Vec3::new(6.25, 16.0, 0.0), Color::rgb(0.31, 0.80, 0.77));
}

fn create_scoreboard_text(
    commands: &mut Commands,
    title: &str,
    value: i32,
    position: Vec3,
    color: Color,
) {
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection::new(
                        format!("{}\n{}", title, value),
                        TextStyle {
                            font_size: 32.0,
                            color,
                            ..default()
                        },
                    ),
                ],
                justify: bevy::text::JustifyText::Center,
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        ScoreBoard,
    ));
}

fn handle_input(
    mut game_data: ResMut<GameData>,
    mut wheel_events: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Mouse wheel input
    for event in wheel_events.read() {
        game_data.left_angle += event.y * 0.01;
        update_selection(&mut game_data);
    }

    // Keyboard input for category selection
    if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::ArrowUp) {
        game_data.selected_category = if game_data.selected_category == 0 {
            game_data.data.len() - 1
        } else {
            game_data.selected_category - 1
        };
        let selected = game_data.selected_category;
        select_category(&mut game_data, selected);
    }
    
    if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::ArrowDown) {
        game_data.selected_category = (game_data.selected_category + 1) % game_data.data.len();
        let selected = game_data.selected_category;
        select_category(&mut game_data, selected);
    }
}

fn handle_mouse_drag(
    mut input_state: ResMut<InputState>,
    mut game_data: ResMut<GameData>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    
    if mouse_button.just_pressed(MouseButton::Left) {
        input_state.dragging = true;
        input_state.drag_start_angle = game_data.left_angle;
        if let Some(cursor_pos) = window.cursor_position() {
            input_state.drag_start_y = cursor_pos.y;
        }
    }
    
    if mouse_button.just_released(MouseButton::Left) {
        input_state.dragging = false;
    }
    
    if input_state.dragging {
        for motion in mouse_motion.read() {
            let screen_height = window.height();
            let rotation_amount = (motion.delta.y / screen_height) * 2.0 * PI;
            game_data.left_angle += rotation_amount;
            update_selection(&mut game_data);
        }
    }
}

fn update_selection(game_data: &mut GameData) {
    // Find which category is currently selected based on angle
    let total: f32 = game_data.data.iter().map(|d| d.kobold).sum();
    let mut angle = 0.0;
    let normalized_angle = ((-game_data.left_angle) % (2.0 * PI) + 2.0 * PI) % (2.0 * PI);
    
    for (i, data_item) in game_data.data.iter().enumerate() {
        let slice_angle = (data_item.kobold / total) * 2.0 * PI;
        if normalized_angle >= angle && normalized_angle < angle + slice_angle {
            game_data.selected_category = i;
            break;
        }
        angle += slice_angle;
    }
    
    // Calculate target angle for right chart alignment
    find_target_angle(game_data);
}

fn select_category(game_data: &mut GameData, category_index: usize) {
    let total: f32 = game_data.data.iter().map(|d| d.kobold).sum();
    let mut angle = 0.0;
    
    for (i, data_item) in game_data.data.iter().enumerate() {
        let slice_angle = (data_item.kobold / total) * 2.0 * PI;
        if i == category_index {
            let mid_angle = angle + slice_angle / 2.0;
            game_data.left_angle = -mid_angle;
            break;
        }
        angle += slice_angle;
    }
    
    update_selection(game_data);
}

fn find_target_angle(game_data: &mut GameData) {
    let total: f32 = game_data.data.iter().map(|d| d.troglodyte).sum();
    let mut angle = 0.0;
    
    // Right chart is in reverse order
    let right_values: Vec<f32> = game_data.data.iter().rev().map(|d| d.troglodyte).collect();
    let target_index = game_data.data.len() - 1 - game_data.selected_category;
    
    for (i, &value) in right_values.iter().enumerate() {
        let slice_angle = (value / total) * 2.0 * PI;
        if i == target_index {
            let mid_angle = angle + slice_angle / 2.0;
            game_data.target_right_angle = PI - mid_angle;
            break;
        }
        angle += slice_angle;
    }
}

fn update_charts(
    mut game_data: ResMut<GameData>,
    mut left_query: Query<&mut Transform, (With<LeftChart>, Without<RightChart>)>,
    mut right_query: Query<&mut Transform, (With<RightChart>, Without<LeftChart>)>,
) {
    // Update left chart rotation
    for mut transform in left_query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(game_data.left_angle);
    }
    
    // Smoothly rotate right chart to target angle
    let angle_diff = game_data.target_right_angle - game_data.right_angle;
    let normalized_diff = ((angle_diff + PI) % (2.0 * PI)) - PI;
    game_data.right_angle += normalized_diff * 0.1;
    
    for mut transform in right_query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(game_data.right_angle);
    }
}

fn update_ui(
    mut commands: Commands,
    game_data: Res<GameData>,
    scoreboard_query: Query<Entity, With<ScoreBoard>>,
) {
    if !game_data.is_changed() {
        return;
    }
    
    // Remove old scoreboards
    for entity in scoreboard_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Create new scoreboards
    let selected = &game_data.data[game_data.selected_category];
    
    // Left scoreboard (Kobolds)
    create_scoreboard_text(&mut commands, "KOBOLDS", selected.kobold as i32, 
                          Vec3::new(-6.25, 16.0, 0.0), Color::rgb(1.0, 0.42, 0.42));
    
    // Center scoreboard (Category name and difference)
    let diff = (selected.kobold - selected.troglodyte).abs() as i32;
    create_scoreboard_text(&mut commands, &selected.name, diff, 
                          Vec3::new(0.0, 16.0, 0.0), Color::rgb(1.0, 1.0, 0.33));
    
    // Right scoreboard (Troglodytes)
    create_scoreboard_text(&mut commands, "TROGLODYTES", selected.troglodyte as i32, 
                          Vec3::new(6.25, 16.0, 0.0), Color::rgb(0.31, 0.80, 0.77));
    
    // Print current selection to console for debugging
    println!("Selected: {} - Kobold: {}, Troglodyte: {}, Diff: {}", 
             selected.name, selected.kobold, selected.troglodyte, diff);
}