#[derive(Clone, Debug)]
pub struct WhyExc {
    pub line: usize,
    pub col: usize,
    pub message: String,
}
