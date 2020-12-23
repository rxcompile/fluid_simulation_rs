pub mod swapchain;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Coords(pub usize, pub usize, pub usize);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CoordsDiff(pub isize, pub isize, pub isize);

pub const ONES: CoordsDiff = CoordsDiff(1, 1, 1);
pub const X_FORW: CoordsDiff = CoordsDiff(1, 0, 0);
pub const Y_FORW: CoordsDiff = CoordsDiff(0, 1, 0);
pub const Z_FORW: CoordsDiff = CoordsDiff(0, 0, 1);
pub const X_BACK: CoordsDiff = CoordsDiff(-1, 0, 0);
pub const Y_BACK: CoordsDiff = CoordsDiff(0, -1, 0);
pub const Z_BACK: CoordsDiff = CoordsDiff(0, 0, -1);

impl Default for Coords {
    fn default() -> Self {
        Self(0, 0, 0)
    }
}

impl From<CoordsDiff> for Coords {
    fn from(c: CoordsDiff) -> Self {
        Self(c.0 as usize, c.1 as usize, c.1 as usize)
    }
}

impl From<Coords> for CoordsDiff {
    fn from(c: Coords) -> Self {
        Self(c.0 as isize, c.1 as isize, c.2 as isize)
    }
}

impl From<(usize, usize, usize)> for Coords {
    fn from(c: (usize, usize, usize)) -> Self {
        Self(c.0, c.1, c.2)
    }
}

impl From<(isize, isize, isize)> for CoordsDiff {
    fn from(c: (isize, isize, isize)) -> Self {
        Self(c.0, c.1, c.2)
    }
}

impl Into<(usize, usize, usize)> for Coords {
    fn into(self) -> (usize, usize, usize) {
        (self.0, self.1, self.2)
    }
}

impl std::ops::Add for CoordsDiff {
    type Output = CoordsDiff;

    fn add(self, rhs: CoordsDiff) -> Self::Output {
        CoordsDiff(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Sub for CoordsDiff {
    type Output = CoordsDiff;

    fn sub(self, rhs: Self) -> Self::Output {
        CoordsDiff(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Add<CoordsDiff> for Coords {
    type Output = Coords;

    fn add(self, rhs: CoordsDiff) -> Self::Output {
        let diff: CoordsDiff = self.into();
        (diff + rhs).into()
    }
}

impl std::ops::Sub<CoordsDiff> for Coords {
    type Output = Coords;

    fn sub(self, rhs: CoordsDiff) -> Self::Output {
        let diff: CoordsDiff = self.into();
        (diff - rhs).into()
    }
}

pub fn index(size: Coords, coord: Coords) -> usize {
    coord.0 + size.0 * (coord.1 + size.1 * coord.2)
}

pub trait Indexable3D<T> {
    fn element(&self, c: Coords) -> &T;
    fn element_mut(&mut self, c: Coords) -> &mut T;
    fn size(&self) -> Coords;
}

pub trait SizeCreatable {
    fn new(size: Coords) -> Self;
}

#[derive(Clone, Debug)]
pub struct Array3D<T> {
    data: Vec<T>,
    size: Coords,
}

impl<T> SizeCreatable for Array3D<T>
where
    T: Default + Clone,
{
    fn new(size: Coords) -> Self {
        let linear = size.0 * size.1 * size.2;
        Array3D {
            data: vec![Default::default(); linear],
            size,
        }
    }
}

impl<T: std::clone::Clone + std::default::Default> Array3D<T> {
    pub fn fill(&mut self, value: T) {
        self.data.fill(value);
    }
}

impl<T> Indexable3D<T> for Array3D<T> {
    fn element(&self, c: Coords) -> &T {
        &self.data[index(self.size, c)]
    }

    fn element_mut(&mut self, c: Coords) -> &mut T {
        &mut self.data[index(self.size, c)]
    }

    fn size(&self) -> Coords {
        self.size
    }
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

pub fn iterate(size: Coords) -> impl std::iter::Iterator<Item = Coords> {
    iterate_from(size, Default::default())
}

pub fn iterate_from(size: Coords, first: Coords) -> impl std::iter::Iterator<Item = Coords> {
    let mut iter = first;
    let mut start = true;
    std::iter::from_fn(move || {
        if start {
            start = false;
            return Some(iter);
        }
        iter.0 += 1;
        if iter.0 >= size.0 {
            iter.0 = first.0;
            iter.1 += 1;
            if iter.1 >= size.1 {
                iter.1 = first.1;
                iter.2 += 1;
                if iter.2 >= size.2 {
                    return None;
                }
            }
        }
        Some(iter)
    })
}

#[test]
fn iterator_test() {
    let vec1: Vec<_> = iterate((1, 1, 1).into()).collect();
    assert_eq!(vec1, vec![(0, 0, 0).into()]);
    let vec2: Vec<_> = iterate((1, 1, 2).into()).collect();
    assert_eq!(vec2, vec![(0, 0, 0).into(), (0, 0, 1).into()]);
    let vec3: Vec<_> = iterate((1, 2, 1).into()).collect();
    assert_eq!(vec3, vec![(0, 0, 0).into(), (0, 1, 0).into()]);
    let vec4: Vec<_> = iterate((2, 1, 1).into()).collect();
    assert_eq!(vec4, vec![(0, 0, 0).into(), (1, 0, 0).into()]);
    let vec5: Vec<_> = iterate((3, 3, 3).into()).collect();
    let vec5_t = vec![
        (0,0,0).into(),
        (1,0,0).into(),
        (2,0,0).into(),
        (0,1,0).into(),
        (1,1,0).into(),
        (2,1,0).into(),
        (0,2,0).into(),
        (1,2,0).into(),
        (2,2,0).into(),
        (0,0,1).into(),
        (1,0,1).into(),
        (2,0,1).into(),
        (0,1,1).into(),
        (1,1,1).into(),
        (2,1,1).into(),
        (0,2,1).into(),
        (1,2,1).into(),
        (2,2,1).into(),
        (0,0,2).into(),
        (1,0,2).into(),
        (2,0,2).into(),
        (0,1,2).into(),
        (1,1,2).into(),
        (2,1,2).into(),
        (0,2,2).into(),
        (1,2,2).into(),
        (2,2,2).into(),
        ];
    assert_eq!(vec5, vec5_t);
}
