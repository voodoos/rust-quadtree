extern crate sdl2;

pub mod geometry;
pub mod traits;

use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::LinkedList;
use std::time::Duration;

use geometry::{Quadrant, AABB};
use traits::*;

type WinCanvas = sdl2::render::Canvas<sdl2::video::Window>;

/// A structure representing a quadtree
///
/// `QuadTrees` are spatial collections of objects.
/// If a node of a QuadTree contains too many elements it
///  will split in four quadtrees representing a
/// four-part partition of the original quadtree.
///
/// An object not fitting on a leaf of the quadtree will
/// be kept in the closest big-enough parent.
///
/// Usage:
/// ```
///let qt = QuadTree::<Obj>::default();
/// qt.insert(obj1);
/// ```
///
/// Use only with even dimensions.
///
/// *Todo: handle odd dimensions too*
#[derive(Debug)]
pub struct QuadTree<T: Collidable> {
    zone: AABB,
    max_values: u32,
    max_depth: u32,
    children: Vec<QuadTree<T>>,
    values: Vec<T>,
}

impl<T: Collidable> QuadTree<T> {
    /// Creates a new QuadTree
    /// with given arguments
    fn new(max_values: u32, max_depth: u32, x: i32, y: i32, w: u32, h: u32) -> QuadTree<T> {
        QuadTree {
            max_values,
            max_depth,
            zone: AABB { x, y, w, h },
            children: Vec::default(),
            values: Vec::default(),
        }
    }

    /// Creates a child of the current tree.
    ///
    /// A child has depth - 1 compared to its parent
    /// and is focused on one of the four quadrants
    fn new_child(&self, q: Quadrant) -> QuadTree<T> {
        QuadTree::<T> {
            zone: Quadrant::quadrant_bbox(&self.zone, q),
            max_depth: self.max_depth - 1,
            max_values: self.max_values,
            children: Vec::<QuadTree<T>>::default(),
            values: Vec::<T>::default(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Checks if a values fits in one of the current node.zone quadrants
    /// Returns `Some(Quadrant)` if it does, `None` otherwise
    fn fits(&self, v: &T) -> Option<Quadrant> {
        for q in Quadrant::all() {
            if v.bounding_box()
                .is_inside(Quadrant::quadrant_bbox(&self.zone, q))
            {
                return Some(q);
            }
        }
        return None;
    }

    /// Correctly insert a new value in a quadtree
    pub fn insert(&mut self, v: T) {
        // If the node is full and not at max-depth
        // OR node already has children
        // we try to insert in a subtree:
        if !self.is_leaf() || (self.values.len() as u32 >= self.max_values && self.max_depth > 0) {
            use Quadrant::*;
            match self.fits(&v) {
                // If v doesn't fit any quadrant, it will stay in the parent node
                None => self.values.push(v),
                Some(q) => {
                    self.split();
                    match q {
                        TopLeft => self.children[0].insert(v),
                        TopRight => self.children[1].insert(v),
                        BottomLeft => self.children[2].insert(v),
                        BottomRight => self.children[3].insert(v),
                    }
                }
            }
        } else {
            // If the actual not is not full or at max-depth:
            self.values.push(v);
        }
    }

    /// Split a leaf in four sub trees
    ///
    /// If node is not a leaf nothing happen.
    fn split(&mut self) {
        if self.children.is_empty() && self.max_depth > 0 {
            // Spawning the children
            for q in Quadrant::all() {
                self.children.push(QuadTree::<T>::new_child(self, q));
            }

            // We dispatch its actual values
            // It is a two step operation to prevent
            // infinite pop / push behavior
            let mut vals = LinkedList::<T>::default();

            while let Some(v) = self.values.pop() {
                vals.push_back(v);
            }

            while let Some(v) = vals.pop_back() {
                self.insert(v);
            }
        }
    }

    // fn get_root(&self) -> &QuadTree<T> {
    //     match self.root {
    //         None => self,
    //         Some(r) => r.get_root(),
    //     }
    // }
}

impl<T: Collidable> Default for QuadTree<T> {
    fn default() -> Self {
        QuadTree::<T>::new(1, 4, 0, 0, 256, 256)
    }
}

impl<T: Collidable + Drawable<WinCanvas>> Drawable<WinCanvas> for QuadTree<T> {
    fn draw(&self, canvas: &mut WinCanvas) -> Result<(), String> {
        let rect = Rect::from(&self.zone);

        let mut rng = rand::thread_rng();
        canvas.set_draw_color(Color::RGB(
            rng.gen_range(1, 255),
            rng.gen_range(1, 255),
            rng.gen_range(1, 255),
        ));
        canvas.draw_rect(rect)?;

        for v in self.values.iter() {
            v.draw(canvas)?;
        }

        for t in self.children.iter() {
            t.draw(canvas)?;
        }

        Ok(())
    }
}

impl<T: Collidable + Dynamic> Dynamic for QuadTree<T> {
    fn update(&mut self, delta: &Duration) -> bool {
        let mut changed = false;

        for t in &mut self.children {
            changed = t.update(delta) || changed;
        }

        let mut moved: LinkedList<usize> = LinkedList::default();

        for (i, v) in self.values.iter_mut().enumerate() {
            if v.update(delta) {
                changed = true;
                if !v.bounding_box().is_inside(self.zone) {
                    moved.push_back(i);
                }
            }
        }

        for i in moved {
            let v = self.values.remove(i);
            //self.get_root().insert(v);
        }

        return changed;
    }
}
