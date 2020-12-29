#![allow(incomplete_features)]
#![feature(maybe_uninit_uninit_array, maybe_uninit_extra)]
#![feature(generic_associated_types)]

mod algorithm;
mod data;
mod math;
mod support_utils;

pub use data::domain::Domain;
pub use data::flow::FlowFlags;
pub use data::properties::DomainProperties;
pub use data::properties::PackProperties;
pub use math::swapchain::Swapchain;
pub use math::Coords;
pub use math::Indexable3D;

#[macro_use]
extern crate bitflags;
