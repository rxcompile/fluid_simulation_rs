pub fn construct_default<T: Default, const S: usize>() -> [T; S] {
    let mut t = std::mem::MaybeUninit::<T>::uninit_array::<S>();
    t.iter_mut().for_each(|i| {
        i.write(Default::default());
    });
    unsafe { std::mem::transmute_copy(&t) }
}

pub fn construct_from<T, I: Iterator, const S: usize>(iter: I) -> [T; S]
where
    I: IntoIterator<Item = T>,
{
    let mut buffer = std::mem::MaybeUninit::uninit_array::<S>();
    for (i, val) in buffer.iter_mut().zip(iter) {
        i.write(val);
    }
    unsafe { std::mem::MaybeUninit::array_assume_init(buffer) }
}
