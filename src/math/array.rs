use super::{
    swapchain::{Swapchain, SwapchainPack},
    Slice3D, Slice3DMut,
};
use crate::{support_utils, Coords};

#[derive(Clone, Debug)]
pub struct Array3D<T> {
    data: Vec<T>,
    size: Coords,
}

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
