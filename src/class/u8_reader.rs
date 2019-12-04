#[derive(Debug)]
pub struct U8Reader {
    pub data: Box<[u8]>,
    pub position: usize,
}

impl U8Reader {
    pub fn new(bytes: &[u8]) -> U8Reader {
        U8Reader {
            data: Box::from(bytes),
            position: 0,
        }
    }

    pub fn read_f32_and_update_position(&mut self) -> f32 {
        f32::from_bits(self.read_u32_and_update_position())
    }

    pub fn read_f64_and_update_position(&mut self) -> f64 {
        f64::from_bits(self.read_u64_and_update_position())
    }

    pub fn read_i32_and_update_position(&mut self) -> i32 {
        self.read_u32_and_update_position() as i32
    }

    pub fn read_i64_and_update_position(&mut self) -> i64 {
        self.read_u64_and_update_position() as i64
    }

    pub fn read_u8_and_update_position(&mut self) -> u8 {
        let begin = self.position;
        self.position += 1;
        self.data[begin]
    }

    pub fn read_u16_and_update_position(&mut self) -> u16 {
        let begin = self.position;
        self.position += 2;
        u16::from_be_bytes([self.data[begin], self.data[begin + 1]])
    }

    pub fn read_u16_vec_and_update_position(&mut self, count: usize) -> Vec<u16> {
        let mut u16_vec: Vec<u16> = Vec::with_capacity(count);
        for _i in 0..count {
            let u16_value = self.read_u16_and_update_position();
            u16_vec.push(u16_value);
        }

        return u16_vec;
    }

    pub fn read_u32_and_update_position(&mut self) -> u32 {
        let begin = self.position;
        self.position += 4;
        u32::from_be_bytes([
            self.data[begin],
            self.data[begin + 1],
            self.data[begin + 2],
            self.data[begin + 3],
        ])
    }

    pub fn read_u64_and_update_position(&mut self) -> u64 {
        let begin = self.position;
        self.position += 8;
        u64::from_be_bytes([
            self.data[begin],
            self.data[begin + 1],
            self.data[begin + 2],
            self.data[begin + 3],
            self.data[begin + 4],
            self.data[begin + 5],
            self.data[begin + 6],
            self.data[begin + 7],
        ])
    }

    pub fn read_bytes_and_update_position(&mut self, length: usize) -> &[u8] {
        let begin = self.position;
        let end = begin + length;
        self.position = end;
        &self.data[begin..end]
    }
}
