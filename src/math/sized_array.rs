use super::{index, Fillable, Indexable3D, Indexable3DMut, Sizeable3D};
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

impl<T, const X: usize, const Y: usize, const Z: usize> Sizeable3D for SizedArray3D<T, X, Y, Z> {
    fn size(&self) -> Coords {
        (X, Y, Z).into()
    }
}

impl<'a, T, const X: usize, const Y: usize, const Z: usize> Indexable3D<'a> for SizedArray3D<T, X, Y, Z>
where
    T: 'static,
{
    type Output = &'a T;

    fn element(&'a self, c: Coords) -> Self::Output {
        &self.0[index(c.0, c.1, c.2, X, Y, Z)]
    }
}

impl<'a, T, const X: usize, const Y: usize, const Z: usize> Indexable3DMut<'a> for SizedArray3D<T, X, Y, Z>
where
    T: 'static,
{
    type OutputMut = &'a mut T;

    fn element_mut(&'a mut self, c: Coords) -> Self::OutputMut {
        &mut self.0[index(c.0, c.1, c.2, X, Y, Z)]
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
