#![feature(min_const_generics)]
#![feature(slice_fill)]
mod algorithm;
mod domain;
mod math;

pub use domain::Domain;
pub use domain::DomainProperties;
pub use domain::PackProperties;
pub use domain::fluid::FlowFlags;
pub use math::Coords;

// usefull aliases
pub type DomainDefault = domain::Domain<2>;

#[macro_use]
extern crate bitflags;
