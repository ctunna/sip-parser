const UTF8_LEADING_BYTE_MASKS: [u8; 4] = [0xFF, 0x1F, 0x0F, 0x07];

pub struct Decoder {
    ch: u32,
    rem: usize,
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder { ch: 0, rem: 0 }
    }

    pub fn get_chars(&mut self, bytes: &[u8], chars: &mut [char]) -> usize {
        let mut index = 0;
        for byte in bytes {
            if index + 1 > chars.len() {
                break;
            }
            if self.rem > 0 {
                self.ch <<= 6;
                self.ch |= (byte & 0x3F) as u32;
                self.rem -= 1;
            } else {
                self.rem = Decoder::char_size(*byte) - 1;
                self.ch = (byte & UTF8_LEADING_BYTE_MASKS[self.rem]) as u32;
            }
            if self.rem == 0 {
                match char::from_u32(self.ch) {
                    Some(c) => chars[index] = c,
                    None => chars[index] = std::char::REPLACEMENT_CHARACTER,
                }
                index += 1;
            }
        }
        index
    }

    fn char_size(byte: u8) -> usize {
        let mask = 0b11110000;
        match byte & mask {
            0b11110000 => 4,
            0b11100000 => 3,
            0b11000000 => 2,
            _ => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_str("");
    }

    #[test]
    fn single_char_single_byte() {
        assert_str("A");
    }

    #[test]
    fn single_char_double_bytes() {
        assert_str("©");
    }

    #[test]
    fn single_char_three_bytes() {
        assert_str("€");
    }

    #[test]
    fn single_char_four_bytes() {
        assert_str("𐍈");
    }

    #[test]
    fn multiple_chars() {
        assert_str("H€ll𐍈\n world!");
    }

    #[test]
    fn partial_decode() {
        let mut decoder = Decoder::new();
        let mut chars = ['\0'; 1024];
        let c1 = decoder.get_chars(&[0xF0, 0x90, 0x8D], &mut chars);
        assert_eq!(c1, 0);
        let c2 = decoder.get_chars(&[0x88], &mut chars);
        assert_eq!(c2, 1);
        assert_eq!(chars[0], '𐍈');
    }

    fn assert_bytes(bytes: &[u8], expected: &str) {
        let mut decoder = Decoder::new();
        let mut chars = ['\0'; 1024];
        let count = decoder.get_chars(bytes, &mut chars);
        assert_eq!(count, expected.chars().count());
        for (i, c) in expected.chars().enumerate() {
            assert_eq!(c, chars[i]);
        }
    }

    fn assert_str(str: &str) {
        assert_bytes(str.as_bytes(), str);
    }
}
