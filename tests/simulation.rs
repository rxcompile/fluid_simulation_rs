use core::fmt;

use fluid_simulation::{iterator, Coords, Domain, Sizeable3D};

#[test]
fn diffusion_stability() {
    let mut domain: Domain<1, 3, 3, 3> = Default::default();
    domain.prop.step_delta_time = 0.1;
    domain.prop.pressure_acceleration = Some(1.);
    domain.prop.vorticity = None;
    domain.prop.velocity_decay = None;
    assert_eq!(domain.velocity(Coords(0, 0, 0)), (0.0, 0.0, 0.0));
    const VALUE: f32 = 32.0;
    domain.set_pressure(Coords(0, 0, 0), &[VALUE]);
    assert_eq!(domain.pressure(Coords(0, 0, 0))[0], 0.0);
    for n in 0..10000 {
        domain.simulate();
        let sum = iterator::iterate(domain.size())
            .map(|c| domain.pressure(c).iter().sum::<f32>())
            .sum::<f32>();
        let diff = (sum - VALUE).abs();
        println!("iter = {} sum = {} diff = {}", n, sum, diff);
        assert!(diff < 0.0001, "iter = {} sum = {}", n, sum);
    }
}
