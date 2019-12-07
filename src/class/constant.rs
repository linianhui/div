// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4

use crate::class::*;

#[derive(Debug)]
pub enum Constant {
    Utf8(ConstantUtf8),
    Integer(ConstantInteger),
    Float(ConstantFloat),
    Long(ConstantLong),
    Double(ConstantDouble),
    Class(ConstantClass),
    String(ConstantString),
    FieldRef(ConstantFieldRef),
    MethodRef(ConstantMethodRef),
    InterfaceMethodRef(ConstantInterfaceMethodRef),
    NameAndType(ConstantNameAndType),
    MethodHandle(ConstantMethodHandle),
    MethodType(ConstantMethodType),
    Dynamic(ConstantDynamic),
    InvokeDynamic(ConstantInvokeDynamic),
    Module(ConstantModule),
    Package(ConstantPackage),
}

impl Constant {
    pub fn new(reader: &mut U8Reader, constant_tag: ConstantTag) -> Constant {
        match constant_tag {
            ConstantTag::Utf8 => Constant::Utf8(ConstantUtf8::new(reader)),
            ConstantTag::Integer => Constant::Integer(ConstantInteger::new(reader)),
            ConstantTag::Float => Constant::Float(ConstantFloat::new(reader)),
            ConstantTag::Long => Constant::Long(ConstantLong::new(reader)),
            ConstantTag::Double => Constant::Double(ConstantDouble::new(reader)),
            ConstantTag::Class => Constant::Class(ConstantClass::new(reader)),
            ConstantTag::String => Constant::String(ConstantString::new(reader)),
            ConstantTag::FieldRef => Constant::FieldRef(ConstantFieldRef::new(reader)),
            ConstantTag::MethodRef => Constant::MethodRef(ConstantMethodRef::new(reader)),
            ConstantTag::InterfaceMethodRef => {
                Constant::InterfaceMethodRef(ConstantInterfaceMethodRef::new(reader))
            }
            ConstantTag::NameAndType => Constant::NameAndType(ConstantNameAndType::new(reader)),
            ConstantTag::MethodHandle => Constant::MethodHandle(ConstantMethodHandle::new(reader)),
            ConstantTag::MethodType => Constant::MethodType(ConstantMethodType::new(reader)),
            ConstantTag::Dynamic => Constant::Dynamic(ConstantDynamic::new(reader)),
            ConstantTag::InvokeDynamic => {
                Constant::InvokeDynamic(ConstantInvokeDynamic::new(reader))
            }
            ConstantTag::Module => Constant::Module(ConstantModule::new(reader)),
            ConstantTag::Package => Constant::Package(ConstantPackage::new(reader)),
        }
    }

    pub fn vec(reader: &mut U8Reader, constant_pool_count: usize) -> Vec<Constant> {
        let mut pool: Vec<Constant> = Vec::with_capacity(constant_pool_count);
        unsafe {
            pool.set_len(constant_pool_count);
        }
        let mut i = 1usize;
        while i < constant_pool_count {
            let constant_tag_raw = reader.read_u8();
            let constant_tag = ConstantTag::from(constant_tag_raw);
            pool[i] = Constant::new(reader, constant_tag);
            if ConstantTag::Long == constant_tag || ConstantTag::Double == constant_tag {
                i = i + 1;
            }
            i = i + 1;
        }
        return pool;
    }
}
