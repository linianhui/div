// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.7

pub struct ModifiedUtf8 {}

impl ModifiedUtf8 {
    pub fn to_string(modified_utf8_bytes: &[u8]) -> String {
        let end_index = modified_utf8_bytes.len() - 1;
        let mut u16s: Vec<u16> = Vec::with_capacity(end_index + 1);
        for mut i in 0..end_index {
            let x = modified_utf8_bytes[i] as u16;
            // 0_xxxxxx
            if x >> 7 == 0b_0000000_0 {
                u16s.push(x);
                continue;
            }

            if i + 1 > end_index {
                continue;
            }
            let y = modified_utf8_bytes[i + 1] as u16;
            // 110_xxxxx && 10_xxxxxx
            if x >> 5 == 0b_00000_110 && y >> 6 == 0b_000000_10 {
                u16s.push(((x & 0b_000_11111) << 6) + (y & 0b_00_111111));
                i += 1;
                continue;
            }

            if i + 2 > end_index {
                continue;
            }
            let z = modified_utf8_bytes[i + 2] as u16;
            // 1110_xxxx && 10_xxxxxx && 10_xxxxxx
            if x >> 4 == 0b_0000_1110 && y >> 6 == 0b_000000_10 && z >> 6 == 0b_000000_10 {
                u16s.push(
                    ((x & 0b_0000_1111) << 12) + ((y & 0b_00_111111) << 6) + (z & 0b_00_111111),
                );
                i += 2;
                continue;
            }
        }
        return String::from_utf16_lossy(u16s.as_slice());
    }
}
