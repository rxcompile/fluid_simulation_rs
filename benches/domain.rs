#![feature(test)]
extern crate test;

use fluid_simulation::Domain;
use test::Bencher;

#[bench]
fn bench_simulation_step_100(b: &mut Bencher) {
    let mut domain: Domain<1, 100, 100, 100> = Default::default();
    b.iter(|| domain.simulate());
}

#[bench]
fn bench_simulation_step_10(b: &mut Bencher) {
    let mut domain: Domain<1, 10, 10, 10> = Default::default();
    b.iter(|| domain.simulate());
}
