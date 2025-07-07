//! TinyText: 3D cube-based vector font for all printable Latin characters (ASCII)
// Supports: adjustable offset, orientation, and scale using stacked cubes
// Usage: TinyText::spawn_text(commands, meshes, materials, "Hello!", origin, scale, orientation, offset, color)

use bevy::prelude::*;
use bevy::math::primitives::Cuboid;


/// Cube position definition for building letters
#[derive(Clone, Copy, Debug)]
pub struct CubePos {
    pub x: i32,
    pub y: i32,
}

/// Glyph definition: a vector of cube positions
pub type Glyph = Vec<CubePos>;

/// 3D cube-based font for printable ASCII (32-126)
pub struct TinyFont {
    glyphs: [Option<Glyph>; 95], // ASCII 32-126
}

impl TinyFont {
    pub fn new() -> Self {
        let mut glyphs: [Option<Glyph>; 95] = std::array::from_fn(|_| None);
        
        // SPACE (32)
        glyphs[0] = Some(vec![]);
        
        // ! (33)
        glyphs[1] = Some(vec![
            CubePos { x: 2, y: 0 },
            CubePos { x: 2, y: 1 },
            CubePos { x: 2, y: 2 },
            CubePos { x: 2, y: 3 },
            CubePos { x: 2, y: 5 },
            CubePos { x: 2, y: 6 },
            CubePos { x: 2, y: 7 },
        ]);
        
        // " (34)
        glyphs[2] = Some(vec![
            CubePos { x: 1, y: 5 },
            CubePos { x: 1, y: 6 },
            CubePos { x: 1, y: 7 },
            CubePos { x: 3, y: 5 },
            CubePos { x: 3, y: 6 },
            CubePos { x: 3, y: 7 },
        ]);
        
        // # (35)
        glyphs[3] = Some(vec![
            CubePos { x: 1, y: 1 }, CubePos { x: 1, y: 2 }, CubePos { x: 1, y: 3 },
            CubePos { x: 1, y: 4 }, CubePos { x: 1, y: 5 }, CubePos { x: 1, y: 6 },
            CubePos { x: 3, y: 1 }, CubePos { x: 3, y: 2 }, CubePos { x: 3, y: 3 },
            CubePos { x: 3, y: 4 }, CubePos { x: 3, y: 5 }, CubePos { x: 3, y: 6 },
            CubePos { x: 0, y: 2 }, CubePos { x: 2, y: 2 }, CubePos { x: 4, y: 2 },
            CubePos { x: 0, y: 5 }, CubePos { x: 2, y: 5 }, CubePos { x: 4, y: 5 },
        ]);
        
        // $ (36)
        glyphs[4] = Some(vec![
            CubePos { x: 2, y: 0 }, CubePos { x: 2, y: 1 }, CubePos { x: 2, y: 2 },
            CubePos { x: 2, y: 3 }, CubePos { x: 2, y: 4 }, CubePos { x: 2, y: 5 },
            CubePos { x: 2, y: 6 }, CubePos { x: 2, y: 7 },
            CubePos { x: 0, y: 1 }, CubePos { x: 1, y: 1 }, CubePos { x: 3, y: 1 },
            CubePos { x: 0, y: 2 }, CubePos { x: 1, y: 3 }, CubePos { x: 3, y: 3 },
            CubePos { x: 0, y: 4 }, CubePos { x: 1, y: 5 }, CubePos { x: 3, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 6 }, CubePos { x: 3, y: 6 },
        ]);
        
        // % (37)
        glyphs[5] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 1 }, CubePos { x: 2, y: 2 },
            CubePos { x: 3, y: 3 }, CubePos { x: 4, y: 4 }, CubePos { x: 5, y: 5 },
            CubePos { x: 6, y: 6 }, CubePos { x: 7, y: 7 },
            CubePos { x: 0, y: 5 }, CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 6 },
            CubePos { x: 1, y: 5 }, CubePos { x: 6, y: 1 }, CubePos { x: 6, y: 2 },
            CubePos { x: 7, y: 2 }, CubePos { x: 7, y: 1 },
        ]);
        
        // & (38)
        glyphs[6] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 0, y: 1 }, CubePos { x: 4, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 3 }, CubePos { x: 0, y: 4 },
            CubePos { x: 3, y: 4 }, CubePos { x: 0, y: 5 }, CubePos { x: 4, y: 5 },
            CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 }, CubePos { x: 4, y: 6 },
            CubePos { x: 5, y: 6 },
        ]);
        
        // ' (39)
        glyphs[7] = Some(vec![
            CubePos { x: 2, y: 5 },
            CubePos { x: 2, y: 6 },
            CubePos { x: 2, y: 7 },
        ]);
        
        // ( (40)
        glyphs[8] = Some(vec![
            CubePos { x: 2, y: 1 }, CubePos { x: 1, y: 2 }, CubePos { x: 1, y: 3 },
            CubePos { x: 1, y: 4 }, CubePos { x: 1, y: 5 }, CubePos { x: 2, y: 6 },
        ]);
        
        // ) (41)
        glyphs[9] = Some(vec![
            CubePos { x: 1, y: 1 }, CubePos { x: 2, y: 2 }, CubePos { x: 2, y: 3 },
            CubePos { x: 2, y: 4 }, CubePos { x: 2, y: 5 }, CubePos { x: 1, y: 6 },
        ]);
        
        // * (42)
        glyphs[10] = Some(vec![
            CubePos { x: 2, y: 2 }, CubePos { x: 2, y: 3 }, CubePos { x: 2, y: 4 },
            CubePos { x: 1, y: 3 }, CubePos { x: 3, y: 3 }, CubePos { x: 1, y: 2 },
            CubePos { x: 3, y: 4 }, CubePos { x: 1, y: 4 }, CubePos { x: 3, y: 2 },
        ]);
        
        // + (43)
        glyphs[11] = Some(vec![
            CubePos { x: 2, y: 1 }, CubePos { x: 2, y: 2 }, CubePos { x: 2, y: 3 },
            CubePos { x: 2, y: 4 }, CubePos { x: 2, y: 5 }, CubePos { x: 0, y: 3 },
            CubePos { x: 1, y: 3 }, CubePos { x: 3, y: 3 }, CubePos { x: 4, y: 3 },
        ]);
        
        // , (44)
        glyphs[12] = Some(vec![
            CubePos { x: 2, y: 0 },
            CubePos { x: 1, y: 1 },
        ]);
        
        // - (45)
        glyphs[13] = Some(vec![
            CubePos { x: 0, y: 3 }, CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 3 },
            CubePos { x: 3, y: 3 }, CubePos { x: 4, y: 3 },
        ]);
        
        // . (46)
        glyphs[14] = Some(vec![
            CubePos { x: 2, y: 0 },
        ]);
        
        // / (47)
        glyphs[15] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 1 }, CubePos { x: 2, y: 2 },
            CubePos { x: 3, y: 3 }, CubePos { x: 4, y: 4 }, CubePos { x: 5, y: 5 },
            CubePos { x: 6, y: 6 }, CubePos { x: 7, y: 7 },
        ]);
        
        // 0 (48)
        glyphs[16] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 0, y: 1 }, CubePos { x: 4, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 4, y: 2 }, CubePos { x: 0, y: 3 }, CubePos { x: 4, y: 3 },
            CubePos { x: 0, y: 4 }, CubePos { x: 4, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 4, y: 5 }, CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 },
            CubePos { x: 3, y: 6 },
        ]);
        
        // 1 (49)
        glyphs[17] = Some(vec![
            CubePos { x: 2, y: 0 }, CubePos { x: 2, y: 1 }, CubePos { x: 2, y: 2 },
            CubePos { x: 2, y: 3 }, CubePos { x: 2, y: 4 }, CubePos { x: 2, y: 5 },
            CubePos { x: 2, y: 6 }, CubePos { x: 1, y: 5 }, CubePos { x: 0, y: 0 },
            CubePos { x: 1, y: 0 }, CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 0 },
        ]);
        
        // 2 (50)
        glyphs[18] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 0 }, CubePos { x: 0, y: 1 },
            CubePos { x: 1, y: 2 }, CubePos { x: 2, y: 3 }, CubePos { x: 3, y: 4 },
            CubePos { x: 4, y: 5 }, CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 6 },
            CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 },
        ]);
        
        // 3 (51)
        glyphs[19] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 2 },
            CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 3 }, CubePos { x: 3, y: 3 },
            CubePos { x: 4, y: 4 }, CubePos { x: 4, y: 5 }, CubePos { x: 0, y: 6 },
            CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 },
        ]);
        
        // 4 (52)
        glyphs[20] = Some(vec![
            CubePos { x: 0, y: 3 }, CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 3 },
            CubePos { x: 3, y: 3 }, CubePos { x: 4, y: 3 }, CubePos { x: 3, y: 0 },
            CubePos { x: 3, y: 1 }, CubePos { x: 3, y: 2 }, CubePos { x: 3, y: 4 },
            CubePos { x: 3, y: 5 }, CubePos { x: 3, y: 6 }, CubePos { x: 0, y: 4 },
            CubePos { x: 1, y: 5 }, CubePos { x: 2, y: 6 },
        ]);
        
        // 5 (53)
        glyphs[21] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 3 },
            CubePos { x: 3, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 },
            CubePos { x: 3, y: 6 }, CubePos { x: 4, y: 6 },
        ]);
        
        // 6 (54)
        glyphs[22] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 0, y: 1 }, CubePos { x: 4, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 4, y: 2 }, CubePos { x: 0, y: 3 }, CubePos { x: 1, y: 3 },
            CubePos { x: 2, y: 3 }, CubePos { x: 3, y: 3 }, CubePos { x: 0, y: 4 },
            CubePos { x: 0, y: 5 }, CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 },
            CubePos { x: 3, y: 6 },
        ]);
        
        // 7 (55)
        glyphs[23] = Some(vec![
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 },
            CubePos { x: 3, y: 6 }, CubePos { x: 4, y: 6 }, CubePos { x: 4, y: 5 },
            CubePos { x: 3, y: 4 }, CubePos { x: 2, y: 3 }, CubePos { x: 1, y: 2 },
            CubePos { x: 1, y: 1 }, CubePos { x: 1, y: 0 },
        ]);
        
        // 8 (56)
        glyphs[24] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 0, y: 1 }, CubePos { x: 4, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 4, y: 2 }, CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 3 },
            CubePos { x: 3, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 4, y: 4 },
            CubePos { x: 0, y: 5 }, CubePos { x: 4, y: 5 }, CubePos { x: 1, y: 6 },
            CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 },
        ]);
        
        // 9 (57)
        glyphs[25] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 2 }, CubePos { x: 0, y: 3 },
            CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 3 }, CubePos { x: 3, y: 3 },
            CubePos { x: 4, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 4, y: 4 },
            CubePos { x: 0, y: 5 }, CubePos { x: 4, y: 5 }, CubePos { x: 1, y: 6 },
            CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 },
        ]);
        
        // : (58)
        glyphs[26] = Some(vec![
            CubePos { x: 2, y: 1 },
            CubePos { x: 2, y: 5 },
        ]);
        
        // ; (59)
        glyphs[27] = Some(vec![
            CubePos { x: 2, y: 0 }, CubePos { x: 1, y: 1 },
            CubePos { x: 2, y: 5 },
        ]);
        
        // < (60)
        glyphs[28] = Some(vec![
            CubePos { x: 4, y: 1 }, CubePos { x: 3, y: 2 }, CubePos { x: 2, y: 3 },
            CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 5 }, CubePos { x: 3, y: 6 },
            CubePos { x: 4, y: 7 },
        ]);
        
        // = (61)
        glyphs[29] = Some(vec![
            CubePos { x: 0, y: 2 }, CubePos { x: 1, y: 2 }, CubePos { x: 2, y: 2 },
            CubePos { x: 3, y: 2 }, CubePos { x: 4, y: 2 }, CubePos { x: 0, y: 4 },
            CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 4 }, CubePos { x: 3, y: 4 },
            CubePos { x: 4, y: 4 },
        ]);
        
        // > (62)
        glyphs[30] = Some(vec![
            CubePos { x: 0, y: 1 }, CubePos { x: 1, y: 2 }, CubePos { x: 2, y: 3 },
            CubePos { x: 3, y: 4 }, CubePos { x: 2, y: 5 }, CubePos { x: 1, y: 6 },
            CubePos { x: 0, y: 7 },
        ]);
        
        // ? (63)
        glyphs[31] = Some(vec![
            CubePos { x: 0, y: 5 }, CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 },
            CubePos { x: 3, y: 6 }, CubePos { x: 4, y: 5 }, CubePos { x: 4, y: 4 },
            CubePos { x: 3, y: 3 }, CubePos { x: 2, y: 3 }, CubePos { x: 2, y: 2 },
            CubePos { x: 2, y: 0 },
        ]);
        
        // @ (64)
        glyphs[32] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 4, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 5, y: 1 },
            CubePos { x: 0, y: 2 }, CubePos { x: 2, y: 2 }, CubePos { x: 3, y: 2 },
            CubePos { x: 4, y: 2 }, CubePos { x: 5, y: 2 }, CubePos { x: 0, y: 3 },
            CubePos { x: 2, y: 3 }, CubePos { x: 4, y: 3 }, CubePos { x: 0, y: 4 },
            CubePos { x: 2, y: 4 }, CubePos { x: 3, y: 4 }, CubePos { x: 4, y: 4 },
            CubePos { x: 1, y: 5 }, CubePos { x: 2, y: 5 }, CubePos { x: 3, y: 5 },
            CubePos { x: 4, y: 5 },
        ]);
        
        // A (65)
        glyphs[33] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 4, y: 0 }, CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 2 },
            CubePos { x: 4, y: 3 }, CubePos { x: 4, y: 4 }, CubePos { x: 4, y: 5 },
            CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 },
            CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 3 }, CubePos { x: 3, y: 3 },
        ]);
        
        // B (66)
        glyphs[34] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 2 },
            CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 3 }, CubePos { x: 3, y: 3 },
            CubePos { x: 4, y: 4 }, CubePos { x: 4, y: 5 }, CubePos { x: 1, y: 6 },
            CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 },
        ]);
        
        // C (67)
        glyphs[35] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 4, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 },
            CubePos { x: 4, y: 6 },
        ]);
        
        // D (68)
        glyphs[36] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 2 },
            CubePos { x: 4, y: 3 }, CubePos { x: 4, y: 4 }, CubePos { x: 4, y: 5 },
            CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 },
        ]);
        
        // E (69)
        glyphs[37] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 0 }, CubePos { x: 1, y: 3 },
            CubePos { x: 2, y: 3 }, CubePos { x: 3, y: 3 }, CubePos { x: 1, y: 6 },
            CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 }, CubePos { x: 4, y: 6 },
        ]);
        
        // F (70)
        glyphs[38] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 3 },
            CubePos { x: 3, y: 3 }, CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 },
            CubePos { x: 3, y: 6 }, CubePos { x: 4, y: 6 },
        ]);
        
        // G (71)
        glyphs[39] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 4, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 },
            CubePos { x: 4, y: 6 }, CubePos { x: 4, y: 5 }, CubePos { x: 4, y: 4 },
            CubePos { x: 3, y: 3 }, CubePos { x: 4, y: 3 },
        ]);
        
        // H (72)
        glyphs[40] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 4, y: 0 }, CubePos { x: 4, y: 1 },
            CubePos { x: 4, y: 2 }, CubePos { x: 4, y: 3 }, CubePos { x: 4, y: 4 },
            CubePos { x: 4, y: 5 }, CubePos { x: 4, y: 6 }, CubePos { x: 1, y: 3 },
            CubePos { x: 2, y: 3 }, CubePos { x: 3, y: 3 },
        ]);
        
        // I (73)
        glyphs[41] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 0 }, CubePos { x: 2, y: 1 },
            CubePos { x: 2, y: 2 }, CubePos { x: 2, y: 3 }, CubePos { x: 2, y: 4 },
            CubePos { x: 2, y: 5 }, CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 6 },
            CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 }, CubePos { x: 4, y: 6 },
        ]);
        
        // J (74)
        glyphs[42] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 1 }, CubePos { x: 3, y: 2 }, CubePos { x: 3, y: 3 },
            CubePos { x: 3, y: 4 }, CubePos { x: 3, y: 5 }, CubePos { x: 0, y: 6 },
            CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 },
            CubePos { x: 4, y: 6 },
        ]);
        
        // K (75)
        glyphs[43] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 4, y: 0 }, CubePos { x: 3, y: 1 },
            CubePos { x: 2, y: 2 }, CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 4 },
            CubePos { x: 3, y: 5 }, CubePos { x: 4, y: 6 },
        ]);
        
        // L (76)
        glyphs[44] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 0 },
        ]);
        
        // M (77)
        glyphs[45] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 6, y: 0 }, CubePos { x: 6, y: 1 },
            CubePos { x: 6, y: 2 }, CubePos { x: 6, y: 3 }, CubePos { x: 6, y: 4 },
            CubePos { x: 6, y: 5 }, CubePos { x: 6, y: 6 }, CubePos { x: 1, y: 5 },
            CubePos { x: 2, y: 4 }, CubePos { x: 3, y: 3 }, CubePos { x: 4, y: 4 },
            CubePos { x: 5, y: 5 },
        ]);
        
        // N (78)
        glyphs[46] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 4, y: 0 }, CubePos { x: 4, y: 1 },
            CubePos { x: 4, y: 2 }, CubePos { x: 4, y: 3 }, CubePos { x: 4, y: 4 },
            CubePos { x: 4, y: 5 }, CubePos { x: 4, y: 6 }, CubePos { x: 1, y: 1 },
            CubePos { x: 2, y: 2 }, CubePos { x: 3, y: 3 },
        ]);
        
        // O (79)
        glyphs[47] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 0, y: 1 }, CubePos { x: 4, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 4, y: 2 }, CubePos { x: 0, y: 3 }, CubePos { x: 4, y: 3 },
            CubePos { x: 0, y: 4 }, CubePos { x: 4, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 4, y: 5 }, CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 },
            CubePos { x: 3, y: 6 },
        ]);
        
        // P (80)
        glyphs[48] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 3 },
            CubePos { x: 3, y: 3 }, CubePos { x: 4, y: 4 }, CubePos { x: 4, y: 5 },
            CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 },
        ]);
        
        // Q (81)
        glyphs[49] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 0, y: 1 }, CubePos { x: 4, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 4, y: 2 }, CubePos { x: 0, y: 3 }, CubePos { x: 4, y: 3 },
            CubePos { x: 0, y: 4 }, CubePos { x: 4, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 4, y: 5 }, CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 },
            CubePos { x: 3, y: 6 }, CubePos { x: 3, y: 1 }, CubePos { x: 4, y: 0 },
        ]);
        
        // R (82)
        glyphs[50] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 3 },
            CubePos { x: 3, y: 3 }, CubePos { x: 4, y: 4 }, CubePos { x: 4, y: 5 },
            CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 },
            CubePos { x: 2, y: 2 }, CubePos { x: 3, y: 1 }, CubePos { x: 4, y: 0 },
        ]);
        
        // S (83)
        glyphs[51] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 2 },
            CubePos { x: 3, y: 3 }, CubePos { x: 2, y: 3 }, CubePos { x: 1, y: 3 },
            CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 }, CubePos { x: 1, y: 6 },
            CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 }, CubePos { x: 4, y: 6 },
        ]);
        
        // T (84)
        glyphs[52] = Some(vec![
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 },
            CubePos { x: 3, y: 6 }, CubePos { x: 4, y: 6 }, CubePos { x: 2, y: 0 },
            CubePos { x: 2, y: 1 }, CubePos { x: 2, y: 2 }, CubePos { x: 2, y: 3 },
            CubePos { x: 2, y: 4 }, CubePos { x: 2, y: 5 },
        ]);
        
        // U (85)
        glyphs[53] = Some(vec![
            CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 }, CubePos { x: 0, y: 3 },
            CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 }, CubePos { x: 0, y: 6 },
            CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 2 }, CubePos { x: 4, y: 3 },
            CubePos { x: 4, y: 4 }, CubePos { x: 4, y: 5 }, CubePos { x: 4, y: 6 },
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
        ]);
        
        // V (86)
        glyphs[54] = Some(vec![
            CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 }, CubePos { x: 0, y: 6 },
            CubePos { x: 1, y: 2 }, CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 0 },
            CubePos { x: 2, y: 1 }, CubePos { x: 3, y: 2 }, CubePos { x: 3, y: 3 },
            CubePos { x: 4, y: 4 }, CubePos { x: 4, y: 5 }, CubePos { x: 4, y: 6 },
        ]);
        
        // W (87)
        glyphs[55] = Some(vec![
            CubePos { x: 0, y: 2 }, CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 },
            CubePos { x: 0, y: 5 }, CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 0 },
            CubePos { x: 1, y: 1 }, CubePos { x: 2, y: 2 }, CubePos { x: 2, y: 3 },
            CubePos { x: 3, y: 0 }, CubePos { x: 3, y: 1 }, CubePos { x: 4, y: 2 },
            CubePos { x: 4, y: 3 }, CubePos { x: 4, y: 4 }, CubePos { x: 4, y: 5 },
            CubePos { x: 4, y: 6 },
        ]);
        
        // X (88)
        glyphs[56] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 1 }, CubePos { x: 2, y: 2 },
            CubePos { x: 2, y: 3 }, CubePos { x: 2, y: 4 }, CubePos { x: 3, y: 5 },
            CubePos { x: 4, y: 6 }, CubePos { x: 4, y: 0 }, CubePos { x: 3, y: 1 },
            CubePos { x: 1, y: 5 }, CubePos { x: 0, y: 6 },
        ]);
        
        // Y (89)
        glyphs[57] = Some(vec![
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 5 }, CubePos { x: 2, y: 4 },
            CubePos { x: 2, y: 3 }, CubePos { x: 2, y: 2 }, CubePos { x: 2, y: 1 },
            CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 5 }, CubePos { x: 4, y: 6 },
        ]);
        
        // Z (90)
        glyphs[58] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 0 }, CubePos { x: 4, y: 1 },
            CubePos { x: 3, y: 2 }, CubePos { x: 2, y: 3 }, CubePos { x: 1, y: 4 },
            CubePos { x: 0, y: 5 }, CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 6 },
            CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 }, CubePos { x: 4, y: 6 },
        ]);
        
        // [ (91)
        glyphs[59] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 1, y: 1 }, CubePos { x: 1, y: 2 },
            CubePos { x: 1, y: 3 }, CubePos { x: 1, y: 4 }, CubePos { x: 1, y: 5 },
            CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 },
        ]);
        
        // \ (92)
        glyphs[60] = Some(vec![
            CubePos { x: 0, y: 7 }, CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 5 },
            CubePos { x: 3, y: 4 }, CubePos { x: 4, y: 3 }, CubePos { x: 5, y: 2 },
            CubePos { x: 6, y: 1 }, CubePos { x: 7, y: 0 },
        ]);
        
        // ] (93)
        glyphs[61] = Some(vec![
            CubePos { x: 3, y: 0 }, CubePos { x: 3, y: 1 }, CubePos { x: 3, y: 2 },
            CubePos { x: 3, y: 3 }, CubePos { x: 3, y: 4 }, CubePos { x: 3, y: 5 },
            CubePos { x: 3, y: 6 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 1, y: 6 }, CubePos { x: 2, y: 6 },
        ]);
        
        // ^ (94)
        glyphs[62] = Some(vec![
            CubePos { x: 2, y: 7 }, CubePos { x: 1, y: 6 }, CubePos { x: 3, y: 6 },
            CubePos { x: 0, y: 5 }, CubePos { x: 4, y: 5 },
        ]);
        
        // _ (95)
        glyphs[63] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 0 },
        ]);
        
        // ` (96)
        glyphs[64] = Some(vec![
            CubePos { x: 1, y: 7 },
            CubePos { x: 2, y: 6 },
        ]);
        
        // a-z (lowercase) - simplified versions
        // a (97)
        glyphs[65] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 2 }, CubePos { x: 4, y: 3 },
            CubePos { x: 4, y: 4 }, CubePos { x: 0, y: 2 }, CubePos { x: 1, y: 2 },
            CubePos { x: 2, y: 2 }, CubePos { x: 3, y: 2 }, CubePos { x: 0, y: 4 },
            CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 4 }, CubePos { x: 3, y: 4 },
        ]);
        
        // b (98)
        glyphs[66] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 2 },
            CubePos { x: 4, y: 3 }, CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 4 },
            CubePos { x: 3, y: 4 },
        ]);
        
        // c (99)
        glyphs[67] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 }, CubePos { x: 0, y: 3 },
            CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 4 }, CubePos { x: 3, y: 4 },
        ]);
        
        // d (100)
        glyphs[68] = Some(vec![
            CubePos { x: 4, y: 0 }, CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 2 },
            CubePos { x: 4, y: 3 }, CubePos { x: 4, y: 4 }, CubePos { x: 4, y: 5 },
            CubePos { x: 4, y: 6 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 4 },
            CubePos { x: 3, y: 4 },
        ]);
        
        // e (101)
        glyphs[69] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 }, CubePos { x: 1, y: 2 },
            CubePos { x: 2, y: 2 }, CubePos { x: 3, y: 2 }, CubePos { x: 4, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 4 },
            CubePos { x: 3, y: 4 },
        ]);
        
        // f (102)
        glyphs[70] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 1, y: 1 }, CubePos { x: 1, y: 2 },
            CubePos { x: 1, y: 3 }, CubePos { x: 1, y: 4 }, CubePos { x: 1, y: 5 },
            CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 6 }, CubePos { x: 0, y: 3 },
            CubePos { x: 2, y: 3 },
        ]);
        
        // g (103)
        glyphs[71] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 2 }, CubePos { x: 4, y: 3 },
            CubePos { x: 4, y: 4 }, CubePos { x: 0, y: 2 }, CubePos { x: 1, y: 2 },
            CubePos { x: 2, y: 2 }, CubePos { x: 3, y: 2 }, CubePos { x: 0, y: 4 },
            CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 4 }, CubePos { x: 3, y: 4 },
            CubePos { x: 0, y: -1 }, CubePos { x: 1, y: -2 }, CubePos { x: 2, y: -2 },
            CubePos { x: 3, y: -2 },
        ]);
        
        // h (104)
        glyphs[72] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 1, y: 2 }, CubePos { x: 2, y: 2 },
            CubePos { x: 3, y: 2 }, CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 0 },
        ]);
        
        // i (105)
        glyphs[73] = Some(vec![
            CubePos { x: 2, y: 0 }, CubePos { x: 2, y: 1 }, CubePos { x: 2, y: 2 },
            CubePos { x: 2, y: 3 }, CubePos { x: 2, y: 4 }, CubePos { x: 2, y: 6 },
        ]);
        
        // j (106)
        glyphs[74] = Some(vec![
            CubePos { x: 3, y: 0 }, CubePos { x: 3, y: 1 }, CubePos { x: 3, y: 2 },
            CubePos { x: 3, y: 3 }, CubePos { x: 3, y: 4 }, CubePos { x: 3, y: 6 },
            CubePos { x: 0, y: -1 }, CubePos { x: 1, y: -1 }, CubePos { x: 2, y: -1 },
        ]);
        
        // k (107)
        glyphs[75] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 0, y: 5 },
            CubePos { x: 0, y: 6 }, CubePos { x: 3, y: 0 }, CubePos { x: 2, y: 1 },
            CubePos { x: 1, y: 2 }, CubePos { x: 2, y: 3 }, CubePos { x: 3, y: 4 },
        ]);
        
        // l (108)
        glyphs[76] = Some(vec![
            CubePos { x: 2, y: 0 }, CubePos { x: 2, y: 1 }, CubePos { x: 2, y: 2 },
            CubePos { x: 2, y: 3 }, CubePos { x: 2, y: 4 }, CubePos { x: 2, y: 5 },
            CubePos { x: 2, y: 6 },
        ]);
        
        // m (109)
        glyphs[77] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 1, y: 2 },
            CubePos { x: 2, y: 1 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 2 },
            CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 0 }, CubePos { x: 6, y: 0 },
            CubePos { x: 6, y: 1 }, CubePos { x: 6, y: 2 }, CubePos { x: 6, y: 3 },
            CubePos { x: 6, y: 4 },
        ]);
        
        // n (110)
        glyphs[78] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 1, y: 2 },
            CubePos { x: 2, y: 2 }, CubePos { x: 3, y: 2 }, CubePos { x: 4, y: 1 },
            CubePos { x: 4, y: 0 },
        ]);
        
        // o (111)
        glyphs[79] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
            CubePos { x: 0, y: 1 }, CubePos { x: 4, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 4, y: 2 }, CubePos { x: 0, y: 3 }, CubePos { x: 4, y: 3 },
            CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 4 }, CubePos { x: 3, y: 4 },
        ]);
        
        // p (112)
        glyphs[80] = Some(vec![
            CubePos { x: 0, y: -2 }, CubePos { x: 0, y: -1 }, CubePos { x: 0, y: 0 },
            CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 }, CubePos { x: 0, y: 3 },
            CubePos { x: 0, y: 4 }, CubePos { x: 1, y: 2 }, CubePos { x: 2, y: 2 },
            CubePos { x: 3, y: 2 }, CubePos { x: 4, y: 3 }, CubePos { x: 4, y: 4 },
            CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 4 }, CubePos { x: 3, y: 4 },
        ]);
        
        // q (113)
        glyphs[81] = Some(vec![
            CubePos { x: 4, y: -2 }, CubePos { x: 4, y: -1 }, CubePos { x: 4, y: 0 },
            CubePos { x: 4, y: 1 }, CubePos { x: 4, y: 2 }, CubePos { x: 4, y: 3 },
            CubePos { x: 4, y: 4 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 4 },
            CubePos { x: 3, y: 4 },
        ]);
        
        // r (114)
        glyphs[82] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 },
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 1, y: 2 },
            CubePos { x: 2, y: 3 }, CubePos { x: 3, y: 4 },
        ]);
        
        // s (115)
        glyphs[83] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 1 }, CubePos { x: 3, y: 2 },
            CubePos { x: 2, y: 2 }, CubePos { x: 1, y: 2 }, CubePos { x: 0, y: 3 },
            CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 4 }, CubePos { x: 3, y: 4 },
            CubePos { x: 4, y: 4 },
        ]);
        
        // t (116)
        glyphs[84] = Some(vec![
            CubePos { x: 1, y: 0 }, CubePos { x: 1, y: 1 }, CubePos { x: 1, y: 2 },
            CubePos { x: 1, y: 3 }, CubePos { x: 1, y: 4 }, CubePos { x: 1, y: 5 },
            CubePos { x: 0, y: 3 }, CubePos { x: 2, y: 3 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 },
        ]);
        
        // u (117)
        glyphs[85] = Some(vec![
            CubePos { x: 0, y: 1 }, CubePos { x: 0, y: 2 }, CubePos { x: 0, y: 3 },
            CubePos { x: 0, y: 4 }, CubePos { x: 4, y: 0 }, CubePos { x: 4, y: 1 },
            CubePos { x: 4, y: 2 }, CubePos { x: 4, y: 3 }, CubePos { x: 4, y: 4 },
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 0 },
        ]);
        
        // v (118)
        glyphs[86] = Some(vec![
            CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 }, CubePos { x: 1, y: 1 },
            CubePos { x: 1, y: 2 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 1 },
            CubePos { x: 3, y: 2 }, CubePos { x: 4, y: 3 }, CubePos { x: 4, y: 4 },
        ]);
        
        // w (119)
        glyphs[87] = Some(vec![
            CubePos { x: 0, y: 2 }, CubePos { x: 0, y: 3 }, CubePos { x: 0, y: 4 },
            CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 1 }, CubePos { x: 2, y: 2 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 2 }, CubePos { x: 4, y: 3 },
            CubePos { x: 4, y: 4 },
        ]);
        
        // x (120)
        glyphs[88] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 1 }, CubePos { x: 2, y: 2 },
            CubePos { x: 3, y: 3 }, CubePos { x: 4, y: 4 }, CubePos { x: 4, y: 0 },
            CubePos { x: 3, y: 1 }, CubePos { x: 1, y: 3 }, CubePos { x: 0, y: 4 },
        ]);
        
        // y (121)
        glyphs[89] = Some(vec![
            CubePos { x: 0, y: 4 }, CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 2 },
            CubePos { x: 2, y: 1 }, CubePos { x: 2, y: 0 }, CubePos { x: 3, y: 3 },
            CubePos { x: 4, y: 4 }, CubePos { x: 1, y: -1 }, CubePos { x: 0, y: -2 },
        ]);
        
        // z (122)
        glyphs[90] = Some(vec![
            CubePos { x: 0, y: 0 }, CubePos { x: 1, y: 0 }, CubePos { x: 2, y: 0 },
            CubePos { x: 3, y: 0 }, CubePos { x: 4, y: 0 }, CubePos { x: 4, y: 1 },
            CubePos { x: 3, y: 2 }, CubePos { x: 2, y: 2 }, CubePos { x: 1, y: 3 },
            CubePos { x: 0, y: 4 }, CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 4 },
            CubePos { x: 3, y: 4 }, CubePos { x: 4, y: 4 },
        ]);
        
        // { (123)
        glyphs[91] = Some(vec![
            CubePos { x: 2, y: 0 }, CubePos { x: 2, y: 1 }, CubePos { x: 2, y: 2 },
            CubePos { x: 1, y: 3 }, CubePos { x: 2, y: 4 }, CubePos { x: 2, y: 5 },
            CubePos { x: 2, y: 6 }, CubePos { x: 3, y: 0 }, CubePos { x: 3, y: 6 },
        ]);
        
        // | (124)
        glyphs[92] = Some(vec![
            CubePos { x: 2, y: 0 }, CubePos { x: 2, y: 1 }, CubePos { x: 2, y: 2 },
            CubePos { x: 2, y: 3 }, CubePos { x: 2, y: 4 }, CubePos { x: 2, y: 5 },
            CubePos { x: 2, y: 6 }, CubePos { x: 2, y: 7 },
        ]);
        
        // } (125)
        glyphs[93] = Some(vec![
            CubePos { x: 2, y: 0 }, CubePos { x: 2, y: 1 }, CubePos { x: 2, y: 2 },
            CubePos { x: 3, y: 3 }, CubePos { x: 2, y: 4 }, CubePos { x: 2, y: 5 },
            CubePos { x: 2, y: 6 }, CubePos { x: 1, y: 0 }, CubePos { x: 1, y: 6 },
        ]);
        
        // ~ (126)
        glyphs[94] = Some(vec![
            CubePos { x: 0, y: 3 }, CubePos { x: 1, y: 4 }, CubePos { x: 2, y: 4 },
            CubePos { x: 3, y: 3 }, CubePos { x: 4, y: 2 }, CubePos { x: 5, y: 2 },
            CubePos { x: 6, y: 3 },
        ]);

        Self { glyphs }
    }

    pub fn get_glyph(&self, c: char) -> Option<&Glyph> {
        if (' '..='~').contains(&c) {
            self.glyphs[c as usize - 32].as_ref()
        } else {
            None
        }
    }
}

