pub struct Source {
    // Acts as immutable string since Source should never be modified.
    source: String,
    source_len: u32,
}
impl Source {
    pub fn new(source: String) -> Source {
        let source_len = source.len() as u32;
        Source { source, source_len }
    }

    pub fn get_char(&self, position: u32) -> Option<u8> {
        if position < self.source_len {
            let src_chr = self.source.as_bytes()[position as usize];
            Some(src_chr)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::source::Source;
    #[test]
    fn test_source() {
        let string = "aaa".to_string();
        let source = Source::new(string);
        if let Some(chr) = source.get_char(0) {
            assert_eq!(chr, "a".to_string().as_bytes()[0])
        }
        if let Some(chr) = source.get_char(1) {
            assert_eq!(chr, "a".to_string().as_bytes()[0])
        }
        if let Some(chr) = source.get_char(2) {
            assert_eq!(chr, "a".to_string().as_bytes()[0])
        }
        let s = source.get_char(3);
        assert_eq!(s, None);
    }
}