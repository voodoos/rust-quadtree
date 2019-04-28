extern crate sdl2;

use super::AABB;
use std::time::Duration;

type WinCanvas = sdl2::render::Canvas<sdl2::video::Window>;

pub trait Collidable {
    fn bounding_box(&self) -> &AABB;
}

pub trait Drawable {
    fn draw(&self, canvas: &mut WinCanvas) -> Result<(), String>;
}

pub trait Dynamic {
    /// Updates the object according to the time
    /// elapsed since last update call.
    ///
    /// Should return `true` if `self` was mutated.
    /// More precise info may be needed
    fn update(&mut self, delta: &Duration) -> bool;
}
