use std::borrow::{Borrow, BorrowMut};

use crate::math::{coords, iterator, Indexable3D, Indexable3DMut};

pub fn decay_velocity<DST, SRC>(dst: &mut DST, src: &SRC, coefficient: f32)
where
    DST: Indexable3DMut<Inner = f32>,
    SRC: Indexable3D<Inner = f32>,
{
    for c in iterator::iterate(src.size()) {
        *dst.element_mut(c).borrow_mut() = *src.element(c).borrow() * coefficient;
    }
}

// TODO: accept pack of preasures to calculate correct sum
pub fn pressuarize<VEL, PR>(wx: &mut VEL, wy: &mut VEL, wz: &mut VEL, pr: &PR, force: f32)
where
    VEL: Indexable3DMut<Inner = f32>,
    PR: Indexable3D<Inner = f32>,
{
    let size = pr.size();
    for c in iterator::iterate(size - coords::ONES) {
        let src_press = *pr.element(c).borrow();
        let force_x = *pr.element(c + coords::X_FORW).borrow() - src_press;
        let force_y = *pr.element(c + coords::Y_FORW).borrow() - src_press;
        let force_z = *pr.element(c + coords::Z_FORW).borrow() - src_press;

        *wx.element_mut(c + coords::ZEROS).borrow_mut() += force * force_x;
        *wx.element_mut(c + coords::X_FORW).borrow_mut() -= force * force_x;
        *wy.element_mut(c + coords::ZEROS).borrow_mut() += force * force_y;
        *wy.element_mut(c + coords::Y_FORW).borrow_mut() -= force * force_y;
        *wz.element_mut(c + coords::ZEROS).borrow_mut() += force * force_z;
        *wz.element_mut(c + coords::Z_FORW).borrow_mut() -= force * force_z;
    }
}

pub fn generate_vortexes<VORT, VEL>(vorticies: &mut VORT, rx: &VEL, ry: &VEL, rz: &VEL)
where
    VORT: Indexable3DMut<Inner = f32>,
    VEL: Indexable3D<Inner = f32>,
{
    let size = vorticies.size();
    let curl = |c| {
        let x = *rx.element(c + coords::Y_FORW).borrow() - *rx.element(c + coords::Y_BACK).borrow();
        let y = *ry.element(c + coords::X_FORW).borrow() - *ry.element(c + coords::X_BACK).borrow();
        let z = *rz.element(c + coords::Z_FORW).borrow() - *rz.element(c + coords::Z_BACK).borrow();
        (x - y - z) * 0.5
    };
    for c in iterator::iterate_range(coords::ONES.into(), size - coords::ONES) {
        *vorticies.element_mut(c).borrow_mut() = curl(c).abs();
    }
}

pub fn apply_vortex<VEL, VORT>(
    wx: &mut VEL,
    wy: &mut VEL,
    wz: &mut VEL,
    vorticies: &VORT,
    force: f32,
) where
    VEL: Indexable3DMut<Inner = f32>,
    VORT: Indexable3D<Inner = f32>,
{
    let size = vorticies.size();
    for c in iterator::iterate_range(coords::ONES.into(), size - coords::ONES) {
        let lr = *vorticies.element(c + coords::X_FORW).borrow()
            - *vorticies.element(c + coords::X_BACK).borrow();
        let ud = *vorticies.element(c + coords::Y_FORW).borrow()
            - *vorticies.element(c + coords::Y_BACK).borrow();
        let bf = *vorticies.element(c + coords::Z_FORW).borrow()
            - *vorticies.element(c + coords::Z_BACK).borrow();
        let length = (lr * lr + ud * ud + bf * bf).sqrt();
        if length > f32::EPSILON {
            let magnitude = *vorticies.element(c).borrow() * force / length;

            *wx.element_mut(c).borrow_mut() -= ud * magnitude;
            *wy.element_mut(c).borrow_mut() += lr * magnitude;
            *wz.element_mut(c).borrow_mut() += bf * magnitude;
        }
    }
}
