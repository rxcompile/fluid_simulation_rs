use crate::Coords;

pub fn iterate(size: Coords) -> impl std::iter::Iterator<Item = Coords> {
    iterate_range(Default::default(), size)
}

pub fn iterate_range(first: Coords, size: Coords) -> impl std::iter::Iterator<Item = Coords> {
    let mut iter = first;
    let mut start = true;
    std::iter::from_fn(move || {
        if start {
            if iter.0 >= size.0 || iter.1 >= size.1 || iter.2 >= size.2 {
                return None;
            }
            start = false;
            return Some(iter);
        }
        iter.0 += 1;
        if iter.0 >= size.0 {
            iter.0 = first.0;
            iter.1 += 1;
            if iter.1 >= size.1 {
                iter.1 = first.1;
                iter.2 += 1;
                if iter.2 >= size.2 {
                    return None;
                }
            }
        }
        Some(iter)
    })
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
