use crate::{
    math::{coords, iterator, Slice3D, Slice3DMut},
    Sized3D,
};

pub fn decay_velocity<DST, SRC>(dst: &mut DST, src: &SRC, coefficient: f32)
where
    DST: for<'a> Slice3DMut<Output<'a> = &'a mut f32> + Sized3D,
    SRC: for<'a> Slice3D<Output<'a> = &'a f32>,
{
    for c in iterator::iterate(dst.size()) {
        *dst.slice_mut(&c) = src.slice(&c) * coefficient;
    }
}

pub fn pressuarize<VEL, PR, const PR_SIZE: usize>(vel: &mut VEL, pr: &PR, force: f32)
where
    VEL: for<'a> Slice3DMut<Output<'a> = [&'a mut f32; 3]> + 'static,
    PR: for<'a> Slice3D<Output<'a> = [&'a f32; PR_SIZE]> + Sized3D + 'static,
{
    let size = pr.size();
    let sum = |c| -> f32 { pr.slice(&c).into_iter().fold(0.0f32, |a, i| a + *i) };
    for c in iterator::iterate(size - coords::ONES) {
        let src_press = sum(c);
        let force_x = sum(c + coords::X_FORW) - src_press;
        let force_y = sum(c + coords::Y_FORW) - src_press;
        let force_z = sum(c + coords::Z_FORW) - src_press;

        let vel0 = vel.slice_mut(&c);
        *vel0[0] += force * force_x;
        *vel0[1] += force * force_y;
        *vel0[2] += force * force_z;

        *vel.slice_mut(&(c + coords::X_FORW))[0] -= force * force_x;
        *vel.slice_mut(&(c + coords::Y_FORW))[1] -= force * force_y;
        *vel.slice_mut(&(c + coords::Z_FORW))[2] -= force * force_z;
    }
}

pub fn generate_vortexes<VORT, VEL>(vorticies: &mut VORT, vel: &VEL)
where
    VORT: for<'a> Slice3DMut<Output<'a> = &'a mut f32> + Sized3D,
    VEL: for<'a> Slice3D<Output<'a> = [&'a f32; 3]>,
{
    let size = vorticies.size();
    for c in iterator::iterate_range(coords::ONES.into(), size - coords::ONES) {
        let x = vel.slice(&(c + coords::Y_FORW))[0] - vel.slice(&(c + coords::Y_BACK))[0];
        let y = vel.slice(&(c + coords::X_FORW))[1] - vel.slice(&(c + coords::X_BACK))[1];
        let z = vel.slice(&(c + coords::Z_FORW))[2] - vel.slice(&(c + coords::Z_BACK))[2];
        *vorticies.slice_mut(&c) = ((x - y - z) * 0.5).abs()
    }
}

pub fn apply_vortex<VEL, VORT>(vel: &mut VEL, vorticies: &VORT, force: f32)
where
    VEL: for<'a> Slice3DMut<Output<'a> = [&'a mut f32; 3]> + 'static,
    VORT: for<'a> Slice3D<Output<'a> = &'a f32> + Sized3D + 'static,
{
    let size = vorticies.size();
    for c in iterator::iterate_range(coords::ONES.into(), size - coords::ONES) {
        let lr = vorticies.slice(&(c + coords::X_FORW)) - vorticies.slice(&(c + coords::X_BACK));
        let ud = vorticies.slice(&(c + coords::Y_FORW)) - vorticies.slice(&(c + coords::Y_BACK));
        let bf = vorticies.slice(&(c + coords::Z_FORW)) - vorticies.slice(&(c + coords::Z_BACK));
        let length = (lr * lr + ud * ud + bf * bf).sqrt();
        if length > f32::EPSILON {
            let magnitude = vorticies.slice(&c) * force / length;

            *vel.slice_mut(&c)[0] -= ud * magnitude;
            *vel.slice_mut(&c)[1] += lr * magnitude;
            *vel.slice_mut(&c)[2] += bf * magnitude;
        }
    }
}
