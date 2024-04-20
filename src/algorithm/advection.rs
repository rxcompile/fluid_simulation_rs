use crate::{
    data::flow::FlowFlags,
    math::{coords, iterator, Coords, CoordsDiff, Slice3D, Slice3DMut},
    Sized3D,
};

const DIFF_TABLE: [CoordsDiff; 8] = [
    CoordsDiff(0, 0, 0),
    CoordsDiff(1, 0, 0),
    CoordsDiff(0, 1, 0),
    CoordsDiff(1, 1, 0),
    CoordsDiff(0, 0, 1),
    CoordsDiff(1, 0, 1),
    CoordsDiff(0, 1, 1),
    CoordsDiff(1, 1, 1),
];

/*
    Composition coefficients
        A_________B
        |\        |\
        | \E______|_\F
        |  |      |  |
        |  |  *new_position(x,y,z)
        C--|------D  |
         \ |       \ |
          \|G_______\H
*/
#[derive(Clone, Default)]
pub(crate) struct AdvectionResult {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
    g: f32,
    h: f32,
    new_position: Coords,
}

pub(crate) fn generate_advection_coefficients<DST, TTL, VEL, BLK>(
    dst: &mut DST,
    totals: &mut TTL,
    vel: &VEL,
    blockage: &BLK,
    force: f32,
) where
    DST: for<'a> Slice3DMut<Output<'a> = &'a mut Option<AdvectionResult>>,
    TTL: for<'a> Slice3DMut<Output<'a> = &'a mut f32> + Sized3D,
    VEL: for<'a> Slice3D<Output<'a> = [&'a f32; 3]>,
    BLK: for<'a> Slice3D<Output<'a> = &'a FlowFlags>,
{
    // This can easily be threaded as the input array is independent from the
    // output array
    let size = totals.size();
    for c in iterator::iterate(size - coords::ONES) {
        let [vx, vy, vz] = vel.slice(&c);

        if vx.abs() <= f32::EPSILON && vy.abs() <= f32::EPSILON && vz.abs() <= f32::EPSILON {
            continue;
        }

        // Find the floating point location of the advection
        let mut new = (
            c.0 as f32 + vx * force,
            c.1 as f32 + vy * force,
            c.2 as f32 + vz * force,
        );

        // Check for and correct boundary collisions
        let _is_collided = collide(&mut new, c, *blockage.slice(&c));

        // Find the nearest top-left integer grid point of the advection
        // x, y, z locations of top-left-back grid point (A) after advection
        let tx1 = new.0.floor() as usize;
        let ty1 = new.1.floor() as usize;
        let tz1 = new.2.floor() as usize;

        // Store the fractional parts
        let fx1 = new.0.fract();
        let fy1 = new.1.fract();
        let fz1 = new.2.fract();

        /*
        A_________B
        |\        |\
        | \E______|_\F
        |  |      |  |
        |  |      |  |
        C--|------D  |
         \ |       \ |
          \|G_______\H


        From Mick West:
        By adding the source value into the destination, we handle the problem
        of multiple destinations but by subtracting it from the source we
        gloss over the problem of multiple sources. Suppose multiple
        destinations have the same (partial) source cells, then what happens
        is the first dest that is processed will get all of that source cell
        (or all of the fraction it needs).  Subsequent dest cells will get a
        reduced fraction.  In extreme cases this will lead to holes forming
        based on the update order.

        Solution:  Maintain an array for dest cells, and source cells.
        For dest cells, store the eight source cells and the eight fractions
        For source cells, store the number of dest cells that source from
        here, and the total fraction E.G.  Dest cells A, B, C all source from
        cell D (and explicit others XYZ, which we don't need to store) So,
        dest cells store A->D(0.1)XYZ..., B->D(0.5)XYZ.... C->D(0.7)XYZ...
        Source Cell D is updated with A, B then C
        Update A:   Dests = 1, Tot = 0.1
        Update B:   Dests = 2, Tot = 0.6
        Update C:   Dests = 3, Tot = 1.3

        How much should go to each of A, B and C? They are asking for a total
        of 1.3, so should they get it all, or should they just get 0.4333 in
        total? Ad Hoc answer: if total <=1 then they get what they ask for if
        total >1 then is is divided between them proportionally. If there were
        two at 1.0, they would get 0.5 each If there were two at 0.5, they
        would get 0.5 each If there were two at 0.1, they would get 0.1 each
        If there were one at 0.6 and one at 0.8, they would get 0.6/1.4 and
        0.8/1.4  (0.429 and 0.571) each

        So in our example, total is 1.3,
        A gets 0.1/1.3, B gets 0.6/1.3 C gets 0.7/1.3, all totaling 1.0

        */
        // Bi-linear interpolation
        let result = AdvectionResult {
            a: (1.0 - fz1) * (1.0 - fy1) * (1.0 - fx1),
            b: (1.0 - fz1) * (1.0 - fy1) * fx1,
            c: (1.0 - fz1) * fy1 * (1.0 - fx1),
            d: (1.0 - fz1) * fy1 * fx1,
            e: fz1 * (1.0 - fy1) * (1.0 - fx1),
            f: fz1 * (1.0 - fy1) * fx1,
            g: fz1 * fy1 * (1.0 - fx1),
            h: fz1 * fy1 * fx1,
            new_position: (tx1, ty1, tz1).into(),
        };

        // Accumulating the total value for the four destinations
        let coords0 = result.new_position + DIFF_TABLE[0];
        let coords1 = result.new_position + DIFF_TABLE[1];
        let coords2 = result.new_position + DIFF_TABLE[2];
        let coords3 = result.new_position + DIFF_TABLE[3];
        let coords4 = result.new_position + DIFF_TABLE[4];
        let coords5 = result.new_position + DIFF_TABLE[5];
        let coords6 = result.new_position + DIFF_TABLE[6];
        let coords7 = result.new_position + DIFF_TABLE[7];
        *totals.slice_mut(&coords0) += result.a;
        *totals.slice_mut(&coords1) += result.b;
        *totals.slice_mut(&coords2) += result.c;
        *totals.slice_mut(&coords3) += result.d;
        *totals.slice_mut(&coords4) += result.e;
        *totals.slice_mut(&coords5) += result.f;
        *totals.slice_mut(&coords6) += result.g;
        *totals.slice_mut(&coords7) += result.h;
        *dst.slice_mut(&c) = Some(result);
    }

    // Normalize values
    for c in iterator::iterate(size) {
        if let Some(k) = dst.slice_mut(&c) {
            // Get the TOTAL fraction requested from each source cell
            // If less then 1.0 in total then no scaling is necessary
            // Scale the amount we are transferring
            k.a /= totals.slice_mut(&(k.new_position + DIFF_TABLE[0])).max(1.0);
            k.b /= totals.slice_mut(&(k.new_position + DIFF_TABLE[1])).max(1.0);
            k.c /= totals.slice_mut(&(k.new_position + DIFF_TABLE[2])).max(1.0);
            k.d /= totals.slice_mut(&(k.new_position + DIFF_TABLE[3])).max(1.0);
            k.e /= totals.slice_mut(&(k.new_position + DIFF_TABLE[4])).max(1.0);
            k.f /= totals.slice_mut(&(k.new_position + DIFF_TABLE[5])).max(1.0);
            k.g /= totals.slice_mut(&(k.new_position + DIFF_TABLE[6])).max(1.0);
            k.h /= totals.slice_mut(&(k.new_position + DIFF_TABLE[7])).max(1.0);
        }
    }
}

fn collide(new: &mut (f32, f32, f32), c: Coords, blockage: FlowFlags) -> bool {
    const MAX_ADVECT: f32 = 1.5; // 1.5 - is center of neighbor cell
    const CLAMP_MIN: f32 = -MAX_ADVECT + f32::EPSILON;
    const CLAMP_MAX: f32 = MAX_ADVECT - f32::EPSILON;
    let this = (c.0 as f32, c.1 as f32, c.2 as f32);
    let delta_x = (new.0 - this.0 as f32).clamp(CLAMP_MIN, CLAMP_MAX);
    let delta_y = (new.1 - this.1 as f32).clamp(CLAMP_MIN, CLAMP_MAX);
    let delta_z = (new.2 - this.2 as f32).clamp(CLAMP_MIN, CLAMP_MAX);
    new.0 = this.0 + delta_x;
    new.1 = this.1 + delta_y;
    new.2 = this.2 + delta_z;

    let mut collided = false;
    if delta_x > 0.0 && blockage.contains(FlowFlags::X_FORW) {
        new.0 = this.0;
        collided = true;
    }
    if delta_y > 0.0 && blockage.contains(FlowFlags::Y_FORW) {
        new.1 = this.1;
        collided = true;
    }
    if delta_z > 0.0 && blockage.contains(FlowFlags::Z_FORW) {
        new.2 = this.2;
        collided = true;
    }
    if delta_x < 0.0 && blockage.contains(FlowFlags::X_BACK) {
        new.0 = this.0;
        collided = true;
    }
    if delta_y < 0.0 && blockage.contains(FlowFlags::Y_BACK) {
        new.1 = this.1;
        collided = true;
    }
    if delta_z < 0.0 && blockage.contains(FlowFlags::Z_BACK) {
        new.2 = this.2;
        collided = true;
    }
    collided
}

pub(crate) fn forward_advection<DST, SRC, COEF>(dst: &mut DST, src: &SRC, coefficients: &COEF)
where
    DST: for<'a> Slice3DMut<Output<'a> = &'a mut f32>,
    SRC: for<'a> Slice3D<Output<'a> = &'a f32>,
    COEF: for<'a> Slice3D<Output<'a> = &'a Option<AdvectionResult>> + Sized3D,
{
    let size = coefficients.size();
    for c in iterator::iterate(size) {
        if let Some(v) = &coefficients.slice(&c) {
            let mut res = v.clone();
            res.a *= src.slice(&(c + DIFF_TABLE[0]));
            res.b *= src.slice(&(c + DIFF_TABLE[1]));
            res.c *= src.slice(&(c + DIFF_TABLE[2]));
            res.d *= src.slice(&(c + DIFF_TABLE[3]));
            res.e *= src.slice(&(c + DIFF_TABLE[4]));
            res.f *= src.slice(&(c + DIFF_TABLE[5]));
            res.g *= src.slice(&(c + DIFF_TABLE[6]));
            res.h *= src.slice(&(c + DIFF_TABLE[7]));

            *dst.slice_mut(&c) -= res.a + res.b + res.c + res.d + res.e + res.f + res.g + res.h;

            *dst.slice_mut(&(res.new_position + DIFF_TABLE[0])) += res.a;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[1])) += res.b;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[2])) += res.c;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[3])) += res.d;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[4])) += res.e;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[5])) += res.f;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[6])) += res.g;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[7])) += res.h;
        }
    }
}

