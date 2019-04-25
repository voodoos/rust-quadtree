use std::collections::LinkedList;

#[derive(Debug)]
pub struct AABB {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}
pub trait Collidable {
    fn bounding_box(&self) -> &AABB;
}

#[derive(Debug)]
pub struct TestVal {
    pub bbox: AABB,
}

impl Collidable for TestVal {
    fn bounding_box(&self) -> &AABB {
        return &self.bbox;
    }
}

/// Four quadrants enum
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// A QuadTree
#[derive(Debug)]
pub struct QuadTree<T: Collidable> {
    zone: AABB,
    max_values: usize,
    max_depth: usize,
    children: LinkedList<QuadTree<T>>,
    values: LinkedList<T>,
}

impl<T: Collidable> QuadTree<T> {
    /// Creates a new QuadTree
    /// with given arguments
    fn new(
        max_values: usize,
        max_depth: usize,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> QuadTree<T> {
        QuadTree {
            max_values,
            max_depth,
            zone: AABB { x, y, w, h },
            children: LinkedList::default(),
            values: LinkedList::default(),
        }
    }

    /// Creates a child of the current tree.
    ///
    /// A child has depth - 1 compared to its parent
    /// and is focused on one of the four quadrants
    fn new_child(&self, q: Quadrant) -> QuadTree<T> {
        use Quadrant::*;
        let z = &self.zone;

        let (x, y, w, h) = match q {
            TopLeft => (z.x, z.y, z.w / 2, z.h / 2),
            TopRight => (z.w / 2 + 1, z.y, z.w - (z.w / 2), z.h / 2),
            BottomLeft => (z.x, z.h / 2 + 1, z.w / 2, z.h - (z.h / 2)),
            BottomRight => (z.w / 2 + 1, z.h / 2 + 1, z.w - (z.w / 2), z.h - (z.h / 2)),
        };

        QuadTree::<T> {
            zone: AABB { x, y, w, h },
            max_depth: self.max_depth - 1,
            max_values: self.max_values,
            children: LinkedList::<QuadTree<T>>::default(),
            values: LinkedList::<T>::default(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn insert(&mut self, v: T) {
        self.values.push_back(v);
        self.split()
    }

    fn split(&mut self) {
        if self.children.is_empty() {
            use Quadrant::*;
            self.children
                .push_back(QuadTree::<T>::new_child(self, TopLeft));
            self.children
                .push_back(QuadTree::<T>::new_child(self, TopRight));
            self.children
                .push_back(QuadTree::<T>::new_child(self, BottomLeft));
            self.children
                .push_back(QuadTree::<T>::new_child(self, BottomRight));
        }
    }
}

impl<T: Collidable> Default for QuadTree<T> {
    fn default() -> Self {
        QuadTree::<T>::new(4, 4, 0, 0, 256, 256)
    }
}
