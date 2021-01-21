#[derive(Debug)]
pub enum Mode {
    Sequential,
    SingleCycle,
    Random,
}

impl Default for Mode {
    #[inline]
    fn default() -> Self {
        Mode::Sequential
    }
}
