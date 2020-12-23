use crate::{
    algorithm::{advection, diffusion, forces},
    fluid::FlowFlags,
    math::{Array3D, Coords, Indexable3D, SizeCreatable},
    swapchain::{Swapable, Swapchain, SwapchainPack},
};

pub struct Domain<const SWAPCHAIN_SIZE: usize> {
    data: DomainRuntime<SWAPCHAIN_SIZE>,
    temp: DomainTemp,
    pub prop: DomainProperties,
}

struct DomainRuntime<const SWAPCHAIN_SIZE: usize> {
    velocity: SwapchainPack<Array3D<f32>, 3, SWAPCHAIN_SIZE>,
    pressure: Swapchain<Array3D<f32>, SWAPCHAIN_SIZE>,
    blockage: Array3D<FlowFlags>,
}

impl<const SWAPCHAIN_SIZE: usize> DomainRuntime<SWAPCHAIN_SIZE> {
    fn new(size: Coords) -> Self {
        Self {
            velocity: SwapchainPack::new(size),
            pressure: Swapchain::new(size),
            blockage: Array3D::new(size),
        }
    }
}

struct DomainTemp {
    vorticies: Array3D<f32>,
    forward_velocity_coefficients: advection::AdvectionCoefficients,
    forward_velocity_coefficients_totals: Array3D<f32>,
    reverse_velocity_coefficients: advection::AdvectionCoefficients,
    reverse_velocity_coefficients_totals: Array3D<f32>,
    pressure_coefficients: advection::AdvectionCoefficients,
    pressure_coefficients_totals: Array3D<f32>,
}

impl DomainTemp {
    fn new(size: Coords) -> Self {
        Self {
            vorticies: Array3D::new(size),
            forward_velocity_coefficients: Array3D::new(size),
            forward_velocity_coefficients_totals: Array3D::new(size),
            reverse_velocity_coefficients: Array3D::new(size),
            reverse_velocity_coefficients_totals: Array3D::new(size),
            pressure_coefficients: Array3D::new(size),
            pressure_coefficients_totals: Array3D::new(size),
        }
    }
}

pub struct DomainProperties {
    pub size: Coords,
    pub velocity_props: PackProperties,
    pub pressure_props: PackProperties,

    pub diffusion_steps: usize,
    pub step_delta_time: f32,
    pub velocity_decay: Option<f32>,
    pub pressure_acceleration: Option<f32>,
    pub vorticity: Option<f32>,
}

pub struct PackProperties {
    pub advection: f32,
    pub diffusion: f32,
}

impl Default for DomainProperties {
    fn default() -> Self {
        Self {
            size: Coords(1, 1, 1),
            velocity_props: PackProperties {
                advection: 0.1,
                diffusion: 0.1,
            },
            pressure_props: PackProperties {
                advection: 0.1,
                diffusion: 0.1,
            },
            diffusion_steps: 1,
            step_delta_time: 1.0 / 10.0,
            velocity_decay: Some(0.1),
            pressure_acceleration: Some(0.1),
            vorticity: Some(0.1),
        }
    }
}

impl<const SWAPCHAIN_SIZE: usize> Domain<SWAPCHAIN_SIZE> {
    pub fn new(prop: DomainProperties) -> Domain<SWAPCHAIN_SIZE> {
        let size = prop.size;
        Domain {
            data: DomainRuntime::new(size),
            temp: DomainTemp::new(size),
            prop,
        }
    }

    pub fn pressure(&self, c: Coords) -> f32 {
        *self.data.pressure.read().element(c)
    }

    pub fn set_pressure(&mut self, c: Coords, v: f32) {
        *self.data.pressure.write().element_mut(c) = v
    }

    pub fn blocked(&self, c: Coords) -> FlowFlags {
        *self.data.blockage.element(c)
    }

    pub fn set_block(&mut self, c: Coords, f: FlowFlags) {
        *self.data.blockage.element_mut(c) = f
    }

    pub fn velocity(&self, c: Coords) -> (f32, f32, f32) {
        (
            *self.data.velocity[0].element(c),
            *self.data.velocity[1].element(c),
            *self.data.velocity[2].element(c),
        )
    }

    pub fn set_velocity(&mut self, c: Coords, v: (f32, f32, f32)) {
        let mut iter = self.data.velocity.iter_mut();
        *iter.next().unwrap().element_mut(c) = v.0;
        *iter.next().unwrap().element_mut(c) = v.1;
        *iter.next().unwrap().element_mut(c) = v.2;
    }

    pub fn simulate(&mut self) {
        // apply modifications from user
        self.data.pressure.swap();
        self.data.velocity.swap();

        // simulate next frame
        self.sim_diffusion();
        self.sim_forces();
        self.sim_advection();
    }
}

