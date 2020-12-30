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
    let vec: Vec<_> = iterate(Coords(3, 3, 3)).collect();
    let vec_t = vec![
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
    assert_eq!(vec, vec_t);
}
