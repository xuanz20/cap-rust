use super::{utils::align_up, CapPtr};

pub struct Untyepd {
    start: usize,
    length: usize,
    watermark: usize,
    first_child: Option<CapPtr>,
}

impl Untyepd {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            length: end - start,
            watermark: start,
            first_child: None
        }
    }

    fn get_start(&self) -> usize {
        self.start.into()
    }

    fn get_end(&self) -> usize {
        self.get_start() + self.length
    }

    pub fn allocate(&mut self, length: usize, alignment: usize) -> usize {
        let addr: usize = align_up(self.watermark, alignment).into();
        unsafe { assert!(addr + length <= self.get_end()); }
        self.watermark = (addr + length).into();
        addr.into()
    }

    pub fn derive<F>(&mut self, length: usize, alignment: usize, f: F)
    where
        F: FnOnce(usize, Option<CapPtr>) -> Option<CapPtr>
    {
        let addr = self.allocate(length, alignment);
        self.first_child = f(addr, self.first_child.take());
    }
}