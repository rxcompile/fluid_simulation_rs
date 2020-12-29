use crate::{
    algorithm::{advection, diffusion, forces},
    data::runtime::{DomainRuntime, DomainTemp},
    math::{swapchain::Swapable, Fillable, Indexable3D, Indexable3DMut},
    support_utils, Coords, DomainProperties,
};

#[derive(Default)]
pub struct Domain<const P_SIZE: usize, const X: usize, const Y: usize, const Z: usize> {
    pub data: DomainRuntime<P_SIZE, X, Y, Z>,
    temp: DomainTemp<X, Y, Z>,
    pub prop: DomainProperties,
}

impl<const P_SIZE: usize, const X: usize, const Y: usize, const Z: usize> Domain<P_SIZE, X, Y, Z> {
    pub fn new(prop: DomainProperties) -> Self {
        Self {
            prop,
            ..Default::default()
        }
    }

    pub fn pressure(&self, c: Coords) -> [f32; P_SIZE] {
        support_utils::construct_from(self.data.pressure.iter().map(|r| *r.element(c)))
    }

    pub fn set_pressure(&mut self, c: Coords, v: &[f32; P_SIZE]) {
        self.data
            .pressure
            .iter_mut()
            .zip(v.iter())
            .for_each(|(w, r)| *w.element_mut(c) = *r);
    }

    pub fn velocity(&self, c: Coords) -> (f32, f32, f32) {
        (
            *self.data.velocity[0].element(c),
            *self.data.velocity[1].element(c),
            *self.data.velocity[2].element(c),
        )
    }

    pub fn set_velocity(&mut self, c: Coords, v: (f32, f32, f32)) {
        *self.data.velocity[0].element_mut(c) = v.0;
        *self.data.velocity[1].element_mut(c) = v.1;
        *self.data.velocity[2].element_mut(c) = v.2;
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
            for (src, dst) in self.data.pressure.iter_mut().map(|i| i.rw_pair()) {
                diffusion::diffusion_step(dst, src, &self.data.blockage, force);
            }
            // swapchain
            self.data.pressure.swap_buffers();
        }
    }

    fn sim_forces(&mut self) {
        if let Some(decay) = self.prop.velocity_decay {
            let coefficient = (1.0 - decay).powf(self.prop.step_delta_time);
            for (src, dst) in self.data.velocity.iter_mut().map(|v| v.rw_pair()) {
                forces::decay_velocity(dst, src, coefficient);
            }
            // swapchain
            self.data.velocity.swap_buffers();
        }
        if let Some(pressure_acceleration) = self.prop.pressure_acceleration {
            self.data.velocity.copy_from_read();
            let force = pressure_acceleration * self.prop.step_delta_time;
            let mut witer = self.data.velocity.iter_mut();
            forces::pressuarize(
                witer.next().unwrap().write(),
                witer.next().unwrap().write(),
                witer.next().unwrap().write(),
                self.data.pressure[0].read(),
                force,
            );
            // swapchain
            self.data.velocity.swap_buffers();
        }
        if let Some(vorticity) = self.prop.vorticity {
            self.data.velocity.copy_from_read();

            let force = vorticity * self.prop.step_delta_time;
            forces::generate_vortexes(
                &mut self.temp.vorticies,
                self.data.velocity[0].read(),
                self.data.velocity[1].read(),
                self.data.velocity[2].read(),
            );
            let mut witer = self.data.velocity.iter_mut();
            forces::apply_vortex(
                witer.next().unwrap().write(),
                witer.next().unwrap().write(),
                witer.next().unwrap().write(),
                &self.temp.vorticies,
                force,
            );
            // swapchain
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

        let (rx, ry, rz) = (
            self.data.velocity[0].read(),
            self.data.velocity[1].read(),
            self.data.velocity[2].read(),
        );
        // cleanup totals
        self.temp.forward_velocity_coefficients_totals.fill(0.0);
        self.temp.reverse_velocity_coefficients_totals.fill(0.0);
        self.temp.pressure_coefficients_totals.fill(0.0);
        // Advection order makes significant differences
        // Advecting pressure first leads to self-maintaining waves and ripple
        // artifacts Advecting velocity first naturally dissipates the waves
        advection::generate_advection_coefficients(
            &mut self.temp.forward_velocity_coefficients,
            &mut self.temp.forward_velocity_coefficients_totals,
            rx,
            ry,
            rz,
            &self.data.blockage,
            scale * self.prop.velocity_props.advection,
        );
        advection::generate_advection_coefficients(
            &mut self.temp.reverse_velocity_coefficients,
            &mut self.temp.reverse_velocity_coefficients_totals,
            rx,
            ry,
            rz,
            &self.data.blockage,
            -scale * self.prop.velocity_props.advection,
        );
        advection::generate_advection_coefficients(
            &mut self.temp.pressure_coefficients,
            &mut self.temp.pressure_coefficients_totals,
            rx,
            ry,
            rz,
            &self.data.blockage,
            scale * self.prop.pressure_props.advection,
        );

        for (r, w) in self.data.velocity.iter_mut().map(|i| i.rw_pair()) {
            advection::forward_advection(w, r, &self.temp.forward_velocity_coefficients);
        }
        for (r, w) in self.data.velocity.iter_mut().map(|i| i.rw_pair()) {
            advection::reverse_advection(w, r, &self.temp.forward_velocity_coefficients);
        }
        self.data.velocity.swap_buffers();

        for (r, w) in self.data.pressure.iter_mut().map(|i| i.rw_pair()) {
            advection::forward_advection(w, r, &self.temp.pressure_coefficients);
        }
        for (r, w) in self.data.pressure.iter_mut().map(|i| i.rw_pair()) {
            advection::reverse_advection(w, r, &self.temp.pressure_coefficients);
        }
        self.data.pressure.swap_buffers();
    }
}
