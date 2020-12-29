#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Coords(pub usize, pub usize, pub usize);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct CoordsDiff(pub isize, pub isize, pub isize);

pub const ONES: CoordsDiff = CoordsDiff(1, 1, 1);
pub const ZEROS: CoordsDiff = CoordsDiff(0, 0, 0);
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
