extern crate sdl2;

use sdl2::rect::Rect;

/// Rectangular bounding box
/// ```text
///   x     w
/// y +⎯⎯⎯⎯⎯⎯˃
///   |
/// h |
///   ˅
/// ```
#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl AABB {
    /// Translate the box
    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    /// Inclusion test
    ///
    /// Tests if this box is inside another one
    pub fn is_inside(&self, other: AABB) -> bool {
        self.x >= other.x
            && self.x + self.w as i32 <= other.x + other.w as i32
            && self.y >= other.y
            && self.y + self.h as i32 <= other.y + other.h as i32
    }
}

/// Creates an AABB from a tuple
impl From<(i32, i32, u32, u32)> for AABB {
    fn from((x, y, w, h): (i32, i32, u32, u32)) -> AABB {
        AABB { x, y, w, h }
    }
}

/// Creates a SDL2 Rect from an AABB
impl From<&AABB> for Rect {
    fn from(bbox: &AABB) -> Self {
        Rect::new(bbox.x, bbox.y, bbox.w, bbox.h)
    }
}

/// Four quadrants enum
#[derive(Copy, Clone)]
pub enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Quadrant {
    /// Computes the bounding box of a quadrant
    pub fn quadrant_bbox(bbox: &AABB, q: Quadrant) -> AABB {
        use Quadrant::*;
        let z = &bbox;
        AABB::from(match q {
            TopLeft => (z.x, z.y, (z.w / 2) + (z.w % 2), (z.h / 2) + (z.h % 2)),
            TopRight => (
                z.x + (z.w / 2 + 1) as i32,
                z.y,
                z.w - (z.w / 2),
                (z.h / 2) + (z.h % 2),
            ),
            BottomLeft => (
                z.x,
                z.y + (z.h / 2 + 1) as i32,
                (z.w / 2) + (z.w % 2),
                z.h - (z.h / 2),
            ),
            BottomRight => (
                z.x + (z.w / 2 + 1) as i32,
                z.y + (z.h / 2 + 1) as i32,
                z.w - (z.w / 2),
                z.h - (z.h / 2),
            ),
        })
    }

    pub fn all() -> Vec<Quadrant> {
        vec![
            Quadrant::TopLeft,
            Quadrant::TopRight,
            Quadrant::BottomLeft,
            Quadrant::BottomRight,
        ]
    }
}
