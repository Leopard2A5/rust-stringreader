use std::io::Read;
use std::io::Result;
use std::slice::Iter;

pub struct StringReader<'a> {
    iter: Iter<'a, u8>,
}

impl<'a> StringReader<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            iter: data.as_bytes().iter(),
        }
    }
}

impl<'a> Read for StringReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        for i in 0..buf.len() {
            if let Some(x) = self.iter.next() {
                buf[i] = *x;
            } else {
                return Ok(i);
            }
        }
        Ok(buf.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test() {
        let data = "abc\ndef";
        let mut reader = BufReader::new(StringReader::new(data));
        let mut buffer = String::new();

        buffer.clear();
        reader.read_line(&mut buffer).unwrap();
        assert_eq!("abc\n", buffer);

        buffer.clear();
        reader.read_line(&mut buffer).unwrap();
        assert_eq!("def", buffer);
    }
}
