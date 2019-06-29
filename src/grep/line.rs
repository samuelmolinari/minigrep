#[derive(Copy, Clone)]
pub struct Line<'a> {
    pub position: usize,
    pub value: &'a String
}
