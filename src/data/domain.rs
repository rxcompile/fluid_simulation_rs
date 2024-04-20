use crate::{
    algorithm::{advection, diffusion, forces}, data::runtime::{DomainRuntime, DomainTemp}, iterator, math::{swapchain::Swapable, Sized3D, Slice3D, Slice3DMut}, Coords, DomainProperties
};

#[derive(Default)]
pub struct Domain<const P_SIZE: usize, const X: usize, const Y: usize, const Z: usize> {
    pub data: DomainRuntime<P_SIZE, X, Y, Z>,
    temp: DomainTemp<X, Y, Z>,
    pub prop: DomainProperties,
}

impl<const P_SIZE: usize, const X: usize, const Y: usize, const Z: usize> Sized3D
    for Domain<P_SIZE, X, Y, Z>
{
    fn size(&self) -> Coords {
        Coords(X, Y, Z)
    }
}

impl<const P_SIZE: usize, const X: usize, const Y: usize, const Z: usize> Domain<P_SIZE, X, Y, Z> {
    pub fn new(prop: DomainProperties) -> Self {
        Self {
            prop,
            ..Default::default()
        }
    }

    pub fn pressure(&self, c: &Coords) -> [f32; P_SIZE] {
        self.data.pressure.slice(c).map(|x| *x)
    }

    pub fn set_pressure(&mut self, c: &Coords, v: &[f32; P_SIZE]) {
        for i in 0..P_SIZE {
            *self.data.pressure.slice_mut(c)[i] = v[i];
        }
    }

    pub fn velocity(&self, c: &Coords) -> (f32, f32, f32) {
        let ar = self.data.velocity.slice(c);
        (*ar[0], *ar[1], *ar[2])
    }

    pub fn set_velocity(&mut self, c: &Coords, v: (f32, f32, f32)) {
        let vel = self.data.velocity.slice_mut(c);
        *vel[0] = v.0;
        *vel[1] = v.1;
        *vel[2] = v.2;
    }

    pub fn simulate(&mut self) {
        // apply modifications from user
        self.data.pressure.swap_buffers();
        self.data.velocity.swap_buffers();

        // simulate next frame
        self.sim_diffusion();
        self.sim_forces();
        self.sim_advection();
    }

    fn sim_diffusion(&mut self) {
        let force = self.prop.pressure_props.diffusion;
        for _ in 0..self.prop.diffusion_steps {
            for (src, dst) in self.data.pressure.rw_pairs() {
                diffusion::diffusion_step(
                    dst,
                    src,
                    &self.data.blockage,
                    force / self.prop.diffusion_steps as f32,
                );
            }
            // swapchain
            self.data.pressure.swap_buffers();
        }
    }

    fn sim_forces(&mut self) {
        if let Some(decay) = self.prop.velocity_decay {
            let coefficient = (1.0 - decay).powf(self.prop.step_delta_time);
            for (src, dst) in self.data.velocity.rw_pairs() {
                forces::decay_velocity(dst, src, coefficient);
            }
            self.data.velocity.swap_buffers();
        }
        if let Some(pressure_acceleration) = self.prop.pressure_acceleration {
            let force = pressure_acceleration * self.prop.step_delta_time;
            forces::pressuarize(&mut self.data.velocity, &self.data.pressure, force);
            self.data.velocity.swap_buffers();
        }
        if let Some(vorticity) = self.prop.vorticity {
            let force = vorticity * self.prop.step_delta_time;
            forces::generate_vortexes(&mut self.temp.vorticies, &self.data.velocity);
            forces::apply_vortex(&mut self.data.velocity, &self.temp.vorticies, force);
            self.data.velocity.swap_buffers();
        }
    }

    fn sim_advection(&mut self) {
        // Change advection scale depending on grid size. Smaller grids means larger
        // cells, so scale should be smaller. Average dimension size of std_dimension
        // value (100) equals an advection_scale of 1
        let avg_dimension = (X + Y + Z) as f32 / 3.0;
        let std_dimension = 100.0f32;
        let scale = avg_dimension / std_dimension * self.prop.step_delta_time;
        if scale <= f32::EPSILON {
            return;
        }

        // cleanup totals
        for c in iterator::iterate(self.size())
        {
            *self.temp.forward_velocity_coefficients_totals.slice_mut(&c) = 0.0;
            *self.temp.reverse_velocity_coefficients_totals.slice_mut(&c) = 0.0;
            *self.temp.pressure_coefficients_totals.slice_mut(&c) = 0.0;
        }
        // Advection order makes significant differences
        // Advecting pressure first leads to self-maintaining waves and ripple
        // artifacts Advecting velocity first naturally dissipates the waves
        advection::generate_advection_coefficients(
            &mut self.temp.forward_velocity_coefficients,
            &mut self.temp.forward_velocity_coefficients_totals,
            &self.data.velocity,
            &self.data.blockage,
            scale * self.prop.velocity_props.advection,
        );
        advection::generate_advection_coefficients(
            &mut self.temp.reverse_velocity_coefficients,
            &mut self.temp.reverse_velocity_coefficients_totals,
            &self.data.velocity,
            &self.data.blockage,
            -scale * self.prop.velocity_props.advection,
        );
        advection::generate_advection_coefficients(
            &mut self.temp.pressure_coefficients,
            &mut self.temp.pressure_coefficients_totals,
            &self.data.velocity,
            &self.data.blockage,
            scale * self.prop.pressure_props.advection,
        );

        for (r, w) in self.data.velocity.rw_pairs() {
            advection::forward_advection(w, r, &self.temp.forward_velocity_coefficients);
        }
        for (r, w) in self.data.velocity.rw_pairs() {
            advection::reverse_advection(w, r, &self.temp.forward_velocity_coefficients);
        }
        self.data.velocity.swap_buffers();

        for (r, w) in self.data.pressure.rw_pairs() {
            advection::forward_advection(w, r, &self.temp.pressure_coefficients);
        }
        for (r, w) in self.data.pressure.rw_pairs() {
            advection::reverse_advection(w, r, &self.temp.pressure_coefficients);
        }
        self.data.pressure.swap_buffers();
    }
}
