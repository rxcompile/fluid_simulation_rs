use crate::math::{
    self, Array3D, Indexable3D, ONES, X_BACK, X_FORW, Y_BACK, Y_FORW, Z_BACK, Z_FORW,
};

pub fn decay_velocity(dst: &mut Array3D<f32>, src: &Array3D<f32>, coefficient: f32) {
    for c in math::iterate(dst.size()) {
        *dst.element_mut(c) = *src.element(c) * coefficient;
    }
}

pub fn pressuarize(pack: &mut [&mut Array3D<f32>], pr: &Array3D<f32>, force: f32) {
    let size = pr.size();
    for c in math::iterate(size - ONES) {
        let src_press = *pr.element(c);
        let force_x = *pr.element(c + X_FORW) - src_press;
        let force_y = *pr.element(c + Y_FORW) - src_press;
        let force_z = *pr.element(c + Z_FORW) - src_press;

        *pack[0].element_mut(c) += force * force_x;
        *pack[0].element_mut(c + X_FORW) -= force * force_x;

        *pack[1].element_mut(c) += force * force_y;
        *pack[1].element_mut(c + Y_FORW) -= force * force_y;

        *pack[2].element_mut(c) += force * force_z;
        *pack[2].element_mut(c + Z_FORW) -= force * force_z;
    }
}

pub fn generate_vortexes(vorticies: &mut Array3D<f32>, pack: &[&Array3D<f32>]) {
    let size = vorticies.size();
    let curl = |c| {
        let x_curl = *pack[0].element(c + Y_FORW) - *pack[0].element(c + Y_BACK);
        let y_curl = *pack[1].element(c + X_FORW) - *pack[1].element(c + X_BACK);
        let z_curl = *pack[2].element(c + Z_FORW) - *pack[2].element(c + Z_BACK);
        (x_curl - y_curl - z_curl) * 0.5
    };
    for c in math::iterate_from(size - ONES, ONES.into()) {
        *vorticies.element_mut(c) = curl(c).abs();
    }
}

pub fn apply_vortex(pack: &mut [&mut Array3D<f32>], vorticies: &Array3D<f32>, force: f32) {
    let size = vorticies.size();
    for c in math::iterate_from(size - ONES, ONES.into()) {
        let lr_curl = *vorticies.element(c + X_FORW) - *vorticies.element(c + X_BACK);
        let ud_curl = *vorticies.element(c + Y_FORW) - *vorticies.element(c + Y_BACK);
        let bf_curl = *vorticies.element(c + Z_FORW) - *vorticies.element(c + Z_BACK);
        let length = (lr_curl * lr_curl + ud_curl * ud_curl + bf_curl * bf_curl).sqrt();
        if length > f32::EPSILON {
            let magnitude = *vorticies.element(c) * force / length;
            *pack[0].element_mut(c) -= ud_curl * magnitude;
            *pack[1].element_mut(c) += lr_curl * magnitude;
            *pack[2].element_mut(c) += bf_curl * magnitude;
        }
    }
}