/// Main TinyText API for 3D cube-based text
pub struct TinyText;

impl TinyText {
    /// Spawns a string as a series of 3D cubes (one entity per cube)
    ///   - commands: Bevy commands for spawning entities
    ///   - meshes: Mesh assets for cube geometry
    ///   - materials: Material assets for cube appearance
    ///   - text: the string to render
    ///   - origin: world position (Vec3)
    ///   - scale: Vec3 (cube size and spacing)
    ///   - orientation: rotation quaternion
    ///   - offset: Vec3 (per-character offset)
    ///   - color: cube color
    pub fn spawn_text(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        text: &str,
        origin: Vec3,
        scale: Vec3,
        orientation: Quat,
        offset: Vec3,
        color: Color,
    ) {
        let font = TinyFont::new();
        let mut cursor = origin;
        
        // Create a cube mesh to reuse
        let cube_mesh = meshes.add(Mesh::from(Cuboid { half_size: Vec3::splat(0.5) }));
        let cube_material = materials.add(StandardMaterial {
            base_color: color,
            ..default()
        });

        for c in text.chars() {
            if let Some(glyph) = font.get_glyph(c) {
                for cube_pos in glyph {
                    // Calculate cube position relative to character origin
                    let local_pos = Vec3::new(
                        cube_pos.x as f32 * scale.x,
                        cube_pos.y as f32 * scale.y,
                        0.0
                    );
                    
                    // Apply orientation and add to cursor position
                    let world_pos = cursor + orientation * local_pos;
                    
                    // Spawn the cube
                    commands.spawn(PbrBundle {
                        mesh: cube_mesh.clone(),
                        material: cube_material.clone(),
                        transform: Transform {
                            translation: world_pos,
                            rotation: orientation,
                            scale: scale,
                        },
                        ..default()
                    });
                }
            }
            
            // Advance cursor for next character
            cursor += orientation * offset;
        }
    }
    
