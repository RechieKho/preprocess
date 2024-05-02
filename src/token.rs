#[derive(Clone, Debug)]
pub struct Token<'text> {
    pub data: String,
    pub line: &'text str,
    pub row: usize,
    pub column: usize,
}
