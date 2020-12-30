#![feature(maybe_uninit_uninit_array, maybe_uninit_extra)]

mod algorithm;
mod data;
mod math;
mod support_utils;

pub use data::properties::{DomainProperties, PackProperties};
pub use data::{domain::Domain, flow::FlowFlags};
pub use math::swapchain::Swapchain;
pub use math::{iterator, Coords, Indexable3D, Indexable3DMut, Sizeable3D};

#[macro_use]
extern crate bitflags;
