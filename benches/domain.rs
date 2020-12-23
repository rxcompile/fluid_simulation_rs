#![feature(test)]
extern crate test;

use fluid_simulation::{Coords, DomainDefault, DomainProperties};
use test::Bencher;

#[bench]
fn bench_simulation_step_100(b: &mut Bencher) {
    let prop = DomainProperties {
        size: Coords(100, 100, 100),
        ..Default::default()
    };
    let mut domain = DomainDefault::new(prop);
    b.iter(|| domain.simulate());
}

#[bench]
fn bench_simulation_step_10(b: &mut Bencher) {
    let prop = DomainProperties {
        size: Coords(10, 10, 10),
        ..Default::default()
    };
    let mut domain = DomainDefault::new(prop);
    b.iter(|| domain.simulate());
}
