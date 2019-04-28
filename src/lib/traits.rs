use super::AABB;
use std::time::Duration;

pub trait Collidable {
    fn bounding_box(&self) -> &AABB;
}

pub trait Drawable<T> {
    fn draw(&self, target: &mut T) -> Result<(), String>;
}

pub trait Dynamic {
    /// Updates the object according to the time
    /// elapsed since last update call.
    ///
    /// Should return `true` if `self` was mutated.
    /// More precise info may be needed
    fn update(&mut self, delta: &Duration) -> bool;
}
