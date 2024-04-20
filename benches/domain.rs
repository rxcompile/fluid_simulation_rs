#![feature(test)]
extern crate test;

use fluid_simulation::{iterator, Domain, Sized3D};
use test::Bencher;

#[bench]
fn bench_simulation_step_50(b: &mut Bencher) {
    let mut domain: Domain<1, 50, 50, 50> = Default::default();
    //b.iter(|| block_on(domain.simulate()));
}
#[bench]
fn bench_simulation_iterate_50(b: &mut Bencher) {
    let domain: Domain<1, 50, 50, 50> = Default::default();
    b.iter(|| {
        iterator::iterate(domain.size())
            .map(|c| domain.pressure(&c).iter().sum::<f32>())
            .sum::<f32>()
    });
}

#[bench]
fn bench_simulation_iterate_vel_50(b: &mut Bencher) {
    let domain: Domain<1, 50, 50, 50> = Default::default();
    b.iter(|| {
        iterator::iterate(domain.size())
            .map(|c| domain.velocity(&c).0)
            .sum::<f32>()
    });
}

#[bench]
fn bench_simulation_step_10(b: &mut Bencher) {
    let mut domain: Domain<1, 10, 10, 10> = Default::default();
    //b.iter(|| block_on(domain.simulate()));
}

#[bench]
fn bench_simulation_iterate_10(b: &mut Bencher) {
    let domain: Domain<1, 10, 10, 10> = Default::default();
    b.iter(|| {
        iterator::iterate(domain.size())
            .map(|c| domain.pressure(&c).iter().sum::<f32>())
            .sum::<f32>()
    });
}

#[bench]
fn bench_simulation_step_256(b: &mut Bencher) {
    let mut domain: Domain<1, 256, 128, 256> = Default::default();
    b.iter(|| domain.simulate());
}
