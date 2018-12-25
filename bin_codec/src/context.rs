#[derive(Default)]
pub struct Context {
    bit_size: Option<usize>,
    count: Option<usize>,
    is_some: Option<bool>,
    has_next: Option<bool>,
}

impl Context {
    pub fn bit_size(&self) -> Option<usize> {
        self.bit_size.clone()
    }
    pub fn count(&self) -> Option<usize> {
        self.count.clone()
    }
    pub fn is_some(&self) -> Option<bool> {
        self.is_some.clone()
    }
    pub fn has_next(&self) -> Option<bool> {
        self.has_next.clone()
    }
    pub fn set_bit_size(&mut self, v: Option<usize>) {
        self.bit_size = v;
    }
    pub fn set_count(&mut self, v: Option<usize>) {
        self.count = v;
    }
    pub fn set_is_some(&mut self, v: Option<bool>) {
        self.is_some = v;
    }
    pub fn set_has_next(&mut self, v: Option<bool>) {
        self.has_next = v;
    }
}