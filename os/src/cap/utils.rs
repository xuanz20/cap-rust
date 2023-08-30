#[inline]
pub fn align_up(addr: usize, alignment: usize) -> usize {
    let mut raw: usize = addr.into();
    raw = raw + alignment - 1;
    let aligned: usize = raw - (raw % alignment);
    aligned.into()
}

#[inline]
pub fn align_down(addr: usize, alignment: usize) -> usize {
    let raw: usize = addr.into();
    let aligned: usize = raw - (raw % alignment);
    aligned.into()
}
