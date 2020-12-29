use crate::{
    data::flow::FlowFlags,
    math::{iterator, Coords, Indexable3D, Indexable3DMut},
};

pub fn diffusion_step<DST, SRC, BLK>(dst: &mut DST, src: &SRC, blockage: &BLK, force: f32)
where
    DST: for<'a> Indexable3DMut<'a, Output = &'a mut f32>,
    SRC: for<'a> Indexable3D<'a, Output = &'a f32>,
    BLK: for<'a> Indexable3D<'a, Output = &'a FlowFlags>,
{
    for c in iterator::iterate(src.size()) {
        diffusion_transfer(dst, src, blockage.element(c), c, force);
    }
}

fn diffusion_transfer<DST, SRC>(
    dst: &mut DST,
    src: &SRC,
    blk: &FlowFlags,
    coords: Coords,
    force: f32,
) where
    DST: for<'a> Indexable3DMut<'a, Output = &'a mut f32>,
    SRC: for<'a> Indexable3D<'a, Output = &'a f32>,
{
    let size = src.size();
    let add = |x: usize, s: usize| {
        let res = x.checked_add(1);
        if res.unwrap_or(s) < s {
            res
        } else {
            None
        }
    };
    let sub = |x: usize, _: usize| x.checked_sub(1);
    let dir_map = [
        (
            FlowFlags::X_FORW,
            (add(coords.0, size.0), Some(coords.1), Some(coords.2)),
        ),
        (
            FlowFlags::X_BACK,
            (sub(coords.0, size.0), Some(coords.1), Some(coords.2)),
        ),
        (
            FlowFlags::Y_FORW,
            (Some(coords.0), add(coords.1, size.1), Some(coords.2)),
        ),
        (
            FlowFlags::Y_BACK,
            (Some(coords.0), sub(coords.1, size.1), Some(coords.2)),
        ),
        (
            FlowFlags::Z_FORW,
            (Some(coords.0), Some(coords.1), add(coords.2, size.2)),
        ),
        (
            FlowFlags::Z_BACK,
            (Some(coords.0), Some(coords.1), sub(coords.2, size.2)),
        ),
    ];
    let vals: Vec<_> = dir_map
        .iter()
        .filter_map(|(dir, nc)| {
            if blk.contains(*dir) {
                return None;
            }
            if let (Some(nx), Some(ny), Some(nz)) = *nc {
                return Some(src.element((nx, ny, nz).into()).clone());
            }
            None
        })
        .collect();
    let (sum, count) = vals
        .iter()
        .fold((0.0, 0.0), |acc, v| (acc.0 + v, acc.1 + 1.0));
    let val = src.element(coords).clone();
    *dst.element_mut(coords) = val + force * (sum - count * val);
}
