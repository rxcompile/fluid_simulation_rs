use fluid_simulation::{iterator, Coords, Domain, Sized3D};

#[test]
fn diffusion_stability() {
    let mut domain: Domain<1, 3, 3, 3> = Default::default();
    domain.prop.step_delta_time = 0.1;
    domain.prop.pressure_acceleration = Some(1.);
    domain.prop.vorticity = None;
    domain.prop.velocity_decay = None;
    assert_eq!(domain.velocity(&Coords(0, 0, 0)), (0.0, 0.0, 0.0));
    domain.set_pressure(&Coords(0, 0, 0), &[32.0]);
    assert_eq!(domain.pressure(&Coords(0, 0, 0))[0], 0.0);
    for n in 0..10000 {
        domain.simulate();
        let sum = iterator::iterate(domain.size()).fold(0.0f32, |a, c| a + domain.pressure(&c)[0]);
        assert!((sum - 32.0).abs() < 0.0001, "iter = {} sum = {}", n, sum);
    }
}
