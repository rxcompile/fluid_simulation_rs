use fluid_simulation::{Coords, Domain};

#[test]
fn stability() {
    let mut domain: Domain<1, 3, 3, 3> = Default::default();
    assert_eq!(domain.velocity(Coords(0, 0, 0)), (0.0, 0.0, 0.0));
    domain.set_pressure(Coords(0, 0, 0), &[32.0]);
    assert_eq!(domain.pressure(Coords(0,0,0))[0], 0.0);
    // let x = &domain.data.pressure[0].read().0;
    // assert_eq!(x, vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    domain.simulate();
    // let x = &domain.data.pressure[0].read().0;
    // assert_eq!(x, vec![32.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
}
