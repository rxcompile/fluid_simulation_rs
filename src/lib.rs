#![feature(maybe_uninit_uninit_array, maybe_uninit_array_assume_init)]

mod algorithm;
mod data;
mod math;
mod support_utils;

pub use data::properties::{DomainProperties, PackProperties};
pub use data::{domain::Domain, flow::FlowFlags};
pub use math::swapchain::Swapchain;
pub use math::{iterator, Coords, Sized3D, Slice3D, Slice3DMut};
pub use math::Pid;

#[macro_use]
extern crate bitflags;
extern crate rayon;
