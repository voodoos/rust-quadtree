extern crate sdl2;

use crate::lib::geometry::AABB;
use crate::lib::traits::*;

use sdl2::rect::Rect;
use std::time::Duration;

type WinCanvas = sdl2::render::Canvas<sdl2::video::Window>;

#[derive(Debug)]
pub struct TestVal {
    pub bbox: AABB,
}

impl Collidable for TestVal {
    fn bounding_box(&self) -> &AABB {
        return &self.bbox;
    }
}

impl Drawable for TestVal {
    fn draw(&self, canvas: &mut WinCanvas) -> Result<(), String> {
        let rect = Rect::new(self.bbox.x, self.bbox.y, self.bbox.w, self.bbox.h);
        canvas.draw_rect(rect)
    }
}

impl Dynamic for TestVal {
    fn update(&mut self, _delta: &Duration) -> bool {
        self.bbox.translate(1, 0);
        return true;
    }
}
