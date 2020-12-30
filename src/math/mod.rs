pub mod coords;
pub mod iterator;
pub mod sized_array;
pub mod swapchain;

pub use coords::{Coords, CoordsDiff};
pub use sized_array::SizedArray3D;

pub fn index(x: usize, y: usize, z: usize, len_x: usize, len_y: usize, len_z: usize) -> usize {
    assert!(z < len_z);
    x + len_x * (y + len_y * z)
}

pub trait Sizeable3D {
    fn size(&self) -> Coords;
}

pub trait Indexable3D<'a>: Sizeable3D {
    type Output: 'a;
    fn element(&'a self, c: Coords) -> Self::Output;
}

pub trait Indexable3DMut<'a>: Indexable3D<'a> {
    type OutputMut: 'a;
    fn element_mut(&'a mut self, c: Coords) -> Self::OutputMut;
}

pub trait Fillable<T> {
    fn fill(&mut self, default: T);
}
