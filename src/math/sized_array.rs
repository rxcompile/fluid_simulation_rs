use super::{
    index, Fillable, FlatIndex, Sized3D, Slice3D, Slice3DMut,
};
use crate::Coords;

#[derive(Clone)]
pub struct SizedArray3D<T, const X: usize, const Y: usize, const Z: usize>(Vec<T>);

impl<T, const X: usize, const Y: usize, const Z: usize> Default for SizedArray3D<T, X, Y, Z>
where
    T: Default + Clone,
{
    fn default() -> Self {
        Self(vec![Default::default(); X * Y * Z])
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> Sized3D for SizedArray3D<T, X, Y, Z> {
    fn size(&self) -> Coords {
        (X, Y, Z).into()
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> FlatIndex for SizedArray3D<T, X, Y, Z> {
    fn to_index(&self, c: &Coords) -> usize {
        index(c.0, c.1, c.2, X, Y, Z)
    }

    fn from_index(&self, _: usize) -> Coords {
        todo!()
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> std::ops::Index<&Coords>
    for SizedArray3D<T, X, Y, Z>
{
    type Output = T;

    fn index(&self, index: &Coords) -> &Self::Output {
        &self.0[self.to_index(&index)]
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> std::ops::IndexMut<&Coords>
    for SizedArray3D<T, X, Y, Z>
{
    fn index_mut(&mut self, index: &Coords) -> &mut Self::Output {
        let index = self.to_index(&index);
        &mut self.0[index]
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> Fillable<T> for SizedArray3D<T, X, Y, Z>
where
    T: Clone,
{
    fn fill(&mut self, default: T) {
        self.0.fill(default)
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> std::ops::Add for SizedArray3D<T, X, Y, Z>
where
    T: std::ops::AddAssign + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut data = self.0;
        for (r, b) in data.iter_mut().zip(&other.0) {
            *r += *b;
        }
        Self(data)
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> std::ops::Sub for SizedArray3D<T, X, Y, Z>
where
    T: std::ops::SubAssign + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut data = self.0;
        for (r, b) in data.iter_mut().zip(&other.0) {
            *r -= *b;
        }
        Self(data)
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> std::ops::Mul for SizedArray3D<T, X, Y, Z>
where
    T: std::ops::MulAssign + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut data = self.0;
        for (r, b) in data.iter_mut().zip(&other.0) {
            *r *= *b;
        }
        Self(data)
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> std::ops::Div for SizedArray3D<T, X, Y, Z>
where
    T: std::ops::DivAssign + Copy,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut data = self.0;
        for (r, b) in data.iter_mut().zip(&other.0) {
            *r /= *b;
        }
        Self(data)
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> std::ops::Add<T>
    for SizedArray3D<T, X, Y, Z>
where
    T: std::ops::AddAssign + Copy,
{
    type Output = Self;

    fn add(self, other: T) -> Self {
        let mut data = self.0;
        for e in data.iter_mut() {
            *e += other;
        }
        Self(data)
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> std::ops::Sub<T>
    for SizedArray3D<T, X, Y, Z>
where
    T: std::ops::SubAssign + Copy,
{
    type Output = Self;

    fn sub(self, other: T) -> Self {
        let mut data = self.0;
        for e in data.iter_mut() {
            *e -= other;
        }
        Self(data)
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> std::ops::Mul<T>
    for SizedArray3D<T, X, Y, Z>
where
    T: std::ops::MulAssign + Copy,
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let mut data = self.0;
        for e in data.iter_mut() {
            *e *= other;
        }
        Self(data)
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> std::ops::Div<T>
    for SizedArray3D<T, X, Y, Z>
where
    T: std::ops::DivAssign + Copy,
{
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        let mut data = self.0;
        for e in data.iter_mut() {
            *e /= other;
        }
        Self(data)
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> Slice3D for SizedArray3D<T, X, Y, Z> {
    type Output<'a> = &'a T where Self: 'a;
    fn slice<'a>(&'a self, c: &Coords) -> Self::Output<'a> {
        &self[c]
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> Slice3DMut for SizedArray3D<T, X, Y, Z> {
    type Output<'a> = &'a mut T where Self: 'a;
    fn slice_mut<'a>(&'a mut self, c: &Coords) -> Self::Output<'a> {
        &mut self[c]
    }
}

