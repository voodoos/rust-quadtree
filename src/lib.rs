use std::collections::LinkedList;
/// Rectangular bounding box
/// ```text
///   x     w
/// y +⎯⎯⎯⎯⎯⎯˃
///   |
/// h |
///   ˅
/// ```
#[derive(Debug)]
pub struct AABB {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl AABB {
    /// Inclusion test
    ///
    /// Tests if this box is inside another one
    pub fn is_inside(&self, other: &AABB) -> bool {
        if self.x >= other.x
            && self.x + self.w <= other.x + other.w
            && self.y >= other.y
            && self.y + self.h <= other.y + other.h
        {
            true
        } else {
            false
        }
    }

    /// Creates an AABB from a tuple
    pub fn from_tuple((x, y, w, h): (usize, usize, usize, usize)) -> AABB {
        AABB { x, y, w, h }
    }
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

impl Quadrant {
    /// Computes the bounding box of a quadrant
    pub fn quadrant_bbox(bbox: &AABB, q: &Quadrant) -> (usize, usize, usize, usize) {
        use Quadrant::*;
        let z = &bbox;
        match q {
            TopLeft => (z.x, z.y, (z.w / 2) + (z.w % 2), (z.h / 2) + (z.h % 2)),
            TopRight => (z.w / 2 + 1, z.y, z.w - (z.w / 2), (z.h / 2) + (z.h % 2)),
            BottomLeft => (z.x, z.h / 2 + 1, (z.w / 2) + (z.w % 2), z.h - (z.h / 2)),
            BottomRight => (z.w / 2 + 1, z.h / 2 + 1, z.w - (z.w / 2), z.h - (z.h / 2)),
        }
    }
}

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
    max_values: usize,
    max_depth: usize,
    children: Vec<QuadTree<T>>,
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
            children: Vec::default(),
            values: LinkedList::default(),
        }
    }

    /// Creates a child of the current tree.
    ///
    /// A child has depth - 1 compared to its parent
    /// and is focused on one of the four quadrants
    fn new_child(&self, q: Quadrant) -> QuadTree<T> {
        let z = &self.zone;

        let (x, y, w, h) = Quadrant::quadrant_bbox(&self.zone, &q);

        QuadTree::<T> {
            zone: AABB { x, y, w, h },
            max_depth: self.max_depth - 1,
            max_values: self.max_values,
            children: Vec::<QuadTree<T>>::default(),
            values: LinkedList::<T>::default(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Checks if a values fits in one of the current node.AABB quadrants
    /// Returns `Some(Quadrant)` if it does, `None` otherwise
    fn fits(&self, v: &T) -> Option<Quadrant> {
        use Quadrant::*;
        if v.bounding_box()
            .is_inside(&AABB::from_tuple(Quadrant::quadrant_bbox(
                &self.zone, &TopLeft,
            )))
        {
            Some(TopLeft)
        } else if v
            .bounding_box()
            .is_inside(&AABB::from_tuple(Quadrant::quadrant_bbox(
                &self.zone, &TopRight,
            )))
        {
            Some(TopRight)
        } else if v
            .bounding_box()
            .is_inside(&AABB::from_tuple(Quadrant::quadrant_bbox(
                &self.zone,
                &BottomLeft,
            )))
        {
            Some(BottomLeft)
        } else if v
            .bounding_box()
            .is_inside(&AABB::from_tuple(Quadrant::quadrant_bbox(
                &self.zone,
                &BottomRight,
            )))
        {
            Some(BottomRight)
        } else {
            None
        }
    }

    /// Correctly insert a new value in a quadtree
    pub fn insert(&mut self, v: T) {
        // If the node is full and not at max-depth we try to insert in a subtree:
        if self.values.len() >= self.max_values && self.max_depth > 0 {
            use Quadrant::*;
            match self.fits(&v) {
                // If v doesn't fit any quadrant, it will stay in the parent node
                None => self.values.push_back(v),
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
            self.values.push_back(v);
        }
    }

    fn split(&mut self) {
        if self.children.is_empty() && self.max_depth > 0 {
            // Spawning the children
            use Quadrant::*;
            self.children.push(QuadTree::<T>::new_child(self, TopLeft));
            self.children.push(QuadTree::<T>::new_child(self, TopRight));
            self.children
                .push(QuadTree::<T>::new_child(self, BottomLeft));
            self.children
                .push(QuadTree::<T>::new_child(self, BottomRight));

            // This node should not accept more value now that it is splitted
            self.max_values = 0;

            // We dispatch it's actual values
            let mut vals = LinkedList::<T>::default();

            while let Some(v) = self.values.pop_back() {
                vals.push_back(v);
            }

            while let Some(v) = vals.pop_back() {
                self.insert(v);
            }
        }
    }
}

impl<T: Collidable> Default for QuadTree<T> {
    fn default() -> Self {
        QuadTree::<T>::new(2, 4, 0, 0, 256, 256)
    }
}