    /// Simplified spawn function with default parameters
    pub fn spawn_simple(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        text: &str,
        origin: Vec3,
        color: Color,
    ) {
        Self::spawn_text(
            commands,
            meshes,
            materials,
            text,
            origin,
            Vec3::new(0.2, 0.2, 0.2), // scale
            Quat::IDENTITY,           // orientation
            Vec3::new(1.5, 0.0, 0.0), // offset (character spacing)
            color,
        );
    }
    
    /// Spawn text facing the camera
    pub fn spawn_billboard(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        text: &str,
        origin: Vec3,
        camera_pos: Vec3,
        color: Color,
    ) {
        let direction = (camera_pos - origin).normalize();
        let orientation = Quat::from_rotation_arc(Vec3::Z, direction);
        
        Self::spawn_text(
            commands,
            meshes,
            materials,
            text,
            origin,
            Vec3::new(0.2, 0.2, 0.2),
            orientation,
            Vec3::new(1.5, 0.0, 0.0),
            color,
        );
    }
}

// Example usage in a Bevy app:
/*
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 15.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // Add lighting
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // Spawn 3D text
    TinyText::spawn_simple(
        &mut commands,
        &mut meshes,
        &mut materials,
        "Hello World!",
        Vec3::new(-10.0, 0.0, 0.0),
        Color::rgb(1.0, 0.5, 0.0),
    );
    
    // Spawn rotated text
    TinyText::spawn_text(
        &mut commands,
        &mut meshes,
        &mut materials,
        "Bevy Rocks!",
        Vec3::new(0.0, 3.0, 0.0),
        Vec3::new(0.3, 0.3, 0.3),
        Quat::from_rotation_y(PI / 4.0),
        Vec3::new(2.0, 0.0, 0.0),
        Color::rgb(0.0, 1.0, 0.5),
    );
}
*/