impl<const SWAPCHAIN_SIZE: usize> Domain<SWAPCHAIN_SIZE> {
    fn sim_diffusion(&mut self) {
        let force = self.prop.pressure_props.diffusion;
        for _ in 0..self.prop.diffusion_steps {
            let (src, dst) = self.data.pressure.rw_pair();
            diffusion::diffusion_step(dst, src, &self.data.blockage, force);
            // swapchain
            self.data.pressure.swap();
        }
    }

    fn sim_forces(&mut self) {
        if let Some(decay) = self.prop.velocity_decay {
            let coefficient = (1.0 - decay).powf(self.prop.step_delta_time);
            for (src, dst) in self.data.velocity.iter_mut().map(|v| v.rw_pair()) {
                forces::decay_velocity(dst, src, coefficient);
            }
            // swapchain
            self.data.velocity.swap();
        }
        if let Some(pressure_acceleration) = self.prop.pressure_acceleration {
            self.data.velocity.copy_from_read();

            let force = pressure_acceleration * self.prop.step_delta_time;
            // TODO: heap allocation is unfortunate here,
            // but there is no way to express this as stack allocated array of 3 mut refs
            forces::pressuarize(
                self.data
                    .velocity
                    .iter_mut()
                    .map(|v| v.write())
                    .collect::<Vec<_>>()
                    .as_mut_slice(),
                self.data.pressure.read(),
                force,
            );
            // swapchain
            self.data.velocity.swap();
        }
        if let Some(vorticity) = self.prop.vorticity {
            self.data.velocity.copy_from_read();

            let force = vorticity * self.prop.step_delta_time;
            // TODO: heap allocation is unfortunate here,
            // but there is no way to express this as stack allocated array of 3 mut refs
            forces::generate_vortexes(
                &mut self.temp.vorticies,
                self.data
                    .velocity
                    .iter()
                    .map(|v| v.read())
                    .collect::<Vec<_>>()
                    .as_slice(),
            );
            // TODO: heap allocation is unfortunate here,
            // but there is no way to express this as stack allocated array of 3 mut refs
            forces::apply_vortex(
                self.data
                    .velocity
                    .iter_mut()
                    .map(|v| v.write())
                    .collect::<Vec<_>>()
                    .as_mut_slice(),
                &self.temp.vorticies,
                force,
            );
            // swapchain
            self.data.velocity.swap();
        }
    }

    fn sim_advection(&mut self) {
        // Change advection scale depending on grid size. Smaller grids means larger
        // cells, so scale should be smaller. Average dimension size of std_dimension
        // value (100) equals an advection_scale of 1
        let avg_dimension = (self.prop.size.0 + self.prop.size.1 + self.prop.size.2) as f32 / 3.0;
        let std_dimension = 100.0f32;
        let scale = avg_dimension / std_dimension * self.prop.step_delta_time;
        if scale <= f32::EPSILON {
            return;
        }

        let mut iter = self.data.velocity.iter_mut();
        let (vx, wx) = iter.next().unwrap().rw_pair();
        let (vy, wy) = iter.next().unwrap().rw_pair();
        let (vz, wz) = iter.next().unwrap().rw_pair();
        let (pr, pw) = self.data.pressure.rw_pair();

        // Advection order makes significant differences
        // Advecting pressure first leads to self-maintaining waves and ripple
        // artifacts Advecting velocity first naturally dissipates the waves
        advection::generate_advection_coefficients(
            &mut self.temp.forward_velocity_coefficients,
            &mut self.temp.forward_velocity_coefficients_totals,
            vx,
            vy,
            vz,
            &self.data.blockage,
            scale * self.prop.velocity_props.advection,
        );
        advection::generate_advection_coefficients(
            &mut self.temp.reverse_velocity_coefficients,
            &mut self.temp.reverse_velocity_coefficients_totals,
            vx,
            vy,
            vz,
            &self.data.blockage,
            -scale * self.prop.velocity_props.advection,
        );
        advection::generate_advection_coefficients(
            &mut self.temp.pressure_coefficients,
            &mut self.temp.pressure_coefficients_totals,
            vx,
            vy,
            vz,
            &self.data.blockage,
            scale * self.prop.pressure_props.advection,
        );

        advection::forward_advection(wx, vx, &self.temp.forward_velocity_coefficients);
        advection::forward_advection(wy, vy, &self.temp.forward_velocity_coefficients);
        advection::forward_advection(wz, vz, &self.temp.forward_velocity_coefficients);

        advection::reverse_advection(wx, vx, &self.temp.reverse_velocity_coefficients);
        advection::reverse_advection(wy, vy, &self.temp.reverse_velocity_coefficients);
        advection::reverse_advection(wz, vz, &self.temp.reverse_velocity_coefficients);
        self.data.velocity.swap();

        advection::forward_advection(pw, pr, &self.temp.pressure_coefficients);
        advection::reverse_advection(pw, pr, &self.temp.pressure_coefficients);
        self.data.pressure.swap();
    }
}
