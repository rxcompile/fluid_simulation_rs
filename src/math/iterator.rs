use crate::Coords;

pub fn iterate(size: Coords) -> impl std::iter::Iterator<Item = Coords> {
    iterate_range(Default::default(), size)
}

pub fn iterate_range(first: Coords, size: Coords) -> impl std::iter::Iterator<Item = Coords> {
    (first.2..size.2)
        .flat_map(move |z| (first.1..size.1).zip(std::iter::from_fn(move || Some(z))))
        .flat_map(move |yz| (first.0..size.0).zip(std::iter::from_fn(move || Some(yz))))
        .map(|(x, (y, z))| Coords(x, y, z))
}

#[test]
fn iterator_test() {
    let vec1: Vec<_> = iterate((1, 1, 1).into()).collect();
    assert_eq!(vec1, vec![(0, 0, 0).into()]);
    let vec2: Vec<_> = iterate((1, 1, 2).into()).collect();
    assert_eq!(vec2, vec![(0, 0, 0).into(), (0, 0, 1).into()]);
    let vec3: Vec<_> = iterate((1, 2, 1).into()).collect();
    assert_eq!(vec3, vec![(0, 0, 0).into(), (0, 1, 0).into()]);
    let vec4: Vec<_> = iterate((2, 1, 1).into()).collect();
    assert_eq!(vec4, vec![(0, 0, 0).into(), (1, 0, 0).into()]);
    let vec5: Vec<_> = iterate((3, 3, 3).into()).collect();
    let vec5_t = vec![
        (0, 0, 0).into(),
        (1, 0, 0).into(),
        (2, 0, 0).into(),
        (0, 1, 0).into(),
        (1, 1, 0).into(),
        (2, 1, 0).into(),
        (0, 2, 0).into(),
        (1, 2, 0).into(),
        (2, 2, 0).into(),
        (0, 0, 1).into(),
        (1, 0, 1).into(),
        (2, 0, 1).into(),
        (0, 1, 1).into(),
        (1, 1, 1).into(),
        (2, 1, 1).into(),
        (0, 2, 1).into(),
        (1, 2, 1).into(),
        (2, 2, 1).into(),
        (0, 0, 2).into(),
        (1, 0, 2).into(),
        (2, 0, 2).into(),
        (0, 1, 2).into(),
        (1, 1, 2).into(),
        (2, 1, 2).into(),
        (0, 2, 2).into(),
        (1, 2, 2).into(),
        (2, 2, 2).into(),
    ];
    assert_eq!(vec5, vec5_t);
}
