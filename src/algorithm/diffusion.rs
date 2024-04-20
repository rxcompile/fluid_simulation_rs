use rayon::prelude::*;

use crate::{
    data::flow::FlowFlags,
    math::{iterator, Coords, Slice3D, Slice3DMut},
    Sized3D,
};

pub fn diffusion_step<DST, SRC, BLK>(dst: &mut DST, src: &SRC, blockage: &BLK, force: f32)
where
    DST: for<'a> Slice3DMut<Output<'a> = &'a mut f32> + Sized3D,
    SRC: for<'a> Slice3D<Output<'a> = &'a f32> + Sized3D + std::marker::Sync,
    BLK: for<'a> Slice3D<Output<'a> = &'a FlowFlags> + std::marker::Sync,
{
    for c in iterator::iterate(dst.size()) {
        let blk = *blockage.slice(&c);
        let transfer_amount = transfer_amount(src, blk, &c, force);
        *dst.slice_mut(&c) = transfer_amount;
    }
}

fn add(x: usize, s: usize) -> Option<usize> {
    let res = x.checked_add(1);
    if res.unwrap_or(s) < s {
        res
    } else {
        None
    }
}

fn sub(x: usize, _: usize) -> Option<usize> {
    x.checked_sub(1)
}

fn transfer_amount<'a, SRC>(src: &'a SRC, blk: FlowFlags, item_pos: &Coords, force: f32) -> f32
where
    SRC: Slice3D<Output<'a> = &'a f32> + Sized3D + std::marker::Sync,
{
    let size = src.size();

    let dir_map = [
        (
            FlowFlags::X_FORW,
            (add(item_pos.0, size.0), Some(item_pos.1), Some(item_pos.2)),
        ),
        (
            FlowFlags::X_BACK,
            (sub(item_pos.0, size.0), Some(item_pos.1), Some(item_pos.2)),
        ),
        (
            FlowFlags::Y_FORW,
            (Some(item_pos.0), add(item_pos.1, size.1), Some(item_pos.2)),
        ),
        (
            FlowFlags::Y_BACK,
            (Some(item_pos.0), sub(item_pos.1, size.1), Some(item_pos.2)),
        ),
        (
            FlowFlags::Z_FORW,
            (Some(item_pos.0), Some(item_pos.1), add(item_pos.2, size.2)),
        ),
        (
            FlowFlags::Z_BACK,
            (Some(item_pos.0), Some(item_pos.1), sub(item_pos.2, size.2)),
        ),
    ];
    let (sum, count) = dir_map
        .par_iter()
        .filter_map(|(dir, nc)| {
            if blk.contains(*dir) {
                return None;
            }
            // wrapped to Option for convenient boundary check
            if let (Some(nx), Some(ny), Some(nz)) = *nc {
                let c = (nx, ny, nz).into();
                return Some(src.slice(&c));
            }
            None
        })
        .fold(|| (0.0, 0.0), |acc, v| (acc.0 + v, acc.1 + 1.0))
        .reduce(|| (0.0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1));
    let val = *src.slice(item_pos);
    val + force * (sum - count as f32 * val)
}
