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

pub fn pressuarize<VEL, PR, const PR_SIZE: usize>(vel: &mut VEL, pr: &PR, force: f32)
where
    VEL: for<'a> Indexable3DMut<'a, Output = [&'a mut f32; 3]>,
    PR: for<'a> Indexable3D<'a, Output = [&'a f32; PR_SIZE]>,
{
    let size = pr.size();
    let sum = |c| pr.element(c).iter().fold(0.0f32, |a, i| a + *i);
    for c in iterator::iterate(size - coords::ONES) {
        let src_press = sum(c);
        let force_x = sum(c + coords::X_FORW) - src_press;
        let force_y = sum(c + coords::Y_FORW) - src_press;
        let force_z = sum(c + coords::Z_FORW) - src_press;

        let vel0 = vel.element_mut(c);
        *vel0[0] += force * force_x;
        *vel0[1] += force * force_y;
        *vel0[2] += force * force_z;

        *vel.element_mut(c + coords::X_FORW)[0] -= force * force_x;
        *vel.element_mut(c + coords::Y_FORW)[1] -= force * force_y;
        *vel.element_mut(c + coords::Z_FORW)[2] -= force * force_z;
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

pub fn apply_vortex<VEL, VORT>(vel: &mut VEL, vorticies: &VORT, force: f32)
where
    VEL: for<'a> Indexable3DMut<'a, Output = [&'a mut f32; 3]>,
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

            let [x, y, z] = vel.element_mut(c);
            *x -= ud * magnitude;
            *y += lr * magnitude;
            *z += bf * magnitude;
        }
    }
}
