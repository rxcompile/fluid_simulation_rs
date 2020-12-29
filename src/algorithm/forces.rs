use crate::math::{coords, iterator, Indexable3D, Indexable3DMut};

pub fn decay_velocity<DST, SRC>(dst: &mut DST, src: &SRC, coefficient: f32)
where
    DST: for<'a> Indexable3DMut<'a, Output = &'a mut f32>,
    SRC: for<'a> Indexable3D<'a, Output = &'a f32>,
{
    for c in iterator::iterate(src.size()) {
        *dst.element_mut(c) = *src.element(c) * coefficient;
    }
}

// TODO: accept pack of preasures to calculate correct sum
pub fn pressuarize<VEL, PR>(wx: &mut VEL, wy: &mut VEL, wz: &mut VEL, pr: &PR, force: f32)
where
    VEL: for<'a> Indexable3DMut<'a, Output = &'a mut f32>,
    PR: for<'a> Indexable3D<'a, Output = &'a f32>,
{
    let size = pr.size();
    for c in iterator::iterate(size - coords::ONES) {
        let src_press = *pr.element(c);
        let force_x = *pr.element(c + coords::X_FORW) - src_press;
        let force_y = *pr.element(c + coords::Y_FORW) - src_press;
        let force_z = *pr.element(c + coords::Z_FORW) - src_press;

        *wx.element_mut(c + coords::ZEROS) += force * force_x;
        *wx.element_mut(c + coords::X_FORW) -= force * force_x;
        *wy.element_mut(c + coords::ZEROS) += force * force_y;
        *wy.element_mut(c + coords::Y_FORW) -= force * force_y;
        *wz.element_mut(c + coords::ZEROS) += force * force_z;
        *wz.element_mut(c + coords::Z_FORW) -= force * force_z;
    }
}

pub fn generate_vortexes<VORT, VEL>(vorticies: &mut VORT, rx: &VEL, ry: &VEL, rz: &VEL)
where
    VORT: for<'a> Indexable3DMut<'a, Output = &'a mut f32>,
    VEL: for<'a> Indexable3D<'a, Output = &'a f32>,
{
    let size = vorticies.size();
    let curl = |c| {
        let x = *rx.element(c + coords::Y_FORW) - *rx.element(c + coords::Y_BACK);
        let y = *ry.element(c + coords::X_FORW) - *ry.element(c + coords::X_BACK);
        let z = *rz.element(c + coords::Z_FORW) - *rz.element(c + coords::Z_BACK);
        (x - y - z) * 0.5
    };
    for c in iterator::iterate_range(coords::ONES.into(), size - coords::ONES) {
        *vorticies.element_mut(c) = curl(c).abs();
    }
}

pub fn apply_vortex<VEL, VORT>(
    wx: &mut VEL,
    wy: &mut VEL,
    wz: &mut VEL,
    vorticies: &VORT,
    force: f32,
) where
    VEL: for<'a> Indexable3DMut<'a, Output = &'a mut f32>,
    VORT: for<'a> Indexable3D<'a, Output = &'a f32>,
{
    let size = vorticies.size();
    for c in iterator::iterate_range(coords::ONES.into(), size - coords::ONES) {
        let lr = *vorticies.element(c + coords::X_FORW) - *vorticies.element(c + coords::X_BACK);
        let ud = *vorticies.element(c + coords::Y_FORW) - *vorticies.element(c + coords::Y_BACK);
        let bf = *vorticies.element(c + coords::Z_FORW) - *vorticies.element(c + coords::Z_BACK);
        let length = (lr * lr + ud * ud + bf * bf).sqrt();
        if length > f32::EPSILON {
            let magnitude = *vorticies.element(c) * force / length;

            *wx.element_mut(c) -= ud * magnitude;
            *wy.element_mut(c) += lr * magnitude;
            *wz.element_mut(c) += bf * magnitude;
        }
    }
}
