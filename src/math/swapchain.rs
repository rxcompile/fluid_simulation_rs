use crate::{math::Coords, support_utils};

use super::{Sized3D, Slice3D, Slice3DMut};

pub trait Swapable {
    fn swap_buffers(&mut self);
}

#[derive(Clone, Debug)]
pub struct Swapchain<T, const SIZE: usize> {
    pub data: [T; SIZE],
    pub current_producer: usize,
    pub current_consumer: usize,
}

impl<T, const SIZE: usize> Default for Swapchain<T, SIZE>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            data: support_utils::construct_default(),
            current_consumer: 0,
            current_producer: 1,
        }
    }
}

impl<T, const SIZE: usize> Swapchain<T, SIZE> {
    pub fn consumer(&self) -> &T {
        &self.data[self.current_consumer]
    }

    pub fn producer(&mut self) -> &mut T {
        &mut self.data[self.current_producer]
    }

    pub fn rw_pair<'a>(&'a mut self) -> (&'a T, &'a mut T) {
        let w_idx = self.current_producer;
        let r_idx = self.current_consumer;
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

impl<T, const SIZE: usize> Sized3D for Swapchain<T, SIZE>
where
    T: Sized3D,
{
    fn size(&self) -> Coords {
        self.consumer().size()
    }
}

impl<T, const SIZE: usize> Slice3D for Swapchain<T, SIZE>
where
    T: Slice3D,
{
    type Output<'a> = T::Output<'a> where Self: 'a;

    fn slice<'a>(&'a self, c: &Coords) -> Self::Output<'a> {
        self.consumer().slice(&c)
    }
}

impl<T, const SIZE: usize> Slice3DMut for Swapchain<T, SIZE>
where
    T: Slice3DMut,
{
    type Output<'a> = T::Output<'a> where Self: 'a;

    fn slice_mut<'a>(&'a mut self, c: &Coords) -> Self::Output<'a> {
        self.producer().slice_mut(&c)
    }
}

impl<T, const SIZE: usize> Swapable for Swapchain<T, SIZE>
where
    T: std::clone::Clone,
{
    fn swap_buffers(&mut self) {
        // Apply write buffer to read
        let (r, w) = self.rw_pair();
        *w = r.clone();
        // Rotate indexes
        self.current_producer = (self.current_producer + 1) % SIZE;
        self.current_consumer = (self.current_consumer + 1) % SIZE;
    }
}

pub struct SwapchainPack<T, const PACK_SIZE: usize, const SW_SIZE: usize> {
    data: [Swapchain<T, SW_SIZE>; PACK_SIZE],
}

impl<T, const PACK_SIZE: usize, const SW_SIZE: usize> SwapchainPack<T, PACK_SIZE, SW_SIZE> {
    pub fn rw_pairs<'a>(&'a mut self) -> [(&'a T, &'a mut T); PACK_SIZE] {
        self.data.each_mut().map(|p| p.rw_pair())
    }
}

impl<T, const PACK_SIZE: usize, const SW_SIZE: usize> Default
    for SwapchainPack<T, PACK_SIZE, SW_SIZE>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            data: support_utils::construct_default(),
        }
    }
}

impl<T, const PACK_SIZE: usize, const SW_SIZE: usize> Slice3D
    for SwapchainPack<T, PACK_SIZE, SW_SIZE>
where
    T: Slice3D,
{
    type Output<'a> = [T::Output<'a>; PACK_SIZE] where Self: 'a;

    fn slice<'a>(&'a self, c: &Coords) -> Self::Output<'a> {
        self.data.each_ref().map(|x| x.slice(c))
    }
}

impl<T, const PACK_SIZE: usize, const SW_SIZE: usize> Slice3DMut
    for SwapchainPack<T, PACK_SIZE, SW_SIZE>
where
    T: Slice3DMut,
{
    type Output<'a> = [T::Output<'a>; PACK_SIZE] where Self: 'a;

    fn slice_mut<'a>(&'a mut self, c: &Coords) -> Self::Output<'a> {
        self.data.each_mut().map(|x| x.slice_mut(c))
    }
}

impl<T, const PACK_SIZE: usize, const SW_SIZE: usize> Swapable
    for SwapchainPack<T, PACK_SIZE, SW_SIZE>
where
    T: std::clone::Clone,
{
    fn swap_buffers(&mut self) {
        self.data.iter_mut().for_each(|x| x.swap_buffers())
    }
}

impl<'a, T, const PACK_SIZE: usize, const SW_SIZE: usize> Sized3D
    for SwapchainPack<T, PACK_SIZE, SW_SIZE>
where
    T: Sized3D,
{
    fn size(&self) -> Coords {
        // any will do
        self.data[0].size()
    }
}
