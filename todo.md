# ClassFile

code: [/src/class](/src/class)

- [x] magic
- [x] minor_version
- [x] major_version
- [x] constant_pool_count
- [ ] constant_pool[constant_pool_count-1]
    - [ ] CONSTANT_Utf8_info
    - [x] CONSTANT_Integer_info
    - [x] CONSTANT_Float_info
    - [x] CONSTANT_Long_info
    - [x] CONSTANT_Double_info
    - [x] CONSTANT_Class_info
    - [x] CONSTANT_String_info
    - [x] CONSTANT_Fieldref_info
    - [x] CONSTANT_Methodref_info
    - [x] CONSTANT_InterfaceMethodref_info
    - [x] CONSTANT_NameAndType_info
    - [x] CONSTANT_MethodHandle_info
    - [x] CONSTANT_MethodType_info
    - [x] CONSTANT_Dynamic_info
    - [x] CONSTANT_InvokeDynamic_info
    - [x] CONSTANT_Module_info
    - [x] CONSTANT_Package_info
- [ ] access_flags
- [ ] this_class
- [ ] super_class
- [ ] interfaces_count
- [ ] interfaces[interfaces_count]
- [ ] fields_count
- [ ] fields[fields_count]
    - [ ] field_info 
- [ ] methods_count
- [ ] methods[methods_count]
    - [ ] method_info 
- [ ] attributes_count
- [ ] attributes[attributes_count]
    - [ ] attribute_info
    - [ ] ConstantValue_attribute
    - [ ] Code_attribute
    - [ ] Exceptions_attribute
    - [ ] SourceFile_attribute
    - [ ] LineNumberTable_attribute
    - [ ] LocalVariableTable_attribute
    - [ ] InnerClasses_attribute
    - [ ] Synthetic_attribute
    - [ ] Deprecated_attribute
    - [ ] EnclosingMethod_attribute
    - [ ] Signature_attribute
    - [ ] SourceDebugExtension_attribute
    - [ ] LocalVariableTypeTable_attribute
    - [ ] RuntimeVisibleAnnotations_attribute
    - [ ] RuntimeInvisibleAnnotations_attribute
    - [ ] RuntimeVisibleParameterAnnotations_attribute
    - [ ] RuntimeInvisibleParameterAnnotations_attribute
    - [ ] AnnotationDefault_attribute
    - [ ] StackMapTable_attribute
    - [ ] BootstrapMethods_attribute
    - [ ] RuntimeVisibleTypeAnnotations_attribute
    - [ ] RuntimeInvisibleTypeAnnotations_attribute
    - [ ] MethodParameters_attribute
    - [ ] Module_attribute
    - [ ] ModulePackages_attribute
    - [ ] ModuleMainClass_attribute
    - [ ] NestHost_attribute
    - [ ] NestMembers_attribute


# Reference

https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html