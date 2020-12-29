use crate::{
    math::{Coords, Indexable3D, Indexable3DMut},
    support_utils::construct_from,
};
use std::mem::MaybeUninit;

use super::Sizeable3D;

pub trait Swapable {
    fn swap_buffers(&mut self);
    fn copy_from_read(&mut self);
}

#[derive(Clone, Debug)]
pub struct Swapchain<T, const SIZE: usize> {
    pub data: [T; SIZE],
    pub current: usize,
}

impl<T, const SIZE: usize> Default for Swapchain<T, SIZE>
where
    T: Default,
{
    fn default() -> Self {
        let mut tmp = MaybeUninit::<T>::uninit_array::<SIZE>();
        for v in tmp.iter_mut() {
            v.write(Default::default());
        }
        Self {
            data: unsafe { std::mem::transmute_copy(&tmp) },
            current: 0,
        }
    }
}

impl<T, const SIZE: usize> Swapchain<T, SIZE> {
    pub fn read(&self) -> &T {
        &self.data[self.current]
    }

    pub fn write(&mut self) -> &mut T {
        &mut self.data[(self.current + 1) % SIZE]
    }

    pub fn rw_pair<'a>(&'a mut self) -> (&'a T, &'a mut T) {
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

impl<T, const SIZE: usize> Sizeable3D for Swapchain<T, SIZE>
where
    T: Sizeable3D,
{
    fn size(&self) -> Coords {
        self.read().size()
    }
}

impl<'a, T, const SIZE: usize> Indexable3D<'a> for Swapchain<T, SIZE>
where
    T: Indexable3D<'a>,
{
    type Output = T::Output;

    fn element(&'a self, c: Coords) -> Self::Output {
        self.read().element(c)
    }
}

impl<'a, T, const SIZE: usize> Indexable3DMut<'a> for Swapchain<T, SIZE>
where
    T: Indexable3DMut<'a>,
{
    type Output = T::Output;

    fn element_mut(&'a mut self, c: Coords) -> Self::Output {
        self.write().element_mut(c)
    }
}

impl<T, const SIZE: usize> Swapable for Swapchain<T, SIZE>
where
    T: std::clone::Clone,
{
    fn swap_buffers(&mut self) {
        self.current = (self.current + 1) % SIZE
    }

    fn copy_from_read(&mut self) {
        let (r, w) = self.rw_pair();
        *w = r.clone();
    }
}

pub type SwapchainPack<T, const PACK_SIZE: usize, const SW_SIZE: usize> =
    [Swapchain<T, SW_SIZE>; PACK_SIZE];

impl<T, const PACK_SIZE: usize, const SW_SIZE: usize> Swapable
    for SwapchainPack<T, PACK_SIZE, SW_SIZE>
where
    T: std::clone::Clone,
{
    fn swap_buffers(&mut self) {
        self.iter_mut().for_each(|x| x.swap_buffers())
    }

    fn copy_from_read(&mut self) {
        self.iter_mut().for_each(|x| x.copy_from_read())
    }
}

impl<'a, T, const PACK_SIZE: usize, const SW_SIZE: usize> Sizeable3D
    for SwapchainPack<T, PACK_SIZE, SW_SIZE>
where
    T: Sizeable3D,
{
    fn size(&self) -> Coords {
        self[0].size()
    }
}

impl<'a, T, const PACK_SIZE: usize, const SW_SIZE: usize> Indexable3D<'a>
    for SwapchainPack<T, PACK_SIZE, SW_SIZE>
where
    T: Indexable3D<'a>,
{
    type Output = [T::Output; PACK_SIZE];

    fn element(&'a self, c: Coords) -> Self::Output {
        construct_from(self.iter().map(|i| i.element(c)))
    }
}

impl<'a, T, const PACK_SIZE: usize, const SW_SIZE: usize> Indexable3DMut<'a>
    for SwapchainPack<T, PACK_SIZE, SW_SIZE>
where
    T: Indexable3DMut<'a>,
{
    type Output = [T::Output; PACK_SIZE];

    fn element_mut(&'a mut self, c: Coords) -> Self::Output {
        construct_from(self.iter_mut().map(|i| i.element_mut(c)))
    }
}
