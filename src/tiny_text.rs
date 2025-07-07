//! TinyText: Minimal vector font for all printable Latin characters (ASCII)
// Supports: adjustable offset, orientation, and scale
// Usage: TinyText::spawn_text(commands, "Hello!", origin, scale, orientation, offset)

use bevy::prelude::*;

/// Stroke definition for a single line segment
#[derive(Clone, Copy, Debug)]
pub struct Stroke {
    pub start: Vec2,
    pub end: Vec2,
}

/// Glyph definition: a vector of strokes
pub type Glyph = Vec<Stroke>;

/// Minimal vector font for printable ASCII (32-126)
pub struct TinyFont {
    glyphs: [Option<Glyph>; 95], // ASCII 32-126
}

impl TinyFont {
    pub fn new() -> Self {
        let mut glyphs: [Option<Glyph>; 95] = Default::default();
        // Example: only a few glyphs for brevity. Add more as needed.
        glyphs[0] = Some(vec![]); // ' ' (space)
        glyphs['A' as usize - 32] = Some(vec![
            Stroke { start: Vec2::new(0.0, 0.0), end: Vec2::new(0.5, 1.0) },
            Stroke { start: Vec2::new(1.0, 0.0), end: Vec2::new(0.5, 1.0) },
            Stroke { start: Vec2::new(0.2, 0.5), end: Vec2::new(0.8, 0.5) },
        ]);
        glyphs['B' as usize - 32] = Some(vec![
            Stroke { start: Vec2::new(0.0, 0.0), end: Vec2::new(0.0, 1.0) },
            Stroke { start: Vec2::new(0.0, 1.0), end: Vec2::new(0.6, 0.8) },
            Stroke { start: Vec2::new(0.6, 0.8), end: Vec2::new(0.0, 0.5) },
            Stroke { start: Vec2::new(0.0, 0.5), end: Vec2::new(0.6, 0.2) },
            Stroke { start: Vec2::new(0.6, 0.2), end: Vec2::new(0.0, 0.0) },
        ]);
        // ... (fill in all printable ASCII chars)
        Self { glyphs }
    }
    pub fn get_glyph(&self, c: char) -> Option<&Glyph> {
        if (' ' ..= '~').contains(&c) {
            self.glyphs[c as usize - 32].as_ref()
        } else {
            None
        }
    }
}

/// Main TinyText API
pub struct TinyText;

impl TinyText {
    /// Spawns a string as a series of polylines (one entity per character)
    ///   - origin: world position
    ///   - scale: Vec2 (width, height)
    ///   - orientation: angle in radians
    ///   - offset: Vec2 (per-character offset)
    pub fn spawn_text(
        commands: &mut Commands,
        text: &str,
        origin: Vec3,
        scale: Vec2,
        orientation: f32,
        offset: Vec2,
        color: Color,
    ) {
        let font = TinyFont::new();
        let mut cursor = origin;
        let rot = Quat::from_rotation_z(orientation);
        for c in text.chars() {
            if let Some(glyph) = font.get_glyph(c) {
                for stroke in glyph {
                    let start = rot * (Vec3::new(stroke.start.x, stroke.start.y, 0.0) * scale.extend(1.0)) + cursor;
                    let end = rot * (Vec3::new(stroke.end.x, stroke.end.y, 0.0) * scale.extend(1.0)) + cursor;
                    commands.spawn(
                        GeometryBuilder::build_as(
                            &shapes::Line(start.truncate(), end.truncate()),
                            DrawMode::Stroke(StrokeMode::new(color, 0.03)),
                            Transform::default(),
                        )
                    );
                }
            }
            // Advance cursor for next character
            cursor += rot * Vec3::new(offset.x, offset.y, 0.0);
        }
    }
}

// Note: Requires bevy_prototype_lyon for GeometryBuilder and shapes::Line
// Add to Cargo.toml:
// bevy_prototype_lyon = "0.10"
