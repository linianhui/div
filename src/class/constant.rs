// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4

use crate::class::ConstantClass;
use crate::class::ConstantDouble;
use crate::class::ConstantDynamic;
use crate::class::ConstantFieldRef;
use crate::class::ConstantFloat;
use crate::class::ConstantInteger;
use crate::class::ConstantInterfaceMethodRef;
use crate::class::ConstantInvokeDynamic;
use crate::class::ConstantLong;
use crate::class::ConstantMethodHandle;
use crate::class::ConstantMethodRef;
use crate::class::ConstantMethodType;
use crate::class::ConstantModule;
use crate::class::ConstantNameAndType;
use crate::class::ConstantPackage;
use crate::class::ConstantString;
use crate::class::ConstantTag;
use crate::class::ConstantUtf8;
use crate::class::U8Reader;

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
    pub fn new(constant_tag: ConstantTag, reader: &mut U8Reader) -> Constant {
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

    pub fn vec(constant_pool_count: usize, reader: &mut U8Reader) -> Vec<Constant> {
        let mut pool: Vec<Constant> = Vec::with_capacity(constant_pool_count);
        unsafe {
            pool.set_len(constant_pool_count);
        }
        let mut i = 1usize;
        while i < constant_pool_count {
            let constant_tag = ConstantTag::from(reader.read_u8());
            pool[i] = Constant::new(constant_tag, reader);
            if ConstantTag::Long == constant_tag || ConstantTag::Double == constant_tag {
                i = i + 1;
            }
            i = i + 1;
        }
        return pool;
    }
}
