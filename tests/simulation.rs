use fluid_simulation::{Coords, DomainDefault, DomainProperties, FlowFlags};

#[test]
fn interface() {
    let prop = DomainProperties {
        size: Coords(3, 3, 3),
        ..Default::default()
    };
    let mut domain = DomainDefault::new(prop);
    assert_eq!(domain.velocity(Coords(0, 0, 0)), (0.0, 0.0, 0.0));
    domain.set_velocity(Coords(0, 0, 0), (1.0, 1.0, 1.0));
    assert_eq!(domain.blocked(Coords(0, 0, 0)), FlowFlags::empty());
    domain.set_block(Coords(0, 0, 0), FlowFlags::X_BACK);

    domain.set_pressure(Coords(0, 0, 0), 32.0);
    assert_eq!(domain.pressure(Coords(0, 0, 0)), 0.0);
    domain.simulate();
    assert_eq!(domain.pressure(Coords(0, 0, 0)), 32.000004);
}
