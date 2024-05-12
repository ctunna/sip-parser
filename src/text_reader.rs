use super::decoder;
use std::io::{BufRead, BufReader};

const BUFFER_CAPACITY: usize = 8192;

pub struct TextReader<'a> {
    reader: BufReader<&'a [u8]>,
    decoder: decoder::Decoder,
    buffer: [char; BUFFER_CAPACITY],
    buffer_size: usize,
    position: usize,
}

impl<'a> TextReader<'a> {
    pub fn new(reader: BufReader<&'a [u8]>) -> TextReader {
        TextReader {
            reader,
            decoder: decoder::Decoder::new(),
            buffer: ['\0'; BUFFER_CAPACITY],
            buffer_size: 0,
            position: 0,
        }
    }

    pub fn read(&mut self) -> i32 {
        if self.position >= self.buffer_size {
            self.buffer_size = match &self.reader.fill_buf() {
                Ok(bytes) => {
                    let result = self.decoder.get_chars(bytes, &mut self.buffer);
                    let length = bytes.len();
                    self.reader.consume(length);
                    result
                }
                Err(_) => 0,
            };
            self.position = 0;

            if self.buffer_size == 0 {
                return -1;
            }
        }
        let result = self.buffer[self.position] as i32;
        self.position += 1;
        result
    }

    pub fn peek(&mut self) -> i32 {
        let result = self.read();
        if result != -1 {
            self.position -= 1;
        }
        result
    }

    pub fn read_line(&mut self) -> String {
        let mut result = String::new();
        loop {
            let c = self.read();
            if c == -1 || c == '\n' as i32 {
                break;
            }
            if c == '\r' as i32 && self.peek() == '\n' as i32 {
                self.read();
                break;
            }
            result.push(std::char::from_u32(c as u32).unwrap());
        }
        result
    }

    pub fn read_to_end(&mut self) -> String {
        let mut result = String::new();
        loop {
            let c = self.read();
            if c == -1 {
                break;
            }
            result.push(std::char::from_u32(c as u32).unwrap());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_empty() {
        assert_str("");
    }

    #[test]
    fn read_single_char() {
        assert_str("A");
    }

    #[test]
    fn read_general_case() {
        assert_str("€hello");
    }

    #[test]
    fn peek_empty() {
        let bytes = "".to_string().into_bytes();
        let mut reader = TextReader::new(BufReader::new(&bytes));
        assert_eq!(reader.peek(), -1);
    }

    #[test]
    fn peek_single_char() {
        let bytes = "A".to_string().into_bytes();
        let mut reader = TextReader::new(BufReader::new(&bytes));
        assert_eq!(reader.peek(), 'A' as i32);
    }

    #[test]
    fn read_line_empty() {
        let bytes = "".to_string().into_bytes();
        let mut reader = TextReader::new(BufReader::new(&bytes));
        assert_eq!(reader.read_line(), "");
    }

    #[test]
    fn read_line_general_case() {
        let bytes = "€hello\nworld".to_string().into_bytes();
        let mut reader = TextReader::new(BufReader::new(&bytes));
        assert_eq!(reader.read_line(), "€hello");
        assert_eq!(reader.read_line(), "world");
    }

    fn assert_str(s: &str) {
        let bytes = s.to_string().into_bytes();
        let mut reader = TextReader::new(BufReader::new(&bytes));
        let mut result = String::new();
        loop {
            let c = reader.read();
            if c == -1 {
                break;
            }
            result.push(std::char::from_u32(c as u32).unwrap());
        }
        assert_eq!(result, s);
    }
}
