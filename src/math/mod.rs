pub mod coords;
pub mod iterator;
pub mod sized_array;
pub mod swapchain;

pub use coords::{Coords, CoordsDiff};
pub use sized_array::SizedArray3D;

pub const fn index(x: usize, y: usize, z: usize, len_x: usize, len_y: usize, len_z: usize) -> usize {
    assert!(z < len_z);
    x + len_x * (y + len_y * z)
}

pub trait FlatIndex {
    fn to_index(&self, c: &Coords) -> usize;
    fn from_index(&self, i: usize) -> Coords;
}

pub trait Sized3D {
    fn size(&self) -> Coords;
}

impl FlatIndex for dyn Sized3D {
    fn to_index(&self, c: &Coords) -> usize {
        let Coords(lx, ly, lz) = self.size();
        index(c.0, c.1, c.2, lx, ly, lz)
    }

    fn from_index(&self, _: usize) -> Coords {
        todo!()
    }
}

pub trait Slice3D {
    type Output<'a> where Self: 'a;
    fn slice<'a, 'b>(&'a self, c: &'b Coords) -> Self::Output<'a>;
}

pub trait Slice3DMut {
    type Output<'a> where Self: 'a;
    fn slice_mut<'a, 'b>(&'a mut self, c: &'b Coords) -> Self::Output<'a>;
}
