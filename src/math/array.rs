use super::{
    swapchain::{Swapchain, SwapchainPack},
    Fillable, Indexable3D, Slice3D, Slice3DMut,
};
use crate::{support_utils, Coords};

#[derive(Clone, Debug)]
pub struct Array3D<T> {
    data: Vec<T>,
    size: Coords,
}

// impl<T> SizeCreatable for Array3D<T>
// where
//     T: Default + Clone,
// {
//     fn new(size: Coords) -> Self {
//         let linear = size.0 * size.1 * size.2;
//         Array3D {
//             data: vec![Default::default(); linear],
//             size,
//         }
//     }
// }

impl<T> Fillable<T> for Array3D<T>
where
    T: std::clone::Clone + std::default::Default,
{
    fn fill(&mut self, default: T) {
        self.data.fill(default);
    }
}

// impl<T> Indexable3D<T> for Array3D<T> {
//     fn element(&self, c: Coords) -> &T {
//         &self.data[index_pack(self.size, c)]
//     }

//     fn element_mut(&mut self, c: Coords) -> &mut T {
//         &mut self.data[index_pack(self.size, c)]
//     }

//     fn size(&self) -> Coords {
//         self.size
//     }
// }

impl<T> std::ops::Add for Array3D<T>
where
    T: std::ops::AddAssign + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(self.size, other.size);
        let mut data = self.data;
        for (r, b) in data.iter_mut().zip(&other.data) {
            *r += *b;
        }
        Array3D {
            data,
            size: self.size,
        }
    }
}

impl<T> std::ops::Sub for Array3D<T>
where
    T: std::ops::SubAssign + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(self.size, other.size);
        let mut data = self.data;
        for (r, b) in data.iter_mut().zip(&other.data) {
            *r -= *b;
        }
        Array3D {
            data,
            size: self.size,
        }
    }
}

impl<T> std::ops::Mul for Array3D<T>
where
    T: std::ops::MulAssign + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        assert_eq!(self.size, other.size);
        let mut data = self.data;
        for (r, b) in data.iter_mut().zip(&other.data) {
            *r *= *b;
        }
        Array3D {
            data,
            size: self.size,
        }
    }
}

impl<T> std::ops::Div for Array3D<T>
where
    T: std::ops::DivAssign + Copy,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        assert_eq!(self.size, other.size);
        let mut data = self.data;
        for (r, b) in data.iter_mut().zip(&other.data) {
            *r /= *b;
        }
        Array3D {
            data,
            size: self.size,
        }
    }
}

impl<T> std::ops::Add<T> for Array3D<T>
where
    T: std::ops::AddAssign + Copy,
{
    type Output = Self;

    fn add(self, other: T) -> Self {
        let mut data = self.data;
        for e in data.iter_mut() {
            *e += other;
        }
        Array3D {
            data,
            size: self.size,
        }
    }
}

impl<T> std::ops::Sub<T> for Array3D<T>
where
    T: std::ops::SubAssign + Copy,
{
    type Output = Self;

    fn sub(self, other: T) -> Self {
        let mut data = self.data;
        for e in data.iter_mut() {
            *e -= other;
        }
        Array3D {
            data,
            size: self.size,
        }
    }
}

impl<T> std::ops::Mul<T> for Array3D<T>
where
    T: std::ops::MulAssign + Copy,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        let mut data = self.data;
        for e in data.iter_mut() {
            *e *= other;
        }
        Array3D {
            data,
            size: self.size,
        }
    }
}

impl<T> std::ops::Div<T> for Array3D<T>
where
    T: std::ops::DivAssign + Copy,
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        let mut data = self.data;
        for e in data.iter_mut() {
            *e /= other;
        }
        Array3D {
            data,
            size: self.size,
        }
    }
}

// impl<T, const SIZE: usize> SizeCreatable for Swapchain<Array3D<T>, SIZE>
// where
//     T: std::clone::Clone + std::default::Default + std::fmt::Debug,
// {
//     fn new(size: Coords) -> Self {
//         Self {
//             data: support_utils::construct_fn(|| Array3D::new(size)),
//             current: 0,
//         }
//     }
// }

// impl<T, const PACK_SIZE: usize, const SW_SIZE: usize> SizeCreatable
//     for SwapchainPack<Array3D<T>, PACK_SIZE, SW_SIZE>
// where
//     T: std::clone::Clone + std::default::Default + std::fmt::Debug,
// {
//     fn new(size: Coords) -> Self {
//         support_utils::construct_fn(|| Swapchain::new(size))
//     }
// }

impl<T> Slice3D for Array3D<T> {
    type Output<'a> = &'a T where Self: 'a;
    fn slice<'a>(&'a self, c: &Coords) -> Self::Output<'a> {
        &self[c]
    }
}

impl<T> Slice3DMut for Array3D<T> {
    type Output<'a> = &'a mut T where Self: 'a;
    fn slice_mut<'a>(&'a mut self, c: &Coords) -> Self::Output<'a> {
        &mut self[c]
    }
}
