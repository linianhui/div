#[derive(Debug)]
pub struct ClassFile {
    pub magic: u32,
}

impl ClassFile {
    pub fn from() -> ClassFile {
        ClassFile {
            magic: 0xCA_FE_BA_BE,
        }
    }
}
