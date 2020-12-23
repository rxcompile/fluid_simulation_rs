use std::convert::TryInto;

use crate::math::{Array3D, Coords, Indexable3D, SizeCreatable};

pub trait Swapable {
    fn swap(&mut self);
    fn copy_from_read(&mut self);
}

#[derive(Clone, Debug)]
pub struct Swapchain<T, const SIZE: usize> {
    pub data: [T; SIZE],
    pub current: usize,
}

impl<T, const SIZE: usize> Swapchain<T, SIZE> {
    pub fn read(&self) -> &T {
        &self.data[self.current]
    }

    pub fn write(&mut self) -> &mut T {
        &mut self.data[(self.current + 1) % SIZE]
    }

    pub fn rw_pair(&mut self) -> (&T, &mut T) {
        let w_idx = (self.current + 1) % SIZE;
        let r_idx = self.current;
        assert_ne!(w_idx, r_idx);
        if w_idx > r_idx {
            let (rs, ws) = self.data.split_at_mut(w_idx);
            return (rs.last().unwrap(), ws.first_mut().unwrap());
        } else {
            let (ws, rs) = self.data.split_at_mut(r_idx);
            return (rs.first().unwrap(), ws.last_mut().unwrap());
        }
    }
}

impl<T, const SIZE: usize> Indexable3D<T> for Swapchain<Array3D<T>, SIZE> {
    fn element(&self, c: Coords) -> &T {
        self.read().element(c)
    }

    fn element_mut(&mut self, c: Coords) -> &mut T {
        self.write().element_mut(c)
    }

    fn size(&self) -> Coords {
        self.read().size()
    }
}

impl<T, const SIZE: usize> Swapable for Swapchain<T, SIZE>
where
    T: std::clone::Clone,
{
    fn swap(&mut self) {
        self.current = (self.current + 1) % SIZE
    }

    fn copy_from_read(&mut self) {
        let (r, w) = self.rw_pair();
        *w = r.clone();
    }
}

impl<T, const SIZE: usize> SizeCreatable for Swapchain<Array3D<T>, SIZE>
where
    T: std::clone::Clone + std::default::Default + std::fmt::Debug,
{
    fn new(size: Coords) -> Self {
        // TODO: this heap allocation is bs, but cant do much about it without unsafe
        Self {
            data: vec![Array3D::new(size); SIZE].try_into().unwrap(),
            current: 0,
        }
    }
}

pub type SwapchainPack<T, const PACK_SIZE: usize, const SW_SIZE: usize> =
    [Swapchain<T, SW_SIZE>; PACK_SIZE];

impl<T, const PACK_SIZE: usize, const SW_SIZE: usize> Swapable
    for SwapchainPack<Array3D<T>, PACK_SIZE, SW_SIZE>
where
    T: std::clone::Clone,
{
    fn swap(&mut self) {
        self.iter_mut().for_each(|x| x.swap())
    }

    fn copy_from_read(&mut self) {
        self.iter_mut().for_each(|x| x.copy_from_read())
    }
}

impl<T, const PACK_SIZE: usize, const SW_SIZE: usize> SizeCreatable
    for SwapchainPack<Array3D<T>, PACK_SIZE, SW_SIZE>
where
    T: std::clone::Clone + std::default::Default + std::fmt::Debug,
{
    fn new(size: Coords) -> Self {
        // TODO: this heap allocation is bs, but cant do much about it without unsafe
        vec![Swapchain::new(size); PACK_SIZE].try_into().unwrap()
    }
}
