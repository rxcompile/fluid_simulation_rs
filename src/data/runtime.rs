use crate::{
    algorithm::advection,
    math::{swapchain::SwapchainPack, SizedArray3D},
    support_utils,
};

use super::flow;

pub struct DomainRuntime<const P: usize, const SX: usize, const SY: usize, const SZ: usize> {
    pub velocity: SwapchainPack<SizedArray3D<f32, SX, SY, SZ>, 3, 2>,
    pub pressure: SwapchainPack<SizedArray3D<f32, SX, SY, SZ>, P, 2>,
    pub blockage: SizedArray3D<flow::FlowFlags, SX, SY, SZ>,
}

impl<const P: usize, const SX: usize, const SY: usize, const SZ: usize> Default
    for DomainRuntime<P, SX, SY, SZ>
{
    fn default() -> Self {
        Self {
            velocity: support_utils::construct_default(),
            pressure: support_utils::construct_default(),
            blockage: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct DomainTemp<const SX: usize, const SY: usize, const SZ: usize> {
    pub vorticies: SizedArray3D<f32, SX, SY, SZ>,
    pub forward_velocity_coefficients: SizedArray3D<Option<advection::AdvectionResult>, SX, SY, SZ>,
    pub reverse_velocity_coefficients: SizedArray3D<Option<advection::AdvectionResult>, SX, SY, SZ>,
    pub pressure_coefficients: SizedArray3D<Option<advection::AdvectionResult>, SX, SY, SZ>,
    pub forward_velocity_coefficients_totals: SizedArray3D<f32, SX, SY, SZ>,
    pub reverse_velocity_coefficients_totals: SizedArray3D<f32, SX, SY, SZ>,
    pub pressure_coefficients_totals: SizedArray3D<f32, SX, SY, SZ>,
}
