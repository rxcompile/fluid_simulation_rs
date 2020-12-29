pub mod coords;
pub mod iterator;
pub mod sized_array;
pub mod swapchain;

use std::borrow::{Borrow, BorrowMut};

pub use coords::{Coords, CoordsDiff};
pub use sized_array::SizedArray3D;

pub fn index(x: usize, y: usize, z: usize, len_x: usize, len_y: usize, len_z: usize) -> usize {
    assert!(z < len_z);
    x + len_x * (y + len_y * z)
}

pub trait Sizeable3D {
    fn size(&self) -> Coords;
}

pub trait Indexable3D: Sizeable3D {
    type Output<'a>: Borrow<Self::Inner>;
    type Inner;
    fn element<'a>(&'a self, c: Coords) -> Self::Output<'a>;
}

pub trait Indexable3DMut: Sizeable3D {
    type Output<'a>: BorrowMut<Self::Inner>;
    type Inner;
    fn element_mut<'a>(&'a mut self, c: Coords) -> Self::Output<'a>;
}

pub trait Fillable<T> {
    fn fill(&mut self, default: T);
}
