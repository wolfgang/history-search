use std::io::{Error, Write};
use std::str::from_utf8;

pub struct StdoutSpy {
    pub written_buf: Vec<u8>,
}

impl StdoutSpy {
    pub fn new() -> Self {
        Self { written_buf: Vec::with_capacity(256) }
    }

    pub fn clear(&mut self) {
        self.written_buf.clear();
    }

    pub fn assert(&self, expected: String) {
        assert_eq!(self.written_buf_as_str(), expected);
    }

    pub fn assert_contains<T>(&self, expected: T) where T: Into<String> {
        let expected_string = expected.into();
        let contains = self.written_buf_as_str().contains(&expected_string);
        if !contains {
            println!("String expected to contain {:?}: \n{:?}", expected_string, self.written_buf_as_str());
        }

        assert!(self.written_buf_as_str().contains(&expected_string));
    }

    pub fn written_buf_as_str(&self) -> &str {
        from_utf8(self.written_buf.as_slice()).unwrap()
    }
}

impl Write for StdoutSpy {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let mut new_vec = Vec::from(buf);
        self.written_buf.append(&mut new_vec);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
