use crate::{
    domain::fluid::FlowFlags,
    math::{self, Array3D, Coords, Indexable3D},
};

pub fn diffusion_step(
    dst: &mut Array3D<f32>,
    src: &Array3D<f32>,
    blockage: &Array3D<FlowFlags>,
    force: f32,
) {
    for c in math::iterate(dst.size()) {
        let blk = *blockage.element(c);
        diffusion_transfer(dst, src, blk, c, force);
    }
}

fn diffusion_transfer(
    dst: &mut Array3D<f32>,
    src: &Array3D<f32>,
    blk: FlowFlags,
    coords: Coords,
    force: f32,
) {
    let size = dst.size();
    let add = |x: usize, s: usize| {
        let res = x.checked_add(1);
        if res.unwrap_or(s) < s {
            res
        } else {
            None
        }
    };
    let dir_map = [
        (
            FlowFlags::X_FORW,
            (add(coords.0, size.0), Some(coords.1), Some(coords.2)),
        ),
        (
            FlowFlags::X_BACK,
            (coords.0.checked_sub(1), Some(coords.1), Some(coords.2)),
        ),
        (
            FlowFlags::Y_FORW,
            (Some(coords.0), add(coords.1, size.1), Some(coords.2)),
        ),
        (
            FlowFlags::Y_BACK,
            (Some(coords.0), coords.1.checked_sub(1), Some(coords.2)),
        ),
        (
            FlowFlags::Z_FORW,
            (Some(coords.0), Some(coords.1), add(coords.2, size.2)),
        ),
        (
            FlowFlags::Z_BACK,
            (Some(coords.0), Some(coords.1), coords.2.checked_sub(1)),
        ),
    ];
    let vals: Vec<_> = dir_map
        .iter()
        .filter_map(|(dir, nc)| {
            if blk.contains(*dir) {
                return None;
            }
            if let (Some(nx), Some(ny), Some(nz)) = *nc {
                return Some(*src.element((nx, ny, nz).into()));
            }
            None
        })
        .collect();
    let (sum, count) = vals
        .iter()
        .fold((0.0, 0.0), |acc, v| (acc.0 + v, acc.1 + 1.0));
    let val = *src.element(coords);
    *dst.element_mut(coords) = val + force * (sum - count * val);
}
