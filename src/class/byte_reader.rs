#[derive(Debug)]
pub struct ByteReader {
    pub data: Box<[u8]>,
    pub offset: usize,
}

impl ByteReader {
    pub fn from(bytes: Box<[u8]>) -> ByteReader {
        ByteReader {
            data: bytes,
            offset: 0,
        }
    }

    pub fn read_u16(&mut self) -> u16 {
        let begin = self.offset;
        self.offset += 2;
        u16::from_be_bytes([self.data[begin], self.data[begin + 1]])
    }

    pub fn read_u32(&mut self) -> u32 {
        let begin = self.offset;
        self.offset += 4;
        u32::from_be_bytes([
            self.data[begin],
            self.data[begin + 1],
            self.data[begin + 2],
            self.data[begin + 3],
        ])
    }
}
