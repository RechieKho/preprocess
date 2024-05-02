#[derive(Clone, Debug)]
pub struct Token<'text> {
    pub data: String,
    pub line: &'text str,
    pub row: usize,
    pub column: usize,
}

impl<'text> PartialEq for Token<'text> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<'text> PartialEq<char> for Token<'text> {
    fn eq(&self, other: &char) -> bool {
        let mut other_str_buffer = [0u8; 4];
        let other_str = other.encode_utf8(&mut other_str_buffer);

        self.data.as_str() == other_str
    }
}
