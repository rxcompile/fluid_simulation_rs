pub fn construct_default<T: Default, const S: usize>() -> [T; S] {
    let mut t = std::mem::MaybeUninit::<T>::uninit_array::<S>();
    t.iter_mut().for_each(|i| {
        i.write(Default::default());
    });
    unsafe { std::mem::transmute_copy(&t) }
}

pub fn construct_from<T, I: Iterator, const S: usize>(iter: I) -> [T; S]
where
    I: Iterator<Item = T>,
{
    let mut buffer = std::mem::MaybeUninit::<T>::uninit_array::<S>();
    let mut x = iter;
    for i in buffer.iter_mut() {
        match x.next() {
            Some(v) => {
                i.write(v);
            }
            _ => panic!("No data to read from while constructing array!"),
        }
    }
    unsafe { std::mem::transmute_copy(&buffer) }
}
