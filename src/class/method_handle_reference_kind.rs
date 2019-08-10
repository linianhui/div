// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-5.html#jvms-5.4.3.5

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum MethodHandleReferenceKind {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}