pub(crate) fn reverse_advection<DST, SRC, COEF>(dst: &mut DST, src: &SRC, coefficients: &COEF)
where
    DST: for<'a> Slice3DMut<Output<'a> = &'a mut f32>,
    SRC: for<'a> Slice3D<Output<'a> = &'a f32>,
    COEF: for<'a> Slice3D<Output<'a> = &'a Option<AdvectionResult>> + Sized3D,
{
    let size = coefficients.size();
    for c in iterator::iterate(size) {
        if let Some(v) = &coefficients.slice(&c) {
            let mut res = v.clone();
            res.a *= src.slice(&(c + DIFF_TABLE[0]));
            res.b *= src.slice(&(c + DIFF_TABLE[1]));
            res.c *= src.slice(&(c + DIFF_TABLE[2]));
            res.d *= src.slice(&(c + DIFF_TABLE[3]));
            res.e *= src.slice(&(c + DIFF_TABLE[4]));
            res.f *= src.slice(&(c + DIFF_TABLE[5]));
            res.g *= src.slice(&(c + DIFF_TABLE[6]));
            res.h *= src.slice(&(c + DIFF_TABLE[7]));

            *dst.slice_mut(&c) += res.a + res.b + res.c + res.d + res.e + res.f + res.g + res.h;

            *dst.slice_mut(&(res.new_position + DIFF_TABLE[0])) -= res.a;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[1])) -= res.b;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[2])) -= res.c;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[3])) -= res.d;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[4])) -= res.e;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[5])) -= res.f;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[6])) -= res.g;
            *dst.slice_mut(&(res.new_position + DIFF_TABLE[7])) -= res.h;
        }
    }
}
