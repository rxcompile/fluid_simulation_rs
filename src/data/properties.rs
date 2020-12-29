pub struct DomainProperties {
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